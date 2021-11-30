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
use picto::{color::Rgba, processing::prelude::*};

fn main() {
	let matches = App::new("resize")
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
		.arg(Arg::with_name("by")
			.short("b")
			.long("by")
			.takes_value(true)
			.required(true)
			.help("The amount to scale by."))
		.arg(Arg::with_name("scaler")
			.short("s")
			.long("scaler")
			.takes_value(true)
			.validator(is_scaler)
			.help("The scaling algorithm to use (nearest, linear, cubic, gaussian, lanczos2, lanczos3, super-xbr). (default is `nearest`)"))
		.get_matches();

	let image = picto::read::from_path::<Rgba, u8, _>(matches.value_of("INPUT").unwrap()).unwrap();
	let by = matches.value_of("by").unwrap().parse::<f32>().unwrap();

	picto::write::to_path(matches.value_of("OUTPUT").unwrap(), &match &*matches
		.value_of("scaler")
		.unwrap_or("nearest")
		.to_lowercase()
	{
		"nearest" => image.scale_by::<scaler::Nearest>(by),
		"linear" => image.scale_by::<scaler::Linear>(by),
		"cubic" => image.scale_by::<scaler::Cubic>(by),
		"gaussian" => image.scale_by::<scaler::Gaussian>(by),
		"lanczos2" => image.scale_by::<scaler::Lanczos2>(by),
		"lanczos3" => image.scale_by::<scaler::Lanczos3>(by),
		"super-xbr" => image.scale_by::<scaler::xbr::Super>(by),

		_ => unreachable!(),
	})
	.unwrap();
}

fn is_scaler(arg: String) -> Result<(), String> {
	match &*arg.to_lowercase() {
		"nearest" | "linear" | "cubic" | "gaussian" | "lanczos2" | "lanczos3" | "super-xbr" => Ok(()),

		_ => Err("unknown scaler".into()),
	}
}
