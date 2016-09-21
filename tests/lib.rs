#[macro_use]
extern crate approx;
extern crate picto;

mod png {
	use picto;
	use picto::color::*;

	#[test]
	fn read_as_is() {
		let image = picto::read::from_path::<u8, Rgb, _>("tests/rainbow.png").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgb::new_u8(0xff, 0x00, 0x00),
			image.get(0, 0), max_relative = 0.01);

		assert_relative_eq!(Rgb::new_u8(0x00, 0x02, 0xff),
			image.get(399, 0), max_relative = 0.01);
	}

	#[test]
	fn read_with_convert() {
		let image = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.png").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgba::new_u8(0xff, 0x00, 0x00, 0xff),
			image.get(0, 0), max_relative = 0.01);

		assert_relative_eq!(Rgba::new_u8(0x00, 0x02, 0xff, 0xff),
			image.get(399, 0), max_relative = 0.01);
	}

	#[test]
	fn write() {
		{
			let mut image = picto::Buffer::<u8, Rgb, _>::new(2, 2);

			image.set(0, 0, &Rgb::new(1.0, 0.0, 0.0));
			image.set(0, 1, &Rgb::new(0.0, 1.0, 0.0));
			image.set(1, 0, &Rgb::new(0.0, 0.0, 1.0));
			image.set(1, 1, &Rgb::new(1.0, 0.0, 1.0));

			picto::write::to_path(&image, "tests/test.png").unwrap();
		}

		{
			let image = picto::read::from_path::<u8, Rgb, _>("tests/test.png").unwrap();

			assert_eq!(2, image.width());
			assert_eq!(2, image.height());

			assert_relative_eq!(Rgb::new(1.0, 0.0, 0.0),
				image.get(0, 0), max_relative = 0.01);

			assert_relative_eq!(Rgb::new(0.0, 1.0, 0.0),
				image.get(0, 1), max_relative = 0.01);

			assert_relative_eq!(Rgb::new(0.0, 0.0, 1.0),
				image.get(1, 0), max_relative = 0.01);

			assert_relative_eq!(Rgb::new(1.0, 0.0, 1.0),
				image.get(1, 1), max_relative = 0.01);
		}
	}
}

mod jpeg {
	use picto;
	use picto::color::*;

	#[test]
	fn read_as_is() {
		let image = picto::read::from_path::<u8, Rgb, _>("tests/rainbow.jpeg").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgb::new_u8(0xff, 0x00, 0x00),
			image.get(0, 0), max_relative = 0.01);

		assert_relative_eq!(Rgb::new_u8(0x00, 0x02, 0xff),
			image.get(399, 0), max_relative = 0.01);
	}

	#[test]
	fn read_with_convert() {
		let image = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.jpeg").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgba::new_u8(0xff, 0x00, 0x00, 0xff),
			image.get(0, 0), max_relative = 0.01);

		assert_relative_eq!(Rgba::new_u8(0x00, 0x02, 0xff, 0xff),
			image.get(399, 0), max_relative = 0.01);
	}
}

mod bmp {
	use picto;
	use picto::color::*;

	#[test]
	fn read_as_is() {
		let image = picto::read::from_path::<u8, Rgb, _>("tests/rainbow.bmp").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgb::new_u8(0xff, 0x00, 0x00),
			image.get(0, 0), max_relative = 0.01);

		assert_relative_eq!(Rgb::new_u8(0x00, 0x02, 0xff),
			image.get(399, 0), max_relative = 0.01);
	}

	#[test]
	fn read_with_convert() {
		let image = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.bmp").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgba::new_u8(0xff, 0x00, 0x00, 0xff),
			image.get(0, 0), max_relative = 0.01);

		assert_relative_eq!(Rgba::new_u8(0x00, 0x02, 0xff, 0xff),
			image.get(399, 0), max_relative = 0.01);
	}

	#[test]
	fn write() {
		{
			let mut image = picto::Buffer::<u8, Rgb, _>::new(2, 2);

			image.set(0, 0, &Rgb::new(1.0, 0.0, 0.0));
			image.set(0, 1, &Rgb::new(0.0, 1.0, 0.0));
			image.set(1, 0, &Rgb::new(0.0, 0.0, 1.0));
			image.set(1, 1, &Rgb::new(1.0, 0.0, 1.0));

			picto::write::to_path(&image, "tests/test.bmp").unwrap();
		}

		{
			let image = picto::read::from_path::<u8, Rgb, _>("tests/test.bmp").unwrap();

			assert_eq!(2, image.width());
			assert_eq!(2, image.height());

			assert_relative_eq!(Rgb::new(1.0, 0.0, 0.0),
				image.get(0, 0), max_relative = 0.01);

			assert_relative_eq!(Rgb::new(0.0, 1.0, 0.0),
				image.get(0, 1), max_relative = 0.01);

			assert_relative_eq!(Rgb::new(0.0, 0.0, 1.0),
				image.get(1, 0), max_relative = 0.01);

			assert_relative_eq!(Rgb::new(1.0, 0.0, 1.0),
				image.get(1, 1), max_relative = 0.01);
		}
	}
}

mod tga {
	use picto;
	use picto::color::*;

	#[test]
	fn read_as_is() {
		let image = picto::read::from_path::<u8, Rgb, _>("tests/rainbow.tga").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgb::new_u8(0xff, 0x00, 0x00),
			image.get(0, 0), max_relative = 0.01);

		assert_relative_eq!(Rgb::new_u8(0x00, 0x02, 0xff),
			image.get(399, 0), max_relative = 0.01);
	}

	#[test]
	fn read_with_convert() {
		let image = picto::read::from_path::<u8, Rgba, _>("tests/rainbow.tga").unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgba::new_u8(0xff, 0x00, 0x00, 0xff),
			image.get(0, 0), max_relative = 0.01);

		assert_relative_eq!(Rgba::new_u8(0x00, 0x02, 0xff, 0xff),
			image.get(399, 0), max_relative = 0.01);
	}

	#[test]
	fn write() {
		{
			let mut image = picto::Buffer::<u8, Rgb, _>::new(2, 2);

			image.set(0, 0, &Rgb::new(1.0, 0.0, 0.0));
			image.set(0, 1, &Rgb::new(0.0, 1.0, 0.0));
			image.set(1, 0, &Rgb::new(0.0, 0.0, 1.0));
			image.set(1, 1, &Rgb::new(1.0, 0.0, 1.0));

			picto::write::to_path(&image, "tests/test.tga").unwrap();
		}

		{
			let image = picto::read::from_path::<u8, Rgb, _>("tests/test.tga").unwrap();

			assert_eq!(2, image.width());
			assert_eq!(2, image.height());

			assert_relative_eq!(Rgb::new(1.0, 0.0, 0.0),
				image.get(0, 0), max_relative = 0.01);

			assert_relative_eq!(Rgb::new(0.0, 1.0, 0.0),
				image.get(0, 1), max_relative = 0.01);

			assert_relative_eq!(Rgb::new(0.0, 0.0, 1.0),
				image.get(1, 0), max_relative = 0.01);

			assert_relative_eq!(Rgb::new(1.0, 0.0, 1.0),
				image.get(1, 1), max_relative = 0.01);
		}
	}
}
