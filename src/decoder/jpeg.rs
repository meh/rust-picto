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
use crate::error::{self, Error};
use crate::buffer::{Buffer, cast};
use crate::pixel;
use crate::color;

pub struct Decoder<R: Read> {
	inner:    jpeg::Decoder<R>,
	metadata: Option<jpeg::ImageInfo>,
}

impl<R: Read> Decoder<R> {
	#[inline]
	pub fn new(input: R) -> Self {
		Decoder {
			inner:    jpeg::Decoder::new(input),
			metadata: None,
		}
	}

	#[inline]
	pub fn metadata(&mut self) -> error::Result<jpeg::ImageInfo> {
		if self.metadata.is_none() {
			r#try!(self.inner.read_info());
			self.metadata = Some(self.inner.info().unwrap());
		}

		Ok(self.metadata.unwrap())
	}
}

impl<P, C, R> super::Decoder<P, C> for Decoder<R>
	where P: pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Luma>,
	      C: pixel::Channel,
	      R: Read
{
	fn frame(&mut self) -> error::Result<Buffer<P, C, Vec<C>>> {
		#[inline]
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

		let mut buffer = r#try!(self.inner.decode());

		macro_rules! buffer {
			($ch:ty, $ty:path) => ({
				Ok(cast::Into::<P, C>::into(r#try!(Buffer::<$ty, $ch, _>::from_raw(
					r#try!(self.metadata()).width as u32,
					r#try!(self.metadata()).height as u32,
					buffer).map_err(|_| Error::Format("wrong dimensions".into())))))
			});
		}

		match r#try!(self.metadata()).pixel_format {
			jpeg::PixelFormat::L8 =>
				buffer!(u8, color::Luma),

			jpeg::PixelFormat::RGB24 =>
				buffer!(u8, color::Rgb),

			jpeg::PixelFormat::CMYK32 => {
				convert(&mut buffer);
				buffer!(u8, color::Rgb)
			}
		}
	}
}
