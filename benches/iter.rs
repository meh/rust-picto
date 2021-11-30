#![feature(test)]
extern crate test;

mod pixels {
	use picto::color::Rgba;
	use test::{self, Bencher};

	#[bench]
	fn image(b: &mut Bencher) {
		let image = image::open("tests/rainbow.png").unwrap().to_rgba();

		b.iter(|| {
			for (x, y, px) in image.enumerate_pixels() {
				test::black_box((x, y, px[0]));
			}
		})
	}

	#[bench]
	fn picto(b: &mut Bencher) {
		let image = picto::read::from_path::<Rgba, u8, _>("tests/rainbow.png").unwrap();

		b.iter(|| {
			for (x, y, px) in image.pixels() {
				test::black_box((x, y, px.get().red));
			}
		})
	}
}

mod pixels_mut {
	use picto::color::Rgba;
	use test::{self, Bencher};

	#[bench]
	fn image(b: &mut Bencher) {
		let mut image = image::open("tests/rainbow.png").unwrap().to_rgba();

		b.iter(|| {
			for (x, y, mut px) in image.enumerate_pixels_mut() {
				*px = image::Rgba {
					data: [0, 127, 255, 255],
				};
				test::black_box((x, y));
			}
		})
	}

	#[bench]
	fn picto(b: &mut Bencher) {
		let mut image = picto::read::from_path::<Rgba, u8, _>("tests/rainbow.png").unwrap();

		b.iter(|| {
			for (x, y, mut px) in image.pixels_mut() {
				px.set(&Rgba::new(0.0, 0.5, 1.0, 1.0));
				test::black_box((x, y));
			}
		})
	}
}
