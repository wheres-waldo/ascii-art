use image::{imageops::FilterType::*, GenericImageView, Pixel};
use std::path::PathBuf;
use structopt::StructOpt;

const BRIGHTNESS_CHARS: [char; 65] = [
    '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '~', '+', '_', '-', '?', ']', '[', '}', '{',
    '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U',
    'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', '#',
    'M', 'W', '&', '8', '%', 'B', '@', '$',
];

const RESIZE: u32 = 90;

#[derive(StructOpt)]
struct Cmd {
    #[structopt(parse(from_os_str))]
    image: PathBuf,
}

fn main() -> Result<(), image::ImageError> {
    let opts = Cmd::from_args();
    let image = image::open(opts.image)?.resize(RESIZE, RESIZE, Lanczos3);
    let ascii_image = image
        .pixels()
        .map(|(_, _, pixel)| {
            let rgb = pixel.channels();
            let brightness = (rgb[0] as usize + rgb[1] as usize + rgb[2] as usize) / 3;
            let index = (brightness as f32 / 255.0 * 64.0) as usize;
            BRIGHTNESS_CHARS[index]
        })
        .collect::<Vec<char>>();

    for line in ascii_image.chunks(image.width() as usize) {
        for char in line {
            print!("{0}{0}", char);
        }
        println!();
    }

    Ok(())
}
