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

use std::{io::Write, ops::Deref};

use gif;

use crate::{
	buffer::Buffer,
	color, error,
	parameter::{HasParameters, Parameter},
	pixel,
};

pub struct Encoder<W: Write> {
	inner: W,
	palette: Vec<u8>,
}

impl<W: Write> Encoder<W> {
	#[inline]
	pub fn new(output: W) -> Self {
		Encoder {
			inner: output,
			palette: vec![],
		}
	}
}

impl<W: Write> Parameter<Encoder<W>> for Vec<u8> {
	#[inline]
	fn set(self, to: &mut Encoder<W>) -> error::Result<()> {
		to.palette = self;

		Ok(())
	}
}

impl<W: Write> HasParameters for Encoder<W> {}

impl<P, C, D, W> super::Encoder<P, C, D> for Encoder<W>
where
	P: pixel::Read<C>,
	P: Into<color::Luma> + Into<color::Lumaa> + Into<color::Rgb> + Into<color::Rgba>,
	C: pixel::Channel,
	D: Deref<Target = [C]>,
	W: Write,
{
	#[inline]
	fn frame(&mut self, buffer: &Buffer<P, C, D>) -> error::Result<()> {
		let mut buffer = buffer.convert::<color::Rgba, u8>();
		let mut encoder = gif::Encoder::new(
			self.inner.by_ref(),
			buffer.width() as u16,
			buffer.height() as u16,
			&self.palette,
		)?;

		encoder.write_frame(&gif::Frame::from_rgba(
			buffer.width() as u16,
			buffer.height() as u16,
			&mut buffer,
		))?;

		Ok(())
	}
}
