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

use imagefmt::{tga, ColFmt};
use crate::error::{self, Error};
use crate::buffer::{Buffer, cast};
use crate::pixel;
use crate::color;

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

impl<P, C, R> super::Decoder<P, C> for Decoder<R>
	where P: pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      C: pixel::Channel,
	      R: Read + Seek
{
	fn frame(&mut self) -> error::Result<Buffer<P, C, Vec<C>>> {
		let image = r#try!(tga::read(self.inner.by_ref(), ColFmt::Auto));

		macro_rules! buffer {
			($ch:ty, $ty:path) => ({
				Ok(cast::Into::<P, C>::into(r#try!(Buffer::<$ty, $ch, _>::from_raw(
					image.w as u32,
					image.h as u32,
					image.buf).map_err(|_| Error::Format("wrong dimensions".into())))))
			});
		}

		match image.fmt {
			ColFmt::Y =>
				buffer!(u8, color::Luma),

			ColFmt::YA =>
				buffer!(u8, color::Lumaa),

			ColFmt::RGB =>
				buffer!(u8, color::Rgb),

			ColFmt::RGBA =>
				buffer!(u8, color::Rgba),

			_ =>
				Err(Error::Unsupported("unsupported color type".into()))
		}
	}
}
