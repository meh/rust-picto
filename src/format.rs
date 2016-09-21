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
}

/// Guess the image format.
pub fn guess<R: Read + Seek>(mut input: R) -> Option<Format> {
	const MAGIC: &'static [(&'static [u8], Format)] = &[
		(b"\x89PNG\r\n\x1a\n", Format::Png),
		(&[0xff, 0xd8, 0xff],  Format::Jpeg),
		(b"GIF89a",            Format::Gif),
		(b"GIF87a",            Format::Gif),
		(b"WEBP",              Format::Webp),
		(b"MM.*",              Format::Tiff),
		(b"II*.",              Format::Tiff),
		(b"BM",                Format::Bmp),
		(&[0, 0, 1, 0],        Format::Ico),
		(b"#?RADIANCE",        Format::Hdr),
	];

	macro_rules! try {
		(return $body:expr) => (
			if let Ok(value) = $body {
				value
			}
			else {
				return None;
			}
		);

		(continue $body:expr) => (
			if let Ok(value) = $body {
				value
			}
			else {
				continue;
			}
		);
	}

	for &(magic, format) in MAGIC.iter() {
		try!(continue input.seek(SeekFrom::Start(0)));

		let mut buffer = vec![0; magic.len()];
		try!(continue input.read_exact(&mut buffer));

		if buffer == &magic[..] {
			try!(return input.seek(SeekFrom::Start(0)));
			return Some(format);
		}
	}

	try!(return input.seek(SeekFrom::Start(0)));

	None
}
