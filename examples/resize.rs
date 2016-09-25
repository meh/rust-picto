extern crate picto;
use picto::color::Rgba;
use picto::processing::prelude::*;

use std::env;

fn main() {
	let image   = picto::read::from_path::<u8, Rgba, _>(env::args().nth(1).unwrap()).unwrap();
	let scaler  = env::var("SCALER").unwrap_or("nearest".into()).to_lowercase();
	let by      = env::args().nth(2).unwrap().parse::<f32>().unwrap();
	let resized = match &*scaler {
		"nearest" =>
			image.scale_by::<scaler::Nearest, u8, Rgba>(by),

		"linear" =>
			image.scale_by::<scaler::Linear, u8, Rgba>(by),

		"cubic" =>
			image.scale_by::<scaler::Cubic, u8, Rgba>(by),

		"lanczos2" =>
			image.scale_by::<scaler::Lanczos2, u8, Rgba>(by),

		"lanczos3" =>
			image.scale_by::<scaler::Lanczos3, u8, Rgba>(by),

		_ =>
			panic!("unknown scaler")
	};

	picto::write::to_path(env::args().nth(3).unwrap(), &resized).unwrap();
}
