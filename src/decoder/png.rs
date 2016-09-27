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
use error::{self, Error};
use buffer::{Buffer, cast};
use pixel;
use color;

enum State<R: Read> {
	Decoder(png::Decoder<R>),
	Reader(png::Reader<R>),
}

pub struct Decoder<R: Read> {
	state: Option<State<R>>,
}

impl<R: Read> Decoder<R> {
	#[inline]
	pub fn new(input: R) -> Self {
		Decoder {
			state: Some(State::Decoder(png::Decoder::new(input))),
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

impl<C, P, R> super::Decoder<C, P> for Decoder<R>
	where C: pixel::Channel,
	      P: pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      R: Read
{
	fn frame(&mut self) -> error::Result<Buffer<C, P, Vec<C>>> {
		let mut buffer = vec![0; try!(self.reader()).output_buffer_size()];
		try!(try!(self.reader()).next_frame(&mut buffer));

		macro_rules! buffer {
			($ch:ty, $ty:path) => ({
				Ok(cast::Into::<C, P>::into(try!(Buffer::<_, $ty, _>::from_raw(
					try!(self.reader()).info().size().0,
					try!(self.reader()).info().size().1,
					buffer).map_err(|_| Error::Format("wrong dimensions".into())))))
			});
		}

		match try!(self.reader()).output_color_type() {
			(png::ColorType::Grayscale, png::BitDepth::Eight) =>
				buffer!(u8, color::Luma),

			(png::ColorType::GrayscaleAlpha, png::BitDepth::Eight) =>
				buffer!(u8, color::Lumaa),

			(png::ColorType::RGB, png::BitDepth::Eight) =>
				buffer!(u8, color::Rgb),

			(png::ColorType::RGBA, png::BitDepth::Eight) =>
				buffer!(u8, color::Rgba),

			(png::ColorType::Grayscale, png::BitDepth::Sixteen) =>
				buffer!(u16, color::Luma),

			(png::ColorType::GrayscaleAlpha, png::BitDepth::Sixteen) =>
				buffer!(u16, color::Lumaa),

			(png::ColorType::RGB, png::BitDepth::Sixteen) =>
				buffer!(u16, color::Rgb),

			(png::ColorType::RGBA, png::BitDepth::Sixteen) =>
				buffer!(u16, color::Rgba),

			_ =>
				Err(Error::Unsupported("unsupported color type".into()))
		}
	}
}
