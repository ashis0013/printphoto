use image::{imageops::resize, io::Reader as ImageReader};
use std::{
    env,
    io::{Result, Write},
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn write_block(stdout: &mut StandardStream, r: u8, g: u8, b: u8) -> Result<()> {
    stdout.set_color(ColorSpec::new().set_bg(Some(Color::Rgb(r, g, b))))?;
    write!(stdout, " ")?;
    Ok(())
}

fn print_image(path: String, width: u32) -> Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    let buff = resize(
        &img,
        width,
        ((img.height() as f64 / img.width() as f64) * (width as f64)) as u32,
        image::imageops::FilterType::Gaussian,
    );
    let mut pixels = buff.pixels();
    for _ in 0..buff.height() {
        for _ in 0..buff.width() {
            let pixel = pixels.next().unwrap();
            write_block(&mut stdout, pixel.0[0], pixel.0[1], pixel.0[2])?;
        }
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Black)))?;
        writeln!(&mut stdout, "")?;
    }
    writeln!(&mut stdout, "")?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide path to image");
    } else if args.len() == 2 {
        print_image(args[1].to_owned(), 64)?;
    } else if args.len() == 4 && args[2] == "-w" {
        print_image(args[1].to_owned(), args[3].parse().unwrap())?;
    }
    Ok(())
}
