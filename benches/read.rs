#![feature(test)]
extern crate test;
extern crate picto;
extern crate image;

mod png {
	mod read {
		use test::Bencher;
		use image;
		use picto;

		#[bench]
		fn image(b: &mut Bencher) {
			b.iter(|| image::open("tests/rainbow.png").unwrap());
		}

		#[bench]
		fn picto_with_convert(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<u8, picto::color::Rgba, _>("tests/rainbow.png").unwrap());
		}

		#[bench]
		fn picto_as_is(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<u8, picto::color::Rgb, _>("tests/rainbow.png").unwrap());
		}
	}
}

mod jpeg {
	mod read {
		use test::Bencher;
		use image;
		use picto;

		#[bench]
		fn image(b: &mut Bencher) {
			b.iter(|| image::open("tests/rainbow.jpeg").unwrap());
		}

		#[bench]
		fn picto_with_convert(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<u8, picto::color::Rgba, _>("tests/rainbow.jpeg").unwrap());
		}

		#[bench]
		fn picto_as_is(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<u8, picto::color::Rgb, _>("tests/rainbow.jpeg").unwrap());
		}
	}
}

mod bmp {
	mod read {
		use test::Bencher;
		use image;
		use picto;

		#[bench]
		fn image(b: &mut Bencher) {
			b.iter(|| image::open("tests/rainbow.bmp").unwrap());
		}

		#[bench]
		fn picto_with_convert(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<u8, picto::color::Rgba, _>("tests/rainbow.bmp").unwrap());
		}

		#[bench]
		fn picto_as_is(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<u8, picto::color::Rgb, _>("tests/rainbow.bmp").unwrap());
		}
	}
}

mod tga {
	mod read {
		use test::Bencher;
		use image;
		use picto;

		#[bench]
		fn image(b: &mut Bencher) {
			b.iter(|| image::open("tests/rainbow.tga").unwrap());
		}

		#[bench]
		fn picto_with_convert(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<u8, picto::color::Rgba, _>("tests/rainbow.tga").unwrap());
		}

		#[bench]
		fn picto_as_is(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<u8, picto::color::Rgb, _>("tests/rainbow.tga").unwrap());
		}
	}
}

mod xyz {
	mod read {
		use test::Bencher;
		use picto;

		#[bench]
		fn picto_with_convert(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<u8, picto::color::Rgba, _>("tests/boat.xyz").unwrap());
		}

		#[bench]
		fn picto_as_is(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<u8, picto::color::Rgb, _>("tests/boat.xyz").unwrap());
		}
	}
}
