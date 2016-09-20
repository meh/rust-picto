#[macro_use]
extern crate approx;
extern crate picto;

mod png {
	use picto;
	use picto::color::*;
	use std::fs::File;

	#[test]
	fn read_as_is() {
		let image = picto::read::from::<u8, Rgb, _>(File::open("tests/rainbow.png").unwrap()).unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgb::new_u8(0xff, 0x00, 0x00),
			image.get(0, 0), max_relative = 0.01);

		assert_relative_eq!(Rgb::new_u8(0x00, 0x02, 0xff),
			image.get(399, 0), max_relative = 0.01);
	}

	#[test]
	fn read_with_convert() {
		let image = picto::read::from::<u8, Rgba, _>(File::open("tests/rainbow.png").unwrap()).unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgba::new_u8(0xff, 0x00, 0x00, 0xff),
			image.get(0, 0), max_relative = 0.01);

		assert_relative_eq!(Rgba::new_u8(0x00, 0x02, 0xff, 0xff),
			image.get(399, 0), max_relative = 0.01);
	}
}

mod jpeg {
	use picto;
	use picto::color::*;
	use std::fs::File;

	#[test]
	fn read_as_is() {
		let image = picto::read::from::<u8, Rgb, _>(File::open("tests/rainbow.jpeg").unwrap()).unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgb::new_u8(0xff, 0x00, 0x00),
			image.get(0, 0), max_relative = 0.01);

		assert_relative_eq!(Rgb::new_u8(0x00, 0x02, 0xff),
			image.get(399, 0), max_relative = 0.01);
	}

	#[test]
	fn read_with_convert() {
		let image = picto::read::from::<u8, Rgba, _>(File::open("tests/rainbow.jpeg").unwrap()).unwrap();

		assert_eq!(400, image.width());
		assert_eq!(326, image.height());

		assert_relative_eq!(Rgba::new_u8(0xff, 0x00, 0x00, 0xff),
			image.get(0, 0), max_relative = 0.01);

		assert_relative_eq!(Rgba::new_u8(0x00, 0x02, 0xff, 0xff),
			image.get(399, 0), max_relative = 0.01);
	}
}
