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

use imagefmt::{tga, ColFmt, ColType};
use error;
use pixel::{self, Pixel};
use buffer::Buffer;
use color;

pub struct Encoder<W: Write> {
	inner: W,
}

impl<W: Write> Encoder<W> {
	#[inline]
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
		let format = buffer.color().unwrap_or(ColFmt::RGB);

		macro_rules! write {
			($ch:ty, $ty:path) => (
				try!(tga::write(self.inner.by_ref(), buffer.width() as usize, buffer.height() as usize,
					format, Cast::<$ty>::cast(buffer).as_ref(), ColType::Auto, None))
			);
		}

		match format {
			ColFmt::Y =>
				write!(u8, color::Luma),

			ColFmt::YA =>
				write!(u8, color::Lumaa),

			ColFmt::RGB =>
				write!(u8, color::Rgb),

			ColFmt::RGBA =>
				write!(u8, color::Rgba),

			_ => unreachable!()
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
	use buffer::Buffer;
	use pixel::{self, Pixel};
	use super::Color;

	impl<C, P, D> Color for Buffer<C, P, D>
		where C: pixel::Channel,
	        P: Pixel<C>
	{
		#[inline]
		fn color(&self) -> Option<ColFmt> {
			None
		}
	}
}

#[cfg(feature = "nightly")]
mod nightly {
	use imagefmt::ColFmt;
	use num::Float;
	use buffer::Buffer;
	use pixel::{self, Pixel};
	use color::{Luma, Lumaa, Rgb, Rgba};
	use super::Color;

	impl<C, P, D> Color for Buffer<C, P, D>
		where C: pixel::Channel,
	        P: Pixel<C>
	{
		#[inline]
		default
		fn color(&self) -> Option<ColFmt> {
			None
		}
	}

	macro_rules! impl_for {
		($ch:ident, $px:ident => $fmt:path) => (
			impl<D, T: Float + 'static> Color for Buffer<$ch, $px<T>, D> {
				#[inline]
				fn color(&self) -> Option<ColFmt> {
					Some($fmt)
				}
			}
		)
	}

	impl_for!(u8, Luma => ColFmt::Y);
	impl_for!(u8, Lumaa => ColFmt::YA);
	impl_for!(u8, Rgb => ColFmt::RGB);
	impl_for!(u8, Rgba => ColFmt::RGBA);
}

cast! {
	(u8,  Luma), (u8,  Lumaa), (u8,  Rgb), (u8,  Rgba)
}
