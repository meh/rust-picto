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

use std::io::{Read, Seek};

use imagefmt::{bmp, ColFmt};
use error::{self, Error};
use buffer::{Buffer, cast};
use pixel;
use color;

pub struct Decoder<R: Read + Seek> {
	inner: R,
}

impl<R: Read + Seek> Decoder<R> {
	#[inline]
	pub fn new(input: R) -> Self {
		Decoder {
			inner: input
		}
	}
}

impl<C, P, R> super::Decoder<C, P> for Decoder<R>
	where C: pixel::Channel,
	      P: pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba>,
	      R: Read + Seek
{
	fn frame(&mut self) -> error::Result<Buffer<C, P, Vec<C>>> {
		let image = try!(bmp::read(self.inner.by_ref(), ColFmt::Auto));

		macro_rules! buffer {
			($ch:ty, $ty:path) => ({
				Ok(cast::Into::<C, P>::into(try!(Buffer::<$ch, $ty, _>::from_raw(
					image.w as u32,
					image.h as u32,
					image.buf).map_err(|_| Error::Format("wrong dimensions".into())))))
			});
		}

		match image.fmt {
			ColFmt::RGB =>
				buffer!(u8, color::Rgb),

			ColFmt::RGBA =>
				buffer!(u8, color::Rgba),

			_ =>
				Err(Error::Unsupported("unsupported color type".into()))
		}
	}
}
