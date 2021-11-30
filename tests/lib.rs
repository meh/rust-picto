#[macro_use]
extern crate approx;
extern crate picto;

mod png {
	use picto::{self, color::*};

	#[test]
	fn read_as_is() {
		let image = picto::read::from_path::<Rgb, u8, _>("tests/rainbow.png").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgb::new_u8(0xff, 0x00, 0x00), image.get(0, 0), epsilon = 0.01);
		assert_relative_eq!(Rgb::new_u8(0x00, 0x02, 0xff), image.get(399, 0), epsilon = 0.01);
	}

	#[test]
	fn read_with_convert() {
		let image = picto::read::from_path::<Rgba, u8, _>("tests/rainbow.png").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgba::new_u8(0xff, 0x00, 0x00, 0xff), image.get(0, 0), epsilon = 0.01);
		assert_relative_eq!(Rgba::new_u8(0x00, 0x02, 0xff, 0xff), image.get(399, 0), epsilon = 0.01);
	}

	#[test]
	fn write() {
		{
			let mut image = picto::Buffer::<Rgb, u8, _>::new(2, 2);

			image.set(0, 0, &Rgb::new(1.0, 0.0, 0.0));
			image.set(0, 1, &Rgb::new(0.0, 1.0, 0.0));
			image.set(1, 0, &Rgb::new(0.0, 0.0, 1.0));
			image.set(1, 1, &Rgb::new(1.0, 0.0, 1.0));

			picto::write::to_path("tests/test.png", &image).unwrap();
		}

		{
			let image = picto::read::from_path::<Rgb, u8, _>("tests/test.png").unwrap();

			assert_eq!(2, image.width());
			assert_eq!(2, image.height());

			assert_relative_eq!(Rgb::new(1.0, 0.0, 0.0), image.get(0, 0), epsilon = 0.01);
			assert_relative_eq!(Rgb::new(0.0, 1.0, 0.0), image.get(0, 1), epsilon = 0.01);
			assert_relative_eq!(Rgb::new(0.0, 0.0, 1.0), image.get(1, 0), epsilon = 0.01);
			assert_relative_eq!(Rgb::new(1.0, 0.0, 1.0), image.get(1, 1), epsilon = 0.01);
		}
	}
}

mod jpeg {
	use picto::{self, color::*};

	#[test]
	fn read_as_is() {
		let image = picto::read::from_path::<Rgb, u8, _>("tests/rainbow.jpeg").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgb::new_u8(0xff, 0x00, 0x00), image.get(0, 0), epsilon = 0.01);
		assert_relative_eq!(Rgb::new_u8(0x00, 0x02, 0xff), image.get(399, 0), epsilon = 0.01);
	}

	#[test]
	fn read_with_convert() {
		let image = picto::read::from_path::<Rgba, u8, _>("tests/rainbow.jpeg").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgba::new_u8(0xff, 0x00, 0x00, 0xff), image.get(0, 0), epsilon = 0.01);
		assert_relative_eq!(Rgba::new_u8(0x00, 0x02, 0xff, 0xff), image.get(399, 0), epsilon = 0.01);
	}
}

mod bmp {
	use picto::{self, color::*};

	#[test]
	fn read_as_is() {
		let image = picto::read::from_path::<Rgb, u8, _>("tests/rainbow.bmp").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgb::new_u8(0xff, 0x00, 0x00), image.get(0, 0), epsilon = 0.01);
		assert_relative_eq!(Rgb::new_u8(0x00, 0x02, 0xff), image.get(399, 0), epsilon = 0.01);
	}

	#[test]
	fn read_with_convert() {
		let image = picto::read::from_path::<Rgba, u8, _>("tests/rainbow.bmp").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgba::new_u8(0xff, 0x00, 0x00, 0xff), image.get(0, 0), epsilon = 0.01);
		assert_relative_eq!(Rgba::new_u8(0x00, 0x02, 0xff, 0xff), image.get(399, 0), epsilon = 0.01);
	}

	#[test]
	fn write() {
		{
			let mut image = picto::Buffer::<Rgb, u8, _>::new(2, 2);

			image.set(0, 0, &Rgb::new(1.0, 0.0, 0.0));
			image.set(0, 1, &Rgb::new(0.0, 1.0, 0.0));
			image.set(1, 0, &Rgb::new(0.0, 0.0, 1.0));
			image.set(1, 1, &Rgb::new(1.0, 0.0, 1.0));

			picto::write::to_path("tests/test.bmp", &image).unwrap();
		}

		{
			let image = picto::read::from_path::<Rgb, u8, _>("tests/test.bmp").unwrap();

			assert_eq!(2, image.width());
			assert_eq!(2, image.height());

			assert_relative_eq!(Rgb::new(1.0, 0.0, 0.0), image.get(0, 0), epsilon = 0.01);
			assert_relative_eq!(Rgb::new(0.0, 1.0, 0.0), image.get(0, 1), epsilon = 0.01);
			assert_relative_eq!(Rgb::new(0.0, 0.0, 1.0), image.get(1, 0), epsilon = 0.01);
			assert_relative_eq!(Rgb::new(1.0, 0.0, 1.0), image.get(1, 1), epsilon = 0.01);
		}
	}
}

mod tga {
	use picto::{self, color::*};

	#[test]
	fn read_as_is() {
		let image = picto::read::from_path::<Rgb, u8, _>("tests/rainbow.tga").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgb::new_u8(0xff, 0x00, 0x00), image.get(0, 0), epsilon = 0.01);
		assert_relative_eq!(Rgb::new_u8(0x00, 0x02, 0xff), image.get(399, 0), epsilon = 0.01);
	}

	#[test]
	fn read_with_convert() {
		let image = picto::read::from_path::<Rgba, u8, _>("tests/rainbow.tga").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgba::new_u8(0xff, 0x00, 0x00, 0xff), image.get(0, 0), epsilon = 0.01);
		assert_relative_eq!(Rgba::new_u8(0x00, 0x02, 0xff, 0xff), image.get(399, 0), epsilon = 0.01);
	}

	#[test]
	fn write() {
		{
			let mut image = picto::Buffer::<Rgb, u8, _>::new(2, 2);

			image.set(0, 0, &Rgb::new(1.0, 0.0, 0.0));
			image.set(0, 1, &Rgb::new(0.0, 1.0, 0.0));
			image.set(1, 0, &Rgb::new(0.0, 0.0, 1.0));
			image.set(1, 1, &Rgb::new(1.0, 0.0, 1.0));

			picto::write::to_path("tests/test.tga", &image).unwrap();
		}

		{
			let image = picto::read::from_path::<Rgb, u8, _>("tests/test.tga").unwrap();

			assert_eq!(2, image.width());
			assert_eq!(2, image.height());

			assert_relative_eq!(Rgb::new(1.0, 0.0, 0.0), image.get(0, 0), epsilon = 0.01);
			assert_relative_eq!(Rgb::new(0.0, 1.0, 0.0), image.get(0, 1), epsilon = 0.01);
			assert_relative_eq!(Rgb::new(0.0, 0.0, 1.0), image.get(1, 0), epsilon = 0.01);
			assert_relative_eq!(Rgb::new(1.0, 0.0, 1.0), image.get(1, 1), epsilon = 0.01);
		}
	}
}

mod gif {
	use std::fs::File;

	use picto::{self, color::*, HasParameters};

	#[test]
	fn read_as_is() {
		let image = picto::read::from_path::<Rgb, u8, _>("tests/rainbow.gif").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgb::new_u8(0xff, 0x00, 0x00), image.get(0, 0), epsilon = 0.2);
		assert_relative_eq!(Rgb::new_u8(0x00, 0x02, 0xff), image.get(399, 0), epsilon = 0.2);
	}

	#[test]
	fn read_with_convert() {
		let image = picto::read::from_path::<Rgba, u8, _>("tests/rainbow.gif").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgba::new_u8(0xff, 0x00, 0x00, 0xff), image.get(0, 0), epsilon = 0.2);
		assert_relative_eq!(Rgba::new_u8(0x00, 0x02, 0xff, 0xff), image.get(399, 0), epsilon = 0.2);
	}

	#[test]
	fn write() {
		{
			let mut image = picto::Buffer::<Rgb, u8, _>::new(2, 2);

			image.set(0, 0, &Rgb::new(0.0, 0.0, 0.0));
			image.set(0, 1, &Rgb::new(0.0, 1.0, 0.0));
			image.set(1, 0, &Rgb::new(0.0, 0.0, 1.0));
			image.set(1, 1, &Rgb::new(1.0, 0.0, 1.0));

			picto::write::gif(File::create("tests/test.gif").unwrap(), &image, |gif| {
				gif.set(vec![255u8, 0, 0, 0, 255, 0, 0, 0, 255, 255, 0, 0]).unwrap();
			})
			.unwrap();
		}

		{
			let image = picto::read::from_path::<Rgb, u8, _>("tests/test.gif").unwrap();

			assert_eq!(2, image.width());
			assert_eq!(2, image.height());

			assert_relative_eq!(Rgb::new(0.0, 0.0, 0.0), image.get(0, 0), epsilon = 0.2);
			assert_relative_eq!(Rgb::new(0.0, 1.0, 0.0), image.get(0, 1), epsilon = 0.2);
			assert_relative_eq!(Rgb::new(0.0, 0.0, 1.0), image.get(1, 0), epsilon = 0.2);
			assert_relative_eq!(Rgb::new(1.0, 0.0, 1.0), image.get(1, 1), epsilon = 0.5);
		}
	}
}

mod xyz {
	use picto::{self, color::*};

	#[test]
	fn read_as_is() {
		let image = picto::read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();

		assert_eq!(320, image.width());
		assert_eq!(240, image.height());

		assert_relative_eq!(Rgb::new_u8(0x1e, 0x03, 0x43), image.get(0, 0), epsilon = 0.01);
		assert_relative_eq!(Rgb::new_u8(0x1f, 0x03, 0x45), image.get(0, 239), epsilon = 0.01);
	}

	#[test]
	fn read_with_convert() {
		let image = picto::read::from_path::<Rgba, u8, _>("tests/boat.xyz").unwrap();

		assert_eq!(320, image.width());
		assert_eq!(240, image.height());

		assert_relative_eq!(Rgba::new_u8(0x1e, 0x03, 0x43, 0xff), image.get(0, 0), epsilon = 0.01);
		assert_relative_eq!(Rgba::new_u8(0x1f, 0x03, 0x45, 0xff), image.get(0, 239), epsilon = 0.01);
	}
}
