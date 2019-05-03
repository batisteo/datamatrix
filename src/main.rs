use clap::{App, Arg};
use css_color_parser::Color as CssColor;
use rand;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use image::{ImageBuffer, Rgb};
const HEIGHT: u32 = 8 + 1;
const WIDTH: u32 = 8 + 1;

fn generate_matrix() -> Vec<Vec<bool>> {
    let mut matrix = Vec::new();
    for _y in 0..HEIGHT {
        let mut row = Vec::new();
        for _x in 0..WIDTH {
            row.push(rand::random());
        }
        matrix.push(row);
    }
    matrix
}

fn generate_finder(data: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let height: u32 = HEIGHT + 2;
    let width: u32 = WIDTH + 2;
    let mut datamatrix = Vec::new();
    for _ in 0..height {
        datamatrix.push(vec![false; width as usize])
    }

    for y in 0..height {
        for x in 0..width {
            if (y == 0 && x % 2 == 0) || (x == height - 1 && y % 2 == 0) || x == 0 || y == width - 1
            {
                datamatrix[y as usize][x as usize] = true
            } else if y != 0 && y < HEIGHT + 1 && x != 0 && x < WIDTH + 1 {
                let value = data[y as usize - 1][x as usize - 1];
                datamatrix[y as usize][x as usize] = value;
            }
        }
    }

    datamatrix
}

fn generate_margin(datamatrix: Vec<Vec<bool>>, size: u32) -> Vec<Vec<bool>> {
    let height: u32 = HEIGHT + 2 + size * 2;
    let width: u32 = WIDTH + 2 + size * 2;
    let mut generated = Vec::new();
    for _ in 0..height {
        generated.push(vec![false; width as usize])
    }

    for y in 0..height {
        for x in 0..width {
            if y > size - 1 && y < height - size && x > size - 1 && x < height - size {
                let value = datamatrix[(y - size) as usize][(x - size) as usize];
                generated[y as usize][x as usize] = value;
            }
        }
    }
    generated
}

fn output_image(args: &clap::ArgMatches, file_name: &String) {
    let color: CssColor = args.value_of("color").unwrap_or("black").parse().unwrap();
    let bg: CssColor = args
        .value_of("background")
        .unwrap_or("white")
        .parse()
        .unwrap();

    let bg_pixel = Rgb {
        data: [bg.r, bg.g, bg.b],
    };
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_pixel(WIDTH, HEIGHT, bg_pixel);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if (y == 0 && x % 2 == 0) || (x == HEIGHT - 1 && y % 2 == 0) || x == 0 || y == WIDTH - 1
            {
                image.get_pixel_mut(x, y).data = [color.r, color.g, color.b];
            }
        }
    }

    image.save(file_name).unwrap();
    println!("Datamatrix generated in {}", file_name);
}

fn output_terminal(_args: &clap::ArgMatches, matrix: &Vec<Vec<bool>>) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    fn set_color(stdout: &mut StandardStream, color: Color) {
        let bg = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
            _ => Color::White,
        };
        stdout
            .set_color(ColorSpec::new().set_fg(Some(color)).set_bg(Some(bg)))
            .unwrap();
    }

    let mut current: Option<&bool> = None;
    for row in matrix.iter() {
        for value in row.iter() {
            if current.unwrap_or(&!value) != value {
                let current_color = match value {
                    true => Color::Black,
                    false => Color::White,
                };
                set_color(&mut stdout, current_color)
            }
            write!(&mut stdout, "██").unwrap();
            current = Some(value);
        }
        stdout.reset().unwrap();
        write!(&mut stdout, "\n").unwrap();
    }
}

fn main() {
    let args = App::new("myapp")
        .version("0.1.0")
        .author("Baptiste Darthenay")
        .about("DataMatrix generator")
        .arg(
            Arg::with_name("output")
                .help("The output PNG file")
                .long("output")
                .short("o")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("color")
                .help("Color of the dots")
                .long("color")
                .short("c")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("background")
                .help("Background color")
                .long("background")
                .short("b")
                .takes_value(true),
        )
        .get_matches();

    let file_name = args.value_of("output");
    let matrix = generate_matrix();
    let matrix = generate_finder(matrix);
    let matrix = generate_margin(matrix, 2);
    match file_name {
        Some(file_name) => output_image(&args, &file_name.to_owned()),
        None => output_terminal(&args, &matrix),
    }
}
