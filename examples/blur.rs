extern crate picto;
use picto::color::Rgba;
use picto::processing::prelude::*;

use std::env;

fn main() {
	let image = picto::read::from_path::<u8, Rgba, _>(env::args().nth(1).unwrap()).unwrap();
	let by    = env::args().nth(2).unwrap().parse::<f32>().unwrap();

	picto::write::to_path(env::args().nth(3).unwrap(), &image.blur::<u8, Rgba>(by)).unwrap();
}
