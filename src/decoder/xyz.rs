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

use xyz;
use error::{self, Error};
use buffer::Buffer;
use pixel::{self, Pixel};
use color;

pub struct Decoder<R: Read> {
	inner: R,
}

impl<R: Read> Decoder<R> {
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
	      R: Read
{
	#[inline]
	fn frame(&mut self) -> error::Result<Buffer<C, P, Vec<C>>> {
		let image = try!(xyz::read(self.inner.by_ref()));

		Ok(Cast::<C, P>::cast(try!(Buffer::<u8, color::Rgb, _>::from_raw(
			image.width as u32, image.height as u32,
			image.to_rgb_buffer())
				.map_err(|_| Error::Format("wrong dimensions".into())))))
	}
}

cast! {
	(u8, Rgb)
}
