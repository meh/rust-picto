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

use png::{self, HasParameters};
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
	      P: pixel::Read<C>,
	      P: Into<color::Luma> + Into<color::Lumaa> + Into<color::Rgb> + Into<color::Rgba>,
	      D: Deref<Target = [C]>,
	      W: Write
{
	fn frame(&mut self, buffer: &Buffer<C, P, D>) -> error::Result<()> {
		let (color, depth) = buffer.color().unwrap_or((png::ColorType::RGBA, png::BitDepth::Eight));

		let mut encoder = png::Encoder::new(self.inner.by_ref(), buffer.width(), buffer.height());
		encoder.set(color).set(depth);

		let mut writer = try!(encoder.write_header());

		macro_rules! write {
			($ch:ty, $ty:path) => (
				try!(writer.write_image_data(Cast::<$ty>::cast(buffer).as_ref()))
			);
		}

		match (color, depth) {
			(png::ColorType::Grayscale, png::BitDepth::Eight) =>
				write!(u8, color::Luma),

			(png::ColorType::GrayscaleAlpha, png::BitDepth::Eight) =>
				write!(u8, color::Lumaa),

			(png::ColorType::RGB, png::BitDepth::Eight) =>
				write!(u8, color::Rgb),

			(png::ColorType::RGBA, png::BitDepth::Eight) =>
				write!(u8, color::Rgba),

			(png::ColorType::Grayscale, png::BitDepth::Sixteen) =>
				write!(u16, color::Luma),

			(png::ColorType::GrayscaleAlpha, png::BitDepth::Sixteen) =>
				write!(u16, color::Lumaa),

			(png::ColorType::RGB, png::BitDepth::Sixteen) =>
				write!(u16, color::Rgb),

			(png::ColorType::RGBA, png::BitDepth::Sixteen) =>
				write!(u16, color::Rgba),

			_ => unreachable!()
		}

		Ok(())
	}
}

trait Color {
	fn color(&self) -> Option<(png::ColorType, png::BitDepth)>;
}

#[cfg(not(feature = "nightly"))]
mod stable {
	use png;
	use buffer::Buffer;
	use pixel::{self, Pixel};
	use super::Color;

	impl<C, P, D> Color for Buffer<C, P, D>
		where C: pixel::Channel,
	        P: Pixel<C>
	{
		#[inline]
		fn color(&self) -> Option<(png::ColorType, png::BitDepth)> {
			None
		}
	}
}

#[cfg(feature = "nightly")]
mod nightly {
	use png;
	use png::ColorType::*;
	use png::BitDepth::*;
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
		fn color(&self) -> Option<(png::ColorType, png::BitDepth)> {
			None
		}
	}

	macro_rules! impl_for {
		($ch:ident, $px:ident => $color:path, $depth:path) => (
			impl<D, T: Float + 'static> Color for Buffer<$ch, $px<T>, D> {
				#[inline]
				fn color(&self) -> Option<(png::ColorType, png::BitDepth)> {
					Some(($color, $depth))
				}
			}
		)
	}

	impl_for!(u8, Luma => Grayscale, Eight);
	impl_for!(u8, Lumaa => GrayscaleAlpha, Eight);
	impl_for!(u8, Rgb => RGB, Eight);
	impl_for!(u8, Rgba => RGBA, Eight);

	impl_for!(u16, Luma => Grayscale, Sixteen);
	impl_for!(u16, Lumaa => GrayscaleAlpha, Sixteen);
	impl_for!(u16, Rgb => RGB, Sixteen);
	impl_for!(u16, Rgba => RGBA, Sixteen);
}

cast! {
	(u8,  Luma), (u8,  Lumaa), (u8,  Rgb), (u8,  Rgba),
	(u16, Luma), (u16, Lumaa), (u16, Rgb), (u16, Rgba),
}
