#![feature(test)]
extern crate test;
extern crate picto;
extern crate image;

mod nearest {
	use test::Bencher;
	use image::{self, GenericImage};
	use picto;
	use picto::color::Rgba;
	use picto::processing::prelude::*;

	#[bench]
	fn image_0x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width / 2, height / 2, image::FilterType::Nearest))
	}

	#[bench]
	fn image_2x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width * 2, height * 2, image::FilterType::Nearest))
	}

	#[bench]
	fn image_3x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width * 3, height * 3, image::FilterType::Nearest))
	}

	#[bench]
	fn image_4x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width * 4, height * 4, image::FilterType::Nearest))
	}

	#[bench]
	fn picto_0x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Nearest, u8, Rgba>(width / 2, height / 2))
	}

	#[bench]
	fn picto_2x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Nearest, u8, Rgba>(width * 2, height * 2))
	}

	#[bench]
	fn picto_3x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Nearest, u8, Rgba>(width * 3, height * 3))
	}

	#[bench]
	fn picto_4x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Nearest, u8, Rgba>(width * 4, height * 4))
	}
}

mod linear {
	use test::Bencher;
	use image::{self, GenericImage};
	use picto;
	use picto::color::Rgba;
	use picto::processing::prelude::*;

	#[bench]
	fn image_0x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width / 2, height / 2, image::FilterType::Triangle))
	}

	#[bench]
	fn image_2x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width * 2, height * 2, image::FilterType::Triangle))
	}

	#[bench]
	fn image_3x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width * 3, height * 3, image::FilterType::Triangle))
	}

	#[bench]
	fn image_4x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width * 4, height * 4, image::FilterType::Triangle))
	}

	#[bench]
	fn picto_0x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Linear, u8, Rgba>(width / 2, height / 2))
	}

	#[bench]
	fn picto_2x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Linear, u8, Rgba>(width * 2, height * 2))
	}

	#[bench]
	fn picto_3x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Linear, u8, Rgba>(width * 3, height * 3))
	}

	#[bench]
	fn picto_4x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Linear, u8, Rgba>(width * 4, height * 4))
	}
}

mod cubic {
	use test::Bencher;
	use image::{self, GenericImage};
	use picto;
	use picto::color::Rgba;
	use picto::processing::prelude::*;

	#[bench]
	fn image_0x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width / 2, height / 2, image::FilterType::CatmullRom))
	}

	#[bench]
	fn image_2x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width * 2, height * 2, image::FilterType::CatmullRom))
	}

	#[bench]
	fn image_3x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width * 3, height * 3, image::FilterType::CatmullRom))
	}

	#[bench]
	fn image_4x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width * 4, height * 4, image::FilterType::CatmullRom))
	}

	#[bench]
	fn picto_0x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Cubic, u8, Rgba>(width / 2, height / 2))
	}

	#[bench]
	fn picto_2x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Cubic, u8, Rgba>(width * 2, height * 2))
	}

	#[bench]
	fn picto_3x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Cubic, u8, Rgba>(width * 3, height * 3))
	}

	#[bench]
	fn picto_4x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Cubic, u8, Rgba>(width * 4, height * 4))
	}
}

mod lanczos2 {
	use test::Bencher;
	use picto;
	use picto::color::Rgba;
	use picto::processing::prelude::*;

	#[bench]
	fn picto_0x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Lanczos2, u8, Rgba>(width / 2, height / 2))
	}

	#[bench]
	fn picto_2x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Lanczos2, u8, Rgba>(width * 2, height * 2))
	}

	#[bench]
	fn picto_3x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Lanczos2, u8, Rgba>(width * 3, height * 3))
	}

	#[bench]
	fn picto_4x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Lanczos2, u8, Rgba>(width * 4, height * 4))
	}
}

mod lanczos3 {
	use test::Bencher;
	use image::{self, GenericImage};
	use picto;
	use picto::color::Rgba;
	use picto::processing::prelude::*;

	#[bench]
	fn image_0x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width / 2, height / 2, image::FilterType::Lanczos3))
	}

	#[bench]
	fn image_2x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width * 2, height * 2, image::FilterType::Lanczos3))
	}

	#[bench]
	fn image_3x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width * 3, height * 3, image::FilterType::Lanczos3))
	}

	#[bench]
	fn image_4x(b: &mut Bencher) {
		let image           = image::open("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize(width * 4, height * 4, image::FilterType::Lanczos3))
	}

	#[bench]
	fn picto_0x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Lanczos3, u8, Rgba>(width / 2, height / 2))
	}

	#[bench]
	fn picto_2x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Lanczos3, u8, Rgba>(width * 2, height * 2))
	}

	#[bench]
	fn picto_3x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Lanczos3, u8, Rgba>(width * 3, height * 3))
	}

	#[bench]
	fn picto_4x(b: &mut Bencher) {
		let image           = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();
		let (width, height) = image.dimensions();

		b.iter(|| image.resize::<scaler::Lanczos3, u8, Rgba>(width * 4, height * 4))
	}
}
