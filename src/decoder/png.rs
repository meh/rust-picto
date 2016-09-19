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

use png;
use format::{Format, Color};
use super::{error, Error};

enum State<R: Read> {
	Decoder(png::Decoder<R>),
	Reader(png::Reader<R>),
}

pub struct Decoder<R: Read> {
	state: Option<State<R>>,

	dimensions: Option<(u32, u32)>,
	color:      Option<Color>,
}

impl<R: Read> Decoder<R> {
	pub fn new(input: R) -> Self {
		Decoder {
			state: Some(State::Decoder(png::Decoder::new(input))),

			dimensions: None,
			color:      None,
		}
	}

	pub fn reader(&mut self) -> error::Result<&mut png::Reader<R>> {
		let inner = self.state.take();

		match inner {
			Some(State::Decoder(decoder)) => {
				let (_, reader) = try!(decoder.read_info());
				self.state = Some(State::Reader(reader));
			}

			Some(State::Reader(reader)) => {
				self.state = Some(State::Reader(reader));
			}

			None => {
				unreachable!()
			}
		}

		if let Some(&mut State::Reader(ref mut reader)) = self.state.as_mut() {
			Ok(reader)
		}
		else {
			unreachable!();
		}
	}
}

impl<R: Read> super::Decoder for Decoder<R> {
	fn format(&mut self) -> error::Result<Format> {
		Ok(Format::Png)
	}

	fn dimensions(&mut self) -> error::Result<(u32, u32)> {
		if let Some(dimensions) = self.dimensions {
			return Ok(dimensions)
		}

		Ok(try!(self.reader()).info().size())
	}

	fn color(&mut self) -> error::Result<Color> {
		if let Some(color) = self.color {
			return Ok(color);
		}

		Ok(try!(self.reader()).output_color_type().into())
	}

	fn frame(&mut self) -> error::Result<Vec<u8>> {
		let mut buffer = vec![0; try!(self.reader()).output_buffer_size()];
		try!(try!(self.reader()).next_frame(&mut buffer));

		Ok(buffer)
	}
}

impl From<(png::ColorType, png::BitDepth)> for Color {
	fn from((kind, depth): (png::ColorType, png::BitDepth)) -> Self {
		let bits = depth as u8;

		match kind {
			png::ColorType::Grayscale =>
				Color::Gray(bits, false),

			png::ColorType::RGB =>
				Color::Rgb(bits, false),

			png::ColorType::Indexed =>
				Color::Palette(bits),

			png::ColorType::GrayscaleAlpha =>
				Color::Gray(bits, true),

			png::ColorType::RGBA =>
				Color::Rgb(bits, true),
		}
	}
}

impl From<png::DecodingError> for Error {
	fn from(value: png::DecodingError) -> Self {
		match value {
			png::DecodingError::IoError(err) =>
				Error::Io(err),

			png::DecodingError::Format(desc) =>
				Error::Format(desc.into_owned()),

			png::DecodingError::InvalidSignature =>
				Error::Format("invalid signature".into()),

			png::DecodingError::CrcMismatch { .. } =>
				Error::Format("CRC error".into()),

			png::DecodingError::Other(desc) =>
				Error::Format(desc.into_owned()),

			png::DecodingError::CorruptFlateStream =>
				Error::Format("compressed data stream corrupted".into())
		}
	}
}
