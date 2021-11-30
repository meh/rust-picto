#![feature(test)]
extern crate test;

mod png {
	mod read {
		use test::Bencher;
		use picto::color::{Rgb, Rgba};

		#[bench]
		fn image(b: &mut Bencher) {
			b.iter(|| image::open("tests/rainbow.png").unwrap());
		}

		#[bench]
		fn picto_with_convert(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<Rgba, u8, _>("tests/rainbow.png").unwrap());
		}

		#[bench]
		fn picto_as_is(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<Rgb, u8, _>("tests/rainbow.png").unwrap());
		}
	}
}

mod jpeg {
	mod read {
		use test::Bencher;
		use picto::color::{Rgb, Rgba};

		#[bench]
		fn image(b: &mut Bencher) {
			b.iter(|| image::open("tests/rainbow.jpeg").unwrap());
		}

		#[bench]
		fn picto_with_convert(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<Rgba, u8, _>("tests/rainbow.jpeg").unwrap());
		}

		#[bench]
		fn picto_as_is(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<Rgb, u8, _>("tests/rainbow.jpeg").unwrap());
		}
	}
}

mod bmp {
	mod read {
		use test::Bencher;
		use picto::color::{Rgb, Rgba};

		#[bench]
		fn image(b: &mut Bencher) {
			b.iter(|| image::open("tests/rainbow.bmp").unwrap());
		}

		#[bench]
		fn picto_with_convert(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<Rgba, u8, _>("tests/rainbow.bmp").unwrap());
		}

		#[bench]
		fn picto_as_is(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<Rgb, u8, _>("tests/rainbow.bmp").unwrap());
		}
	}
}

mod tga {
	mod read {
		use test::Bencher;
		use picto::color::{Rgb, Rgba};

		#[bench]
		fn image(b: &mut Bencher) {
			b.iter(|| image::open("tests/rainbow.tga").unwrap());
		}

		#[bench]
		fn picto_with_convert(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<Rgba, u8, _>("tests/rainbow.tga").unwrap());
		}

		#[bench]
		fn picto_as_is(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<Rgb, u8, _>("tests/rainbow.tga").unwrap());
		}
	}
}

mod xyz {
	mod read {
		use test::Bencher;
		use picto::color::{Rgb, Rgba};

		#[bench]
		fn picto_with_convert(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<Rgba, u8, _>("tests/boat.xyz").unwrap());
		}

		#[bench]
		fn picto_as_is(b: &mut Bencher) {
			b.iter(|| picto::read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap());
		}
	}
}
