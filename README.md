jpg2ascii

Convert images to ASCII art with a Rust library and CLI.

Features
- Read common image formats (JPG/PNG, etc.) via the `image` crate
- Aspect-aware scaling for monospaced fonts
- Configurable charset, invert, gamma, contrast, brightness, threshold
- Optional ANSI TrueColor output to terminals
- Parallel conversion for fast processing

Quick Start
- Build: `cargo build --release`
- Run: `cargo run -- <image> --width 80`

Examples
- Basic: `cargo run -- image2ascii/convert/testdata/husky_100x100.jpg --width 80`
- Force color: `cargo run -- image2ascii/convert/testdata/png_sample_image.png --width 80 --color`
- No color: `cargo run -- image2ascii/convert/testdata/png_sample_image.png --width 80 --no-color`
- STDIN input: `type input.jpg | cargo run -- - --width 80` (PowerShell)
- Animate GIF: `cargo run -- image2ascii/convert/testdata/pikaqiu.gif --animate --fps 12`

CLI Options (selected)
- `--width`, `--height`, `--scale` — control output size
- `--charset` — characters from light→dark (default: ` .:-=+*#%@`)
- `--invert` — invert light/dark mapping
- `--color` / `--no-color` — enable/disable ANSI color
- `--brightness`, `--gamma`, `--contrast`, `--threshold` — tone controls
- `--aspect` — character height/width ratio (default: 2.0)

Library Usage
Add to your project and use the API:

```
use jpg2ascii::{convert_path_to_ascii, Config};

fn main() -> anyhow::Result<()> {
    let cfg = Config { width: Some(80), ..Default::default() };
    let ascii = convert_path_to_ascii("./image.jpg", &cfg)?;
    println!("{}", ascii);
    Ok(())
}
```

Notes
- The `image2ascii/` directory contains a Go reference implementation and test images; it is not part of the Rust build.
- Use a monospaced font in your terminal for best alignment.
- Some older terminals may not support ANSI truecolor; use `--no-color` if colors look wrong.
