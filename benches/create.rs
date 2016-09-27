#![feature(test)]
extern crate test;
extern crate picto;
extern crate image;

mod from_fn {
	use test::Bencher;
	use image;
	use picto;

	#[bench]
	fn image(b: &mut Bencher) {
		b.iter(|| image::RgbImage::from_fn(1024, 1024, |x, y| {
			let w = (x as f32 + y as f32) / 2048.0;

			image::Rgb { data: [
				(w * 255.0) as u8,
				(w * 255.0) as u8,
				(w * 255.0) as u8
			] }
		}));
	}

	#[bench]
	fn picto(b: &mut Bencher) {
		b.iter(|| picto::Buffer::<u8, picto::color::Rgb, _>::from_fn(1024, 1024, |x, y| {
			let w = (x as f32 + y as f32) / 2048.0;

			picto::color::Rgb::new(w, w, w)
		}));
	}
}

mod from_pixel {
	use test::Bencher;
	use image;
	use picto;

	#[bench]
	fn image(b: &mut Bencher) {
		b.iter(|| image::RgbImage::from_pixel(1024, 1024, image::Rgb { data: [0, 0, 0] }))
	}

	#[bench]
	fn picto(b: &mut Bencher) {
		b.iter(|| picto::Buffer::<u8, picto::color::Rgb, _>::from_pixel(1024, 1024, &picto::color::Rgb::new_u8(0, 0, 0)))
	}
}
