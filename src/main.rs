use image::{GenericImageView, Pixel};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cmd {
    #[structopt(parse(from_os_str))]
    pub image: PathBuf,
}

fn main() -> Result<(), image::ImageError> {
    let opts = Cmd::from_args();
    let image = image::open(opts.image)?;
    let (x, y) = image.dimensions();

    println!("Successfully constructed brightness matrix!");
    println!("Brightness matrix size {} x {}", x, y);
    println!("Iterating through pixel brightnesses:");

    image
        .pixels()
        .map(|(_, _, pixel)| {
            let rgb = pixel.channels();
            ((rgb[0] as u32 + rgb[1] as u32 + rgb[2] as u32) / 3) as u8
        })
        .for_each(|brightness| {
            println!("{}", brightness);
        });

    Ok(())
}
