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
use error::{self, Error};
use format::Format;
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
	fn format(&mut self, _format: Format) -> error::Result<()> {
		Ok(())
	}

	fn frame(&mut self, buffer: &Buffer<C, P, D>) -> error::Result<()> {
		let (color, depth) = buffer.color().unwrap_or((png::ColorType::RGBA, png::BitDepth::Eight));

		let mut encoder = png::Encoder::new(self.inner.by_ref(), buffer.width(), buffer.height());
		encoder.set(color).set(depth);

		let mut writer = try!(encoder.write_header());

		macro_rules! write {
			($ch:ty, $ty:path) => (
				try!(writer.write_image_data(Cast::<$ch, $ty>::cast(buffer).as_ref()))
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

			_ => unreachable!()
		}

		Ok(())
	}
}

impl From<png::EncodingError> for Error {
	fn from(value: png::EncodingError) -> Self {
		match value {
			png::EncodingError::IoError(err) =>
				Error::Io(err),

			png::EncodingError::Format(desc) =>
				Error::Format(desc.into_owned()),
		}
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
	use num::Float;
	use buffer::Buffer;
	use pixel::{self, Pixel};
	use color;
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

	impl<D, T: Float + 'static> Color for Buffer<u8, color::Luma<T>, D> {
		#[inline]
		fn color(&self) -> Option<(png::ColorType, png::BitDepth)> {
			Some((png::ColorType::Grayscale, png::BitDepth::Eight))
		}
	}

	impl<D, T: Float + 'static> Color for Buffer<u8, color::Lumaa<T>, D> {
		#[inline]
		fn color(&self) -> Option<(png::ColorType, png::BitDepth)> {
			Some((png::ColorType::GrayscaleAlpha, png::BitDepth::Eight))
		}
	}

	impl<D, T: Float + 'static> Color for Buffer<u8, color::Rgb<T>, D> {
		#[inline]
		fn color(&self) -> Option<(png::ColorType, png::BitDepth)> {
			Some((png::ColorType::RGB, png::BitDepth::Eight))
		}
	}

	impl<D, T: Float + 'static> Color for Buffer<u8, color::Rgba<T>, D> {
		#[inline]
		fn color(&self) -> Option<(png::ColorType, png::BitDepth)> {
			Some((png::ColorType::RGBA, png::BitDepth::Eight))
		}
	}
}

cast! { (u8, Luma), (u8, Lumaa), (u8, Rgb), (u8, Rgba) }
