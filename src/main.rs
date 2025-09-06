use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use clap::{ArgAction, Parser};

use jpg2ascii::{convert_bytes_to_ascii, convert_gif_bytes_to_ascii_frames, convert_gif_path_to_ascii_frames, convert_path_to_ascii, Config, DEFAULT_CHARSET};

#[derive(Parser, Debug)]
#[command(name = "jpg2ascii", version, about = "Convert images to ASCII art")] 
struct Cli {
    /// Input image path or '-' for STDIN
    input: PathBuf,

    /// Output file path (default: stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Output width (characters)
    #[arg(short = 'W', long)]
    width: Option<u32>,

    /// Output height (characters before aspect correction)
    #[arg(short = 'H', long)]
    height: Option<u32>,

    /// Scale both width and height by factor
    #[arg(short, long)]
    scale: Option<f32>,

    /// Character set from light to dark
    #[arg(long, default_value = DEFAULT_CHARSET)]
    charset: String,

    /// Invert light/dark mapping
    #[arg(long, action = ArgAction::SetTrue)]
    invert: bool,

    /// Enable ANSI truecolor output
    #[arg(long, action = ArgAction::SetTrue)]
    color: bool,

    /// Force disable color even if --color is set
    #[arg(long, action = ArgAction::SetTrue)]
    no_color: bool,

    /// Gamma correction (1.0 = none)
    #[arg(long, default_value_t = 1.0)]
    gamma: f32,

    /// Contrast factor (1.0 = none)
    #[arg(long, default_value_t = 1.0)]
    contrast: f32,

    /// Brightness offset (-1.0..1.0)
    #[arg(long, default_value_t = 0.0)]
    brightness: f32,

    /// Animate GIF frames to stdout
    #[arg(long, action = ArgAction::SetTrue)]
    animate: bool,

    /// Target FPS when animating GIFs (default: 12)
    #[arg(long, default_value_t = 12.0)]
    fps: f32,

    /// Threshold 0..255 for B/W mapping
    #[arg(long)]
    threshold: Option<u8>,

    /// Character aspect ratio (height/width), typical ~2.0
    #[arg(long, default_value_t = 2.0)]
    aspect: f32,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let use_color = args.color && !args.no_color && ansi_supported();
    let cfg = Config {
        width: args.width,
        height: args.height,
        scale: args.scale,
        charset: args.charset,
        invert: args.invert,
        color: use_color,
        brightness: args.brightness,
        gamma: args.gamma,
        contrast: args.contrast,
        threshold: args.threshold,
        aspect: args.aspect,
        ..Default::default()
    };

    if args.animate {
        // Render frames (GIF) and animate to stdout with clear between frames
        let frames: Vec<String> = if args.input.as_os_str() == "-" {
            use std::io::Read;
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf)?;
            match convert_gif_bytes_to_ascii_frames(&buf, &cfg) {
                Ok(f) => f,
                Err(_) => vec![convert_bytes_to_ascii(&buf, &cfg)?],
            }
        } else {
            let ext = args
                .input
                .extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_ascii_lowercase());
            if matches!(ext.as_deref(), Some("gif")) {
                convert_gif_path_to_ascii_frames(&args.input, &cfg)?
            } else {
                vec![convert_path_to_ascii(&args.input, &cfg)?]
            }
        };

        let delay = if args.fps > 0.0 {
            std::time::Duration::from_secs_f32(1.0 / args.fps)
        } else {
            std::time::Duration::from_millis(100)
        };
        let mut first = true;
        for frame in frames {
            if !first { print!("\x1b[2J\x1b[H"); }
            first = false;
            println!("{}", frame);
            std::thread::sleep(delay);
        }
        return Ok(());
    } else {
        // Single frame path
        let ascii = if args.input.as_os_str() == "-" {
            use std::io::Read;
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf)?;
            convert_bytes_to_ascii(&buf, &cfg)?
        } else {
            convert_path_to_ascii(&args.input, &cfg)?
        };

        match args.output {
            Some(path) => {
                fs::write(path, ascii)?;
            }
            None => {
                let stdout = io::stdout();
                let mut handle = stdout.lock();
                handle.write_all(ascii.as_bytes())?;
            }
        }
    }

    Ok(())
}

fn ansi_supported() -> bool {
    // Disable if NO_COLOR is set
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    // Only enable on TTY
    if !atty::is(atty::Stream::Stdout) {
        return false;
    }
    // On Windows, try enabling VT processing
    #[cfg(windows)]
    if enable_windows_ansi() {
        return true;
    }
    // Heuristic using TERM
    if let Ok(term) = std::env::var("TERM") {
        let t = term.to_ascii_lowercase();
        return t.contains("xterm") || t.contains("ansi") || t.contains("vt100") || t.contains("screen");
    }
    // On modern Windows terminals truecolor is typically supported; default to true
    cfg!(windows)
}

#[cfg(windows)]
fn enable_windows_ansi() -> bool {
    use windows_sys::Win32::System::Console::{GetConsoleMode, GetStdHandle, SetConsoleMode, ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_OUTPUT_HANDLE};
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle == 0 { return false; }
        let mut mode: u32 = 0;
        if GetConsoleMode(handle, &mut mode) == 0 { return false; }
        let new_mode = mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;
        SetConsoleMode(handle, new_mode) != 0
    }
}
