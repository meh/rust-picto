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

use std::io::{Read, Cursor};

use decoder;
use color;
use pixel::{self, Pixel};
use buffer::Buffer;
use format::{self, Format};

pub fn from_memory<C, P>(slice: &[u8]) -> decoder::Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>
{
	from::<C, P, _>(Cursor::new(slice),
		try!(format::guess(slice).ok_or(decoder::Error::Format("unsupported image format".into()))))
}

pub fn from<C, P, R>(input: R, format: Format) -> decoder::Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      R: Read
{
	match format {
		#[cfg(feature = "png")]
		Format::Png => {
			decoder::load::<C, P, _>(decoder::png::Decoder::new(input))
		}

		_ =>
			Err(decoder::Error::Format("unsupported image format".into())),
	}
}
