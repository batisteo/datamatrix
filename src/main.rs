extern crate clap;
extern crate css_color_parser;
extern crate image;


use clap::{App, Arg};
use css_color_parser::Color as CssColor;
use image::{ImageBuffer, Rgb};

const WIDTH: u32 = 11;
const HEIGHT: u32 = 11;

fn main() {
    let args = App::new("myapp")
    .version("0.1.0")
    .author("Baptiste Darthenay")
    .about("DataMatrix generator")
    .arg(Arg::with_name("output")
        .help("The output PNG file")
        .long("output")
        .short("o")
        .takes_value(true))
    .arg(Arg::with_name("color")
        .help("Color of the dots")
        .long("color")
        .short("c")
        .takes_value(true))
    .arg(Arg::with_name("background")
        .help("Background color")
        .long("background")
        .short("b")
        .takes_value(true))
    .get_matches();

    let file_name = args.value_of("output").unwrap_or("datamatrix.png");
    let color: CssColor = args.value_of("color").unwrap_or("black").parse().unwrap();
    let bg: CssColor = args.value_of("background").unwrap_or("white").parse().unwrap();

    let bg_pixel = Rgb {data: [bg.r, bg.g, bg.b]};
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_pixel(WIDTH, HEIGHT, bg_pixel);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if (y == 0 && x % 2 == 0) || (x == HEIGHT -1 && y % 2 == 0) || x == 0 || y == WIDTH -1 {
                image.get_pixel_mut(x, y).data = [color.r, color.g, color.b];
            }
        }
    }

    image.save(file_name).unwrap();
    println!("Datamatrix generated in {}", file_name);
}
