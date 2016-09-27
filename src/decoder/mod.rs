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

use buffer::Buffer;
use pixel::{self, Pixel};
use error;

/// An image decoder.
pub trait Decoder<C, P>
	where C: pixel::Channel,
	      P: Pixel<C>
{
	/// Decode a frame from the stream.
	fn frame(&mut self) -> error::Result<Buffer<C, P, Vec<C>>>;
}

#[cfg(feature = "png")]
pub mod png;

#[cfg(feature = "jpeg")]
pub mod jpeg;

#[cfg(feature = "bmp")]
pub mod bmp;

#[cfg(feature = "tga")]
pub mod tga;

#[cfg(feature = "gif")]
pub mod gif;

#[cfg(feature = "xyz")]
pub mod xyz;
