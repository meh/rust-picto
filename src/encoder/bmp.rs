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

use imagefmt::{bmp, ColFmt, ColType};

use crate::{
	buffer::{cast, Buffer},
	color, error, pixel,
};

pub struct Encoder<W: Write> {
	inner: W,
}

impl<W: Write> Encoder<W> {
	#[inline]
	pub fn new(output: W) -> Self {
		Encoder { inner: output }
	}
}

impl<P, C, D, W> super::Encoder<P, C, D> for Encoder<W>
where
	P: pixel::Read<C>,
	P: Into<color::Rgb> + Into<color::Rgba>,
	C: pixel::Channel,
	D: Deref<Target = [C]>,
	W: Write,
{
	fn frame(&mut self, buffer: &Buffer<P, C, D>) -> error::Result<()> {
		let format = buffer.color().unwrap_or(ColFmt::RGB);

		macro_rules! write {
			($ch:ty, $ty:path) => {
				bmp::write(
					self.inner.by_ref(),
					buffer.width() as usize,
					buffer.height() as usize,
					format,
					cast::Bytes::<$ty, $ch>::bytes(buffer).as_ref(),
					ColType::Auto,
					None,
				)?
			};
		}

		match format {
			ColFmt::RGB => write!(u8, color::Rgb),

			ColFmt::RGBA => write!(u8, color::Rgba),

			_ => unreachable!(),
		}

		Ok(())
	}
}

trait Color {
	fn color(&self) -> Option<ColFmt>;
}

#[cfg(not(feature = "nightly"))]
mod stable {
	use imagefmt::ColFmt;

	use super::Color;
	use crate::{
		buffer::Buffer,
		pixel::{self, Pixel},
	};

	impl<P, C, D> Color for Buffer<P, C, D>
	where
		P: Pixel<C>,
		C: pixel::Channel,
	{
		#[inline]
		fn color(&self) -> Option<ColFmt> {
			None
		}
	}
}

#[cfg(feature = "nightly")]
mod nightly {
	use buffer::Buffer;
	use color::{Rgb, Rgba};
	use imagefmt::ColFmt;
	use num::Float;
	use pixel::{self, Pixel};

	use super::Color;

	impl<P, C, D> Color for Buffer<P, C, D>
	where
		P: Pixel<C>,
		C: pixel::Channel,
	{
		#[inline]
		default fn color(&self) -> Option<ColFmt> {
			None
		}
	}

	macro_rules! impl_for {
		($ch:ident, $px:ident => $fmt:path) => {
			impl<D, T: Float + 'static> Color for Buffer<$px<T>, $ch, D> {
				#[inline]
				fn color(&self) -> Option<ColFmt> {
					Some($fmt)
				}
			}
		};
	}

	impl_for!(u8, Rgb => ColFmt::RGB);
	impl_for!(u8, Rgba => ColFmt::RGBA);
}
