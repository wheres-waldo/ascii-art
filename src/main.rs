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

    println!("Successfully loaded image!");
    println!("Pixel matrix size {} x {}", x, y);
    println!("Iterating through pixel contents:");

    for (_, _, pixel) in image.pixels() {
        let rgb = pixel.channels();
        println!("({}, {}, {})", rgb[0], rgb[1], rgb[2]);
    }

    Ok(())
}
