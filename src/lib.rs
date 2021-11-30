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

//! Image encoding/decoding and manipulation library.

#![cfg_attr(feature = "nightly", feature(specialization))]
#[cfg(test)] #[macro_use] extern crate approx;

extern crate byteorder;
extern crate num;

#[doc(hidden)]
pub extern crate palette;

/// Color types.
pub mod color {
	pub use palette::*;
	pub use palette::pixel::*;
}

#[cfg(feature = "png")]
extern crate png;

#[cfg(feature = "jpeg")]
extern crate jpeg_decoder;

#[cfg(any(feature = "bmp", feature = "tga"))]
extern crate imagefmt;

#[cfg(feature = "gif")]
extern crate gif;

#[cfg(feature = "xyz")]
extern crate xyz;

#[cfg(feature = "processing")]
extern crate color_quant;

#[cfg(feature = "processing")]
extern crate exoquant;

#[macro_use]
mod util;

mod error;
pub use crate::error::{Error, Result};

mod parameter;
pub use crate::parameter::{HasParameters, Parameter};

mod region;
pub use crate::region::Region;

mod orientation;
pub use crate::orientation::Orientation;

/// Basic traits for types within buffers and views.
pub mod pixel;
pub use crate::pixel::Pixel;

/// Buffer related functionality.
pub mod buffer;
pub use crate::buffer::Buffer;

/// Types of view within a `Buffer`.
pub mod view;
pub use crate::view::View;

/// Iterator types.
pub mod iter;

/// Image manipulation functions.
#[cfg(feature = "processing")]
pub mod processing;

/// Image format related functions.
pub mod format;
pub use crate::format::Format;

mod decoder;
pub use crate::decoder::Decoder;

mod encoder;
pub use crate::encoder::Encoder;

/// Image decoding functions.
pub mod read;

/// Image encoding functions.
pub mod write;
