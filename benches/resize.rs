#![feature(test)]
extern crate test;

macro_rules! image {
	($b:expr, $path:expr, $algorithm:ident, $by:expr) => {
		let image = image::open($path).unwrap();
		let (width, height) = image.dimensions();

		$b.iter(|| {
			image.resize(
				(width as f32 * $by) as u32,
				(height as f32 * $by) as u32,
				image::FilterType::$algorithm,
			)
		})
	};
}

macro_rules! picto {
	($b:expr, $path:expr, $algorithm:ident, $by:expr) => {
		let image = picto::read::from_path::<Rgba, u8, _>($path).unwrap();
		let (width, height) = image.dimensions();

		$b.iter(|| image.resize::<scaler::$algorithm>((width as f32 * $by) as u32, (height as f32 * $by) as u32));
	};

	($b:expr, $path:expr, $module:ident::$algorithm:ident, $by:expr) => {
		let image = picto::read::from_path::<Rgba, u8, _>($path).unwrap();
		let (width, height) = image.dimensions();

		$b.iter(|| image.resize::<scaler::$module::$algorithm>((width as f32 * $by) as u32, (height as f32 * $by) as u32));
	};
}

mod nearest {
	use image::{self, GenericImage};
	use picto::{color::Rgba, processing::prelude::*};
	use test::Bencher;

	#[bench]
	fn image_0x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Nearest, 0.5);
	}

	#[bench]
	fn image_1x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Nearest, 1.0);
	}

	#[bench]
	fn image_2x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Nearest, 2.0);
	}

	#[bench]
	fn image_3x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Nearest, 3.0);
	}

	#[bench]
	fn image_4x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Nearest, 4.0);
	}

	#[bench]
	fn picto_0x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Nearest, 0.5);
	}

	#[bench]
	fn picto_1x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Nearest, 1.0);
	}

	#[bench]
	fn picto_2x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Nearest, 2.0);
	}

	#[bench]
	fn picto_3x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Nearest, 3.0);
	}

	#[bench]
	fn picto_4x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Nearest, 4.0);
	}
}

mod linear {
	use image::{self, GenericImage};
	use picto::{color::Rgba, processing::prelude::*};
	use test::Bencher;

	#[bench]
	fn image_0x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Triangle, 0.5);
	}

	#[bench]
	fn image_1x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Triangle, 1.0);
	}

	#[bench]
	fn image_2x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Triangle, 2.0);
	}

	#[bench]
	fn image_3x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Triangle, 3.0);
	}

	#[bench]
	fn image_4x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Triangle, 4.0);
	}

	#[bench]
	fn picto_0x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Linear, 0.5);
	}

	#[bench]
	fn picto_1x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Linear, 1.0);
	}

	#[bench]
	fn picto_2x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Linear, 2.0);
	}

	#[bench]
	fn picto_3x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Linear, 3.0);
	}

	#[bench]
	fn picto_4x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Linear, 4.0);
	}
}

mod cubic {
	use image::{self, GenericImage};
	use picto::{color::Rgba, processing::prelude::*};
	use test::Bencher;

	#[bench]
	fn image_0x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", CatmullRom, 0.5);
	}

	#[bench]
	fn image_1x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", CatmullRom, 1.0);
	}

	#[bench]
	fn image_2x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", CatmullRom, 2.0);
	}

	#[bench]
	fn image_3x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", CatmullRom, 3.0);
	}

	#[bench]
	fn image_4x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", CatmullRom, 4.0);
	}

	#[bench]
	fn picto_0x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Cubic, 0.5);
	}

	#[bench]
	fn picto_1x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Cubic, 1.0);
	}

	#[bench]
	fn picto_2x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Cubic, 2.0);
	}

	#[bench]
	fn picto_3x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Cubic, 3.0);
	}

	#[bench]
	fn picto_4x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Cubic, 4.0);
	}
}

mod gaussian {
	use image::{self, GenericImage};
	use picto::{color::Rgba, processing::prelude::*};
	use test::Bencher;

	#[bench]
	fn image_0x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Gaussian, 0.5);
	}

	#[bench]
	fn image_1x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Gaussian, 1.0);
	}

	#[bench]
	fn image_2x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Gaussian, 2.0);
	}

	#[bench]
	fn image_3x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Gaussian, 3.0);
	}

	#[bench]
	fn image_4x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Gaussian, 4.0);
	}

	#[bench]
	fn picto_0x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Gaussian, 0.5);
	}

	#[bench]
	fn picto_1x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Gaussian, 1.0);
	}

	#[bench]
	fn picto_2x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Gaussian, 2.0);
	}

	#[bench]
	fn picto_3x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Gaussian, 3.0);
	}

	#[bench]
	fn picto_4x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Gaussian, 4.0);
	}
}

mod lanczos2 {
	use picto::{color::Rgba, processing::prelude::*};
	use test::Bencher;

	#[bench]
	fn picto_0x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Lanczos2, 0.5);
	}

	#[bench]
	fn picto_1x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Lanczos2, 1.0);
	}

	#[bench]
	fn picto_2x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Lanczos2, 2.0);
	}

	#[bench]
	fn picto_3x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Lanczos2, 3.0);
	}

	#[bench]
	fn picto_4x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Lanczos2, 4.0);
	}
}

mod lanczos3 {
	use image::{self, GenericImage};
	use picto::{color::Rgba, processing::prelude::*};
	use test::Bencher;

	#[bench]
	fn image_0x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Lanczos3, 0.5);
	}

	#[bench]
	fn image_1x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Lanczos3, 1.0);
	}

	#[bench]
	fn image_2x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Lanczos3, 2.0);
	}

	#[bench]
	fn image_3x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Lanczos3, 3.0);
	}

	#[bench]
	fn image_4x(b: &mut Bencher) {
		image!(b, "tests/rainbow.png", Lanczos3, 4.0);
	}

	#[bench]
	fn picto_0x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Lanczos3, 0.5);
	}

	#[bench]
	fn picto_1x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Lanczos3, 1.0);
	}

	#[bench]
	fn picto_2x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Lanczos3, 2.0);
	}

	#[bench]
	fn picto_3x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Lanczos3, 3.0);
	}

	#[bench]
	fn picto_4x(b: &mut Bencher) {
		picto!(b, "tests/rainbow.png", Lanczos3, 4.0);
	}
}

mod xbr {
	mod zuper {
		use picto::{color::Rgba, processing::prelude::*};
		use test::Bencher;

		#[bench]
		fn picto_2x(b: &mut Bencher) {
			picto!(b, "tests/rainbow.png", xbr::Super, 2.0);
		}

		#[bench]
		fn picto_4x(b: &mut Bencher) {
			picto!(b, "tests/rainbow.png", xbr::Super, 4.0);
		}
	}
}
