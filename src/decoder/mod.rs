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

use format::{Format, Color};
use buffer::{Buffer, Cast};
use pixel::{self, Pixel};
use color;

mod error;
pub use self::error::{Error, Result};

#[cfg(feature = "png")]
pub mod png;

#[cfg(feature = "jpeg")]
pub mod jpeg;

/// An image decoder.
pub trait Decoder {
	/// The format the decoder is going to return.
	fn format(&mut self) -> Result<Format>;

	/// The dimensions of the image to be loaded.
	fn dimensions(&mut self) -> Result<(u32, u32)>;

	/// The color type for the image to be loaded.
	fn color(&mut self) -> Result<Color>;

	/// Decode a frame from the stream.
	fn frame(&mut self) -> Result<Vec<u8>>;
}

/// Load the image from the given `Decoder`.
pub fn load<C, P, D>(mut decoder: D) -> Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      D: Decoder
{
	let dimensions = try!(decoder.dimensions());
	let color      = try!(decoder.color());
	let frame      = try!(decoder.frame());

	macro_rules! buffer {
		($ch:ty, $ty:path) => ({
			Ok(Cast::<C, P>::cast(try!(Buffer::<$ch, $ty, _>::from_raw(dimensions.0, dimensions.1, frame)
				.map_err(|_| Error::Format("wrong dimensions".into())))))
		});
	}

	match color {
		Color::Gray(8, false) =>
			buffer!(u8, color::Luma),

		Color::Gray(8, true) =>
			buffer!(u8, color::Lumaa),

		Color::Rgb(8, false) =>
			buffer!(u8, color::Rgb),

		Color::Rgb(8, true) =>
			buffer!(u8, color::Rgba),

		_ =>
			Err(Error::Format("unsupported color type".into()))
	}
}
