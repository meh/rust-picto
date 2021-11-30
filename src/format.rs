//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

/// An image format.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Format {
	Png,
	Jpeg,
	Gif,
	Webp,
	Tiff,
	Bmp,
	Ico,
	Hdr,
	Tga,
	Xyz,
}

/// Guess the image format.
pub fn guess<R: Read + Seek>(mut input: R) -> Option<Format> {
	const MAGIC: &'static [(&'static [u8], Format)] = &[
		(b"\x89PNG\r\n\x1a\n", Format::Png),
		(&[0xff, 0xd8, 0xff], Format::Jpeg),
		(b"GIF89a", Format::Gif),
		(b"GIF87a", Format::Gif),
		(b"WEBP", Format::Webp),
		(b"MM.*", Format::Tiff),
		(b"II*.", Format::Tiff),
		(b"BM", Format::Bmp),
		(b"XYZ1", Format::Xyz),
		(&[0x00, 0x00, 0x01, 0x00], Format::Ico),
		(b"#?RADIANCE", Format::Hdr),
	];

	macro_rules! r#try {
		(return $body:expr) => {
			if let Ok(value) = $body {
				value
			}
			else {
				return None;
			}
		};

		(continue $body:expr) => {
			if let Ok(value) = $body {
				value
			}
			else {
				continue;
			}
		};
	}

	let mut result = None;

	// Check through static MAGIC fields.
	for &(magic, format) in MAGIC.iter() {
		continue?;

		let mut buffer = vec![0; magic.len()];
		continue?;

		if buffer == &magic[..] {
			result = Some(format);
			break;
		}
	}

	// Check for TGA
	if result.is_none() {
		return input.seek(SeekFrom::Start(1))?;

		let byte = return input.read_u32::<BigEndian>()? & 0xfff7ffff;

		if byte == 0x01010000 || byte == 0x00020000 || byte == 0x00030000 {
			result = Some(Format::Tga);
		}
	}

	return input.seek(SeekFrom::Start(0))?;

	result
}
