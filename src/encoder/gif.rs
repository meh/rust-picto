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

use std::io::Write;
use std::ops::Deref;

use gif;
use error;
use pixel::{self, Pixel};
use buffer::Buffer;
use color;

pub struct Encoder<W: Write> {
	inner: W,
}

impl<W: Write> Encoder<W> {
	pub fn new(output: W) -> Self {
		Encoder {
			inner: output,
		}
	}
}

impl<C, P, D, W> super::Encoder<C, P, D> for Encoder<W>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C>,
	      P: Into<color::Luma> + Into<color::Lumaa> + Into<color::Rgb> + Into<color::Rgba>,
	      D: Deref<Target = [C]>,
	      W: Write
{
	fn frame(&mut self, buffer: &Buffer<C, P, D>) -> error::Result<()> {
		let mut buffer  = buffer.convert::<u8, color::Rgba>();
		let mut encoder = try!(gif::Encoder::new(self.inner.by_ref(),
			buffer.width() as u16, buffer.height() as u16, &[]));

		try!(encoder.write_frame(&gif::Frame::from_rgba(
			buffer.width() as u16, buffer.height() as u16, &mut buffer)));

		Ok(())
	}
}
