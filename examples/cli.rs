use ascii_image::{render_path_to_ascii, Options};
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: cargo run --example cli -- <image>");
        anyhow::bail!("no image provided");
    }

    let opts = Options {
        max_width: 90,
        max_height: 90,
        colored: true, // set to false for plain ASCII
    };

    let output = render_path_to_ascii(&args[1], &opts)?;
    print!("{output}");

    Ok(())
}
