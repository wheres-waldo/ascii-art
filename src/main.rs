use colored::*;
use image::{imageops::FilterType::*, GenericImageView, Pixel};
use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};

const BRIGHTNESS_CHARS: [char; 65] = [
    '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '~', '+', '_', '-', '?', ']', '[', '}', '{',
    '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U',
    'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', '#',
    'M', 'W', '&', '8', '%', 'B', '@', '$',
];

#[derive(StructOpt)]
/// Converts and prints images to ascii
struct Cmd {
    #[structopt(parse(from_os_str))]
    /// Path to image
    image: PathBuf,
    #[structopt(long, short, default_value = "ascii", possible_values = &Output::variants(), case_insensitive = true)]
    /// Output type of image
    output: Output,
    #[structopt(long, short, default_value = "90")]
    /// Width of output
    width: u32,
    #[structopt(long, short)]
    /// Inverts image
    invert: bool,
    #[structopt(long, short, default_value = "average", possible_values = &Brightness::variants(), case_insensitive = true)]
    brightness: Brightness,
}

arg_enum! {
    #[derive(Debug)]
    enum Output {
        Ascii,
        Matrix,
        Color,
    }
}

arg_enum! {
    #[derive(Debug)]
    enum Brightness {
        Average,
        MinMax,
        Luma,
    }
}

fn main() -> Result<(), image::ImageError> {
    let opts = Cmd::from_args();
    let image = image::open(&opts.image)?.resize(opts.width, opts.width, Lanczos3);
    let ascii_image = image
        .pixels()
        .map(|(_, _, pixel)| {
            let rgb = pixel.channels();
            let mut brightness = match opts.brightness {
                Brightness::Average => (rgb[0] as usize + rgb[1] as usize + rgb[2] as usize) / 3,
                Brightness::MinMax => {
                    (rgb[0].max(rgb[1].max(rgb[2])) as usize
                        + rgb[0].min(rgb[1].min(rgb[2])) as usize)
                        / 2
                }
                Brightness::Luma => pixel.to_luma().channels()[0] as usize,
            };

            if opts.invert {
                brightness = (brightness as isize - 255).abs() as usize
            }

            let index = (brightness as f32 / 255.0 * 64.0) as usize;

            match opts.output {
                Output::Ascii => format!("{}", BRIGHTNESS_CHARS[index]).white(),
                Output::Matrix => format!("{}", '▉').truecolor(0, brightness as u8, 0),
                Output::Color => format!("{}", '▉').truecolor(rgb[0], rgb[1], rgb[2]),
            }
        })
        .collect::<Vec<ColoredString>>();

    // Can I combine this into one iter?
    for line in ascii_image.chunks(image.width() as usize) {
        for char in line {
            print!("{0}{0}", char);
        }
        println!();
    }

    Ok(())
}
