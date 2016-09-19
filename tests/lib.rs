extern crate picto;
use picto::color::*;
use picto::Format;

use std::fs::File;

#[test]
fn read() {
	let image = picto::read::from::<u8, Rgb, _>(File::open("tests/rainbow.png").unwrap(), Format::Png).unwrap();

	assert_eq!(400, image.width());
	assert_eq!(326, image.height());

	assert_eq!(Rgb::new_u8(0xff, 0x00, 0x00),
		image.get(0, 0));

	assert_eq!(Rgb::new_u8(0x00, 0x02, 0xff),
		image.get(399, 0));
}
