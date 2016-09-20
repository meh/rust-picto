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

use std::io::Read;

use jpeg_decoder as jpeg;
use format::{Format, Color};
use super::{error, Error};

pub struct Decoder<R: Read> {
	inner:    jpeg::Decoder<R>,
	metadata: Option<jpeg::ImageInfo>,
}

impl<R: Read> Decoder<R> {
	pub fn new(input: R) -> Self {
		Decoder {
			inner:    jpeg::Decoder::new(input),
			metadata: None,
		}
	}

	pub fn metadata(&mut self) -> error::Result<jpeg::ImageInfo> {
		if self.metadata.is_none() {
			try!(self.inner.read_info());
			self.metadata = Some(self.inner.info().unwrap());
		}

		Ok(self.metadata.unwrap())
	}
}

impl<R: Read> super::Decoder for Decoder<R> {
	fn format(&mut self) -> error::Result<Format> {
		Ok(Format::Jpeg)
	}

	fn dimensions(&mut self) -> error::Result<(u32, u32)> {
		Ok((try!(self.metadata()).width as u32, try!(self.metadata()).height as u32))
	}

	fn color(&mut self) -> error::Result<Color> {
		Ok(try!(self.metadata()).pixel_format.into())
	}

	fn frame(&mut self) -> error::Result<Vec<u8>> {
		fn convert(buffer: &mut Vec<u8>) {
			let     length = buffer.len();
			let mut cmyk   = 0;
			let mut rgb    = 0;

			while cmyk < length {
				let c = buffer[cmyk    ] as f32 / 255.0;
				let m = buffer[cmyk + 1] as f32 / 255.0;
				let y = buffer[cmyk + 2] as f32 / 255.0;
				let k = buffer[cmyk + 3] as f32 / 255.0;

				// CMYK -> CMY
				let c = c * (1.0 - k) + k;
				let m = m * (1.0 - k) + k;
				let y = y * (1.0 - k) + k;

				// CMY -> RGB
				let r = (1.0 - c) * 255.0;
				let g = (1.0 - m) * 255.0;
				let b = (1.0 - y) * 255.0;

				buffer[rgb    ] = r as u8;
				buffer[rgb + 1] = g as u8;
				buffer[rgb + 2] = b as u8;

				cmyk += 4;
				rgb  += 3;
			}

			buffer.resize((length / 4 ) * 3, 0);
			buffer.shrink_to_fit();
		}

		let mut frame = try!(self.inner.decode());

		if try!(self.metadata()).pixel_format == jpeg::PixelFormat::CMYK32 {
			convert(&mut frame);
		}

		Ok(frame)
	}
}

impl From<jpeg::PixelFormat> for Color {
	fn from(value: jpeg::PixelFormat) -> Self {
		match value {
			jpeg::PixelFormat::L8 =>
				Color::Gray(8, false),

			jpeg::PixelFormat::RGB24 =>
				Color::Rgb(8, false),

			jpeg::PixelFormat::CMYK32 =>
				Color::Rgb(8, false),
		}
	}
}

impl From<jpeg::Error> for Error {
	fn from(value: jpeg::Error) -> Self {
		match value {
			jpeg::Error::Format(desc)      => Error::Format(desc),
			jpeg::Error::Unsupported(desc) => Error::Unsupported(format!("{:?}", desc)),
			jpeg::Error::Io(err)           => Error::Io(err),
			jpeg::Error::Internal(err)     => Error::Format(err.description().to_owned()),
		}
	}
}
