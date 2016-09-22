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

#![cfg_attr(feature = "nightly", feature(specialization))]

extern crate byteorder;
extern crate num;

pub extern crate palette;
pub use palette as color;

#[cfg(feature = "png")]
extern crate png;

#[cfg(feature = "jpeg")]
extern crate jpeg_decoder;

#[cfg(any(feature = "bmp", feature = "tga"))]
extern crate imagefmt;

#[cfg(feature = "xyz")]
extern crate xyz;

pub mod error;
pub use error::{Error, Result};

pub mod area;
pub use area::Area;

pub mod pixel;
pub use pixel::Pixel;

pub mod view;
pub use view::View;

pub mod iter;

mod buffer;
pub use buffer::Buffer;

pub mod format;
pub use format::Format;

mod decoder;
pub use decoder::Decoder;

mod encoder;
pub use encoder::Encoder;

pub mod read;
pub mod write;
