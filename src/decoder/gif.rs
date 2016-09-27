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

use gif::{self, SetParameter};
use error::{self, Error};
use buffer::Buffer;
use pixel::{self, Pixel};
use color;

enum State<R: Read> {
	Decoder(gif::Decoder<R>),
	Reader(gif::Reader<R>),
}

pub struct Decoder<R: Read> {
	state: Option<State<R>>,
}

impl<R: Read> Decoder<R> {
	#[inline]
	pub fn new(input: R) -> Self {
		let mut decoder = gif::Decoder::new(input);
		decoder.set(gif::ColorOutput::RGBA);

		Decoder {
			state: Some(State::Decoder(decoder)),
		}
	}

	pub fn reader(&mut self) -> error::Result<&mut gif::Reader<R>> {
		let inner = self.state.take();

		match inner {
			Some(State::Decoder(decoder)) => {
				self.state = Some(State::Reader(try!(decoder.read_info())));
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
	#[inline]
	fn frame(&mut self) -> error::Result<Buffer<C, P, Vec<C>>> {
		let frame = try!(try!(try!(self.reader()).read_next_frame()).ok_or(Error::Format("no frames".into())));

		Ok(Cast::<C, P>::cast(try!(Buffer::<u8, color::Rgba, _>::from_raw(
			frame.width as u32, frame.height as u32,
			frame.buffer.clone().into_owned())
				.map_err(|_| Error::Format("wrong dimensions".into())))))
	}
}

cast! {
	(u8, Rgba)
}
