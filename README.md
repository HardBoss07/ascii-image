# ASCII Image

A Rust library to render images as ASCII art with optional ANSI color output. Supports resizing and smooth character ramps for better aesthetics.

## Features

* Convert images to ASCII art
* Optional 24-bit color (ANSI) output
* Configurable maximum width and height
* Simple, lightweight, no external dependencies besides `image` and `palette`

## Installation

Add to your `Cargo.toml`:

```bash
[dependencies]
cargo add ascii_image
```

Or clone the repo:

```bash
git clone https://github.com/yourusername/ascii-image.git
cd ascii-image
cargo build --release
```

## Usage as a library

```rust
use ascii_image::{render_path_to_ascii, Options};

fn main() -> anyhow::Result<()> {
    let opts = Options {
        max_width: 100,
        max_height: 50,
        colored: false,
    };

    let ascii = render_path_to_ascii("example.png", &opts)?;
    println!("{}", ascii);

    Ok(())
}
```

## Usage via CLI Example

You can also use it without writing Rust code, using the included CLI example:

```rust
use ascii_image::{render_path_to_ascii, Options};
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: cargo run --example cli -- <image>");
        anyhow::bail!("no image provided");
    }

    let opts = Options {
        max_width: 120,
        max_height: 60,
        colored: true, // set to false for plain ASCII
    };

    let output = render_path_to_ascii(&args[1], &opts)?;
    print!("{output}");

    Ok(())
}
```

Run the example:

```bash
cargo run --example cli -- path/to/image.png
```

## License

MIT License © 2025 Matteo Bosshard