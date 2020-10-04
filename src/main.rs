use image::{GenericImageView, Pixel};
use std::path::PathBuf;
use structopt::StructOpt;

const BRIGHTNESS_CHARS: [char; 65] = [
    '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '~', '+', '_', '-', '?', ']', '[', '}', '{',
    '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U',
    'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', '#',
    'M', 'W', '&', '8', '%', 'B', '@', '$',
];

#[derive(StructOpt)]
pub struct Cmd {
    #[structopt(parse(from_os_str))]
    pub image: PathBuf,
}

fn main() -> Result<(), image::ImageError> {
    let opts = Cmd::from_args();
    let image = image::open(opts.image)?;
    let (x, y) = image.dimensions();

    println!("Successfully constructed ASCII matrix!");
    println!("ASCII matrix size {} x {}", x, y);
    println!("Iterating through pixel ASCII characters:");

    image
        .pixels()
        .map(|(_, _, pixel)| {
            let rgb = pixel.channels();
            let brightness = (rgb[0] as usize + rgb[1] as usize + rgb[2] as usize) / 3;
            let index = (brightness as f32 / 255.0 * 64.0) as usize;
            BRIGHTNESS_CHARS[index]
        })
        .for_each(|brightness| {
            println!("{}", brightness);
        });

    Ok(())
}
