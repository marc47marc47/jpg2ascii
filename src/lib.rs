use image::{imageops::FilterType, DynamicImage, GenericImageView};
use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;
use std::io::Cursor;
use rayon::prelude::*;
use anyhow::Result;
use std::path::Path;

pub const DEFAULT_CHARSET: &str = " .:-=+*#%@"; // low -> high density

#[derive(Clone, Debug)]
pub struct Config {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub scale: Option<f32>,
    pub charset: String,
    pub invert: bool,
    pub color: bool,
    pub brightness: f32, // -1.0..1.0; 0.0 = none
    pub gamma: f32,      // 1.0 = no change
    pub contrast: f32,   // 1.0 = no change
    pub threshold: Option<u8>, // 0..=255, None = continuous
    pub aspect: f32,     // character height/width ratio, ~2.0 for many fonts
    pub filter: FilterType,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            scale: None,
            charset: DEFAULT_CHARSET.to_string(),
            invert: false,
            color: false,
            brightness: 0.0,
            gamma: 1.0,
            contrast: 1.0,
            threshold: None,
            aspect: 2.0,
            filter: FilterType::Triangle,
        }
    }
}

pub fn convert_path_to_ascii<P: AsRef<Path>>(path: P, cfg: &Config) -> Result<String> {
    let img = image::open(path)?;
    Ok(convert_image_to_ascii(&img, cfg))
}

pub fn convert_bytes_to_ascii(bytes: &[u8], cfg: &Config) -> Result<String> {
    let img = image::load_from_memory(bytes)?;
    Ok(convert_image_to_ascii(&img, cfg))
}

pub fn convert_gif_bytes_to_ascii_frames(bytes: &[u8], cfg: &Config) -> Result<Vec<String>> {
    let cursor = Cursor::new(bytes);
    let decoder = GifDecoder::new(cursor)?;
    let frames = decoder.into_frames();
    let mut out = Vec::new();
    for f in frames {
        let f = f?;
        let buf = f.into_buffer();
        let dynimg = DynamicImage::ImageRgba8(buf);
        out.push(convert_image_to_ascii(&dynimg, cfg));
    }
    Ok(out)
}

pub fn convert_gif_path_to_ascii_frames<P: AsRef<Path>>(path: P, cfg: &Config) -> Result<Vec<String>> {
    let bytes = std::fs::read(path)?;
    convert_gif_bytes_to_ascii_frames(&bytes, cfg)
}

pub fn convert_image_to_ascii(img: &DynamicImage, cfg: &Config) -> String {
    let lines = convert_image_to_ascii_lines(img, cfg);
    lines.join("\n")
}

pub fn convert_image_to_ascii_lines(img: &DynamicImage, cfg: &Config) -> Vec<String> {
    let (tw, th) = target_size(img.dimensions(), cfg);
    let resized = img.resize_exact(tw, th, cfg.filter).to_rgba8();

    (0..th)
        .into_par_iter()
        .map(|y| {
            let mut line = String::with_capacity(tw as usize);
            for x in 0..tw {
                let px = resized.get_pixel(x, y);
                let (r, g, b, a) = (px[0], px[1], px[2], px[3]);
                // blend alpha on black background
                let (r, g, b) = if a < 255 {
                    let af = (a as f32) / 255.0;
                    ((r as f32 * af) as u8, (g as f32 * af) as u8, (b as f32 * af) as u8)
                } else {
                    (r, g, b)
                };
                let mut lum = luminance(r, g, b);
                // brightness offset
                if cfg.brightness != 0.0 {
                    lum = (lum + cfg.brightness).clamp(0.0, 1.0);
                }
                if let Some(t) = cfg.threshold {
                    lum = if (lum * 255.0).round() as u8 >= t { 1.0 } else { 0.0 };
                } else {
                    if cfg.gamma != 1.0 && cfg.gamma > 0.0 {
                        lum = lum.powf(1.0 / cfg.gamma);
                    }
                    if cfg.contrast != 1.0 {
                        lum = (lum - 0.5) * cfg.contrast + 0.5;
                        lum = lum.clamp(0.0, 1.0);
                    }
                }
                let ch = map_luma_to_char(lum, &cfg.charset, cfg.invert);
                if cfg.color {
                    use std::fmt::Write as _;
                    let _ = write!(line, "\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, ch);
                } else {
                    line.push(ch);
                }
            }
            line
        })
        .collect()
}

fn target_size((w, h): (u32, u32), cfg: &Config) -> (u32, u32) {
    let mut tw: f32;
    let mut th: f32;
    let aspect = if cfg.aspect > 0.0 { cfg.aspect } else { 2.0 };

    match (cfg.width, cfg.height, cfg.scale) {
        (Some(w_set), Some(h_set), _) => {
            tw = w_set as f32;
            // apply aspect correction on height
            th = h_set as f32 / aspect;
        }
        (Some(w_set), None, s) => {
            tw = w_set as f32;
            th = (h as f32 / aspect) * (tw / w as f32);
            if let Some(s) = s { tw *= s; th *= s; }
        }
        (None, Some(h_set), s) => {
            th = h_set as f32 / aspect;
            tw = (w as f32) * (th * aspect / h as f32); // invert derivation
            if let Some(s) = s { tw *= s; th *= s; }
        }
        (None, None, Some(s)) => {
            tw = w as f32 * s;
            th = (h as f32 / aspect) * s;
        }
        (None, None, None) => {
            // sensible default width
            tw = 80.0;
            th = (h as f32 / aspect) * (tw / w as f32);
        }
    }

    if tw < 1.0 { tw = 1.0; }
    if th < 1.0 { th = 1.0; }
    (tw.round() as u32, th.round() as u32)
}

fn luminance(r: u8, g: u8, b: u8) -> f32 {
    // linearized approximation is omitted; keep simple sRGB weights
    (0.2126 * (r as f32) + 0.7152 * (g as f32) + 0.0722 * (b as f32)) / 255.0
}

fn map_luma_to_char(lum: f32, charset: &str, invert: bool) -> char {
    let mut chars: Vec<char> = charset.chars().collect();
    if chars.is_empty() {
        chars = DEFAULT_CHARSET.chars().collect();
    }
    let n = chars.len();
    let mut v = if invert { 1.0 - lum } else { lum };
    if v < 0.0 { v = 0.0; }
    if v > 1.0 { v = 1.0; }
    let idx = (v * (n as f32 - 1.0)).round() as usize;
    chars[idx]
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_luminance_basic() {
        assert!((luminance(0, 0, 0) - 0.0).abs() < 1e-6);
        assert!((luminance(255, 255, 255) - 1.0).abs() < 1e-6);
        let gray = luminance(128, 128, 128);
        assert!(gray > 0.4 && gray < 0.6);
    }

    #[test]
    fn test_map_luma_to_char_invert() {
        let cs = " .#"; // 0 -> space, 1 -> '#'
        let c1 = map_luma_to_char(0.0, cs, false);
        let c2 = map_luma_to_char(1.0, cs, false);
        let c3 = map_luma_to_char(0.0, cs, true);
        assert_eq!(c1, ' ');
        assert_eq!(c2, '#');
        assert_eq!(c3, '#');
    }

    #[test]
    fn test_target_size_defaults() {
        let cfg = Config::default();
        let (w, h) = super::target_size((400, 200), &cfg);
        assert_eq!(w, 80);
        assert!(h > 0);
    }
}
