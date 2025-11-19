use image::{DynamicImage, GenericImageView, Pixel};
use palette::{Hsv, Srgb, FromColor};

pub struct Options {
    pub max_width: u32,
    pub max_height: u32,
    pub colored: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            max_width: 80,
            max_height: 50,
            colored: false,
        }
    }
}

pub fn render_path_to_ascii<P: AsRef<std::path::Path>>(path: P, opts: &Options) -> image::ImageResult<String> {
    let img = image::open(path)?;
    Ok(render_to_ascii(&img, opts))
}

pub fn render_to_ascii(img: &DynamicImage, opts: &Options) -> String {
    let rgb = img.to_rgb8();
    let (iw, ih) = rgb.dimensions();

    let w = opts.max_width.min(iw);
    let h = opts.max_height.min(ih);

    let mut out = String::new();

    for cy in 0..h {
        let y = cy * ih / h;

        for cx in 0..w {
            let x = cx * iw / w;
            let p = rgb.get_pixel(x, y).0;

            let r = p[0];
            let g = p[1];
            let b = p[2];

            // RGB -> HSV (corrected: removed into_linear())
            let rgbf = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
            let hsv: Hsv = Hsv::from_color(rgbf);

            let ch = brightness_to_char(brightness(r, g, b));

            if opts.colored {
                // 24-bit truecolor ANSI
                let (rr, gg, bb) = (r, g, b);
                out.push_str(&format!("\x1b[38;2;{rr};{gg};{bb}m{ch}\x1b[0m"));
            } else {
                out.push(ch);
            }
        }

        out.push('\n');
    }

    out
}

fn brightness(r: u8, g: u8, b: u8) -> u8 {
    ((0.2126 * r as f32)
        + (0.7152 * g as f32)
        + (0.0722 * b as f32)) as u8
}

const RAMP: &[u8] = b" .`:-=+*#%&@BM8W";

fn brightness_to_char(br: u8) -> char {
    let index = (br as usize * (RAMP.len() - 1)) / 255;
    RAMP[index] as char
}
