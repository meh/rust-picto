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
use picto::color::Rgba;
use picto::processing::prelude::*;

fn main() {
	let matches = App::new("dither")
		.version(env!("CARGO_PKG_VERSION"))
		.about("Resize an image.")
		.arg(Arg::with_name("INPUT")
			.index(1)
			.required(true)
			.help("Path to the input image."))
		.arg(Arg::with_name("OUTPUT")
			.index(2)
			.required(true)
			.help("Path to the output image"))
		.arg(Arg::with_name("colors")
			.short("c")
			.long("colors")
			.takes_value(true)
			.required(true)
			.help("The number of colors."))
		.arg(Arg::with_name("ditherer")
			.short("d")
			.long("ditherer")
			.takes_value(true)
			.validator(is_ditherer)
			.help("The dithering algorithm to use (neuquant, pal-mono-dark). (default is `neuquant`)"))
		.get_matches();

	let image  = picto::read::from_path::<Rgba, u8, _>(matches.value_of("INPUT").unwrap()).unwrap();
	let colors = matches.value_of("colors").unwrap().parse::<u32>().unwrap();

	picto::write::to_path(matches.value_of("OUTPUT").unwrap(), &match &*matches.value_of("ditherer").unwrap_or("neuquant").to_lowercase() {
		"neuquant" =>
			image.dither::<ditherer::neuquant::Best>(colors),

		"mono-dark" =>
			image.dither::<ditherer::Palette<ditherer::palette::table::MonoDark>>(colors),

		"mono-light" =>
			image.dither::<ditherer::Palette<ditherer::palette::table::MonoLight>>(colors),

		"mono-gray-1" =>
			image.dither::<ditherer::Palette<ditherer::palette::table::Gray1>>(colors),

		"mono-gray-2" =>
			image.dither::<ditherer::Palette<ditherer::palette::table::Gray2>>(colors),

		"mono-gray-4" =>
			image.dither::<ditherer::Palette<ditherer::palette::table::Gray4>>(colors),

		"mono-gray-8" =>
			image.dither::<ditherer::Palette<ditherer::palette::table::Gray8>>(colors),

		"xterm" =>
			image.dither::<ditherer::Palette<ditherer::palette::table::XTerm>>(colors),

		"vt340" =>
			image.dither::<ditherer::Palette<ditherer::palette::table::VT340>>(colors),

		_ =>
			unreachable!()
	}).unwrap();
}

fn is_ditherer(arg: String) -> Result<(), String> {
	match &*arg.to_lowercase() {
		"neuquant" |
		"mono-dark" |
		"mono-light" |
		"mono-gray-1" |
		"mono-gray-2" |
		"mono-gray-4" |
		"mono-gray-8" |
		"xterm" |
		"vt340" =>
			Ok(()),

		_ =>
			Err("unknown ditherer".into())
	}
}
