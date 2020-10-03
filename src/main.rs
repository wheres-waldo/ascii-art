use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cmd {
    #[structopt(parse(from_os_str))]
    pub image: PathBuf,
}

fn main() -> Result<(), image::ImageError> {
    let opts = Cmd::from_args();
    let (x, y) = image::image_dimensions(opts.image)?;

    println!("Successfully loaded image!");
    println!("Image size {} x {}", x, y);

    Ok(())
}
