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
use error::{self, Error};
use format::Format;
use buffer::Buffer;
use pixel::{self, Pixel};
use color;

pub struct Decoder<R: Read + Seek> {
	inner: R,
}

impl<R: Read + Seek> Decoder<R> {
	pub fn new(input: R) -> Self {
		Decoder {
			inner: input
		}
	}
}

impl<R: Read + Seek> super::Parameter<Decoder<R>> for Format {
	fn get(_from: &mut Decoder<R>) -> error::Result<Self> {
		Ok(Format::Tga)
	}
}

impl<C, P, R> super::Decoder<C, P> for Decoder<R>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      R: Read + Seek
{
	fn frame(&mut self) -> error::Result<Buffer<C, P, Vec<C>>> {
		let image = try!(tga::read(self.inner.by_ref(), ColFmt::Auto));

		macro_rules! buffer {
			($ch:ty, $ty:path) => ({
				Ok(Cast::<C, P>::cast(try!(Buffer::<$ch, $ty, _>::from_raw(
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

cast! {
	(u8, Luma), (u8, Lumaa), (u8, Rgb), (u8, Rgba)
}
