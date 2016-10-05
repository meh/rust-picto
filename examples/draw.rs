//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

extern crate clap;
use clap::{App, Arg};

extern crate picto;
use picto::color::Rgb;
use picto::processing::prelude::*;

extern crate term_size;


const UPPER: char = '▀';
const LOWER: char = '▄';

fn main() {
	let matches = App::new("draw")
		.version(env!("CARGO_PKG_VERSION"))
		.about("Draw an image in your terminal.")
		.arg(Arg::with_name("INPUT")
			.index(1)
			.required(true)
			.help("The path to the image to draw."))
		.arg(Arg::with_name("margin")
			.short("m")
			.long("margin")
			.takes_value(true)
			.help("Add a margin around the picture."))
		.get_matches();

	let margin          = matches.value_of("margin").unwrap_or("0").parse::<usize>().unwrap();
	let (width, height) = term_size::dimensions().expect("not a terminal?");
	let (width, height) = (width - (margin * 2), (height - (margin * 2)) * 2);
	let image           = picto::read::from_path::<Rgb, u8, _>(matches.value_of("INPUT").unwrap()).unwrap()
		.scale_to::<scaler::Lanczos3>(width as u32, height as u32);

	let offset = (width as u32 - image.width()) / 2;

	println!("");
	for y in 0 .. image.height() / 2 {
		let y = y * 2;

		print!("{:1$}", "", offset as usize);
		for x in 0 .. image.width() {
			let (top, bottom): ((u8, u8, u8), (u8, u8, u8)) =
				(image.get(x, y).to_pixel(), image.get(x, y + 1).to_pixel());

			print!("\x1B[38;2;{};{};{}m\
			        \x1B[48;2;{};{};{}m{}\x1B[0m",

			top.0, top.1, top.2,
			bottom.0, bottom.1, bottom.2,
			UPPER);
		}
		println!("");
	}
	println!("");
}
