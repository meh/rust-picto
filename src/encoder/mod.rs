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

use std::ops::Deref;

use crate::{
	buffer::Buffer,
	error,
	pixel::{self, Pixel},
};

/// An image encoder.
pub trait Encoder<P, C, D>
where
	P: Pixel<C>,
	C: pixel::Channel,
	D: Deref<Target = [C]>,
{
	/// A frame for the image, respecting the previously defined metadata.
	fn frame(&mut self, buffer: &Buffer<P, C, D>) -> error::Result<()>;
}

#[cfg(feature = "png")]
pub mod png;

#[cfg(feature = "bmp")]
pub mod bmp;

#[cfg(feature = "tga")]
pub mod tga;

#[cfg(feature = "gif")]
pub mod gif;
