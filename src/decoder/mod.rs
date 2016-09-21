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

use format::Format;
use buffer::Buffer;
use pixel::{self, Pixel};
use error;

#[cfg(feature = "png")]
pub mod png;

#[cfg(feature = "jpeg")]
pub mod jpeg;

/// An image decoder.
pub trait Decoder<C: pixel::Channel, P: Pixel<C>> {
	/// The format the decoder is going to return.
	fn format(&mut self) -> error::Result<Format>;

	/// Decode a frame from the stream.
	fn frame(&mut self) -> error::Result<Buffer<C, P, Vec<C>>>;
}
