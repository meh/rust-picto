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

use std::path::Path;
use std::fs::File;
use std::io::{Read, Seek, Cursor, BufReader};

use decoder::{self, Decoder};
use color;
use pixel;
use buffer::Buffer;
use format::{self, Format};
use error::{self, Error};

/// Load an image from an input stream, guessing its format.
pub fn from<C, P, R>(mut input: R) -> error::Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      R: Read + Seek
{
	let format = try!(format::guess(input.by_ref()).ok_or(Error::Format("unsupported image format".into())));
	with_format(input, format)
}

/// Load an image from memory, guessing its format.
pub fn from_memory<C, P>(slice: &[u8]) -> error::Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>
{
	from(Cursor::new(slice))
}

/// Load an image from the given path, guessing its format.
pub fn from_path<C, P, R>(path: R) -> error::Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      R: AsRef<Path>
{
	from(BufReader::new(try!(File::open(path))))
}

/// Load an image from an input stream with the given format.
pub fn with_format<C, P, R>(input: R, format: Format) -> error::Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      R: Read + Seek
{
	match format {
		#[cfg(feature = "png")]
		Format::Png =>
			png(input, |_| { }),

		#[cfg(feature = "jpeg")]
		Format::Jpeg =>
			jpeg(input, |_| { }),

		#[cfg(feature = "bmp")]
		Format::Bmp =>
			bmp(input, |_| { }),

		#[cfg(feature = "tga")]
		Format::Tga =>
			tga(input, |_| { }),

		#[cfg(feature = "gif")]
		Format::Gif =>
			gif(input, |_| { }),

		#[cfg(feature = "xyz")]
		Format::Xyz =>
			xyz(input, |_| { }),

		_ =>
			Err(Error::Unsupported("unsupported image format".into())),
	}
}

/// Load a PNG image from an input stream, with the ability to set parameters
/// on the decoder.
#[cfg(feature = "png")]
#[inline]
pub fn png<C, P, F, R>(input: R, func: F) -> error::Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      F: FnOnce(&mut decoder::png::Decoder<R>),
	      R: Read
{
	let mut decoder = decoder::png::Decoder::new(input);
	func(&mut decoder);
	decoder.frame()
}

/// Load a JPEG image from an input stream, with the ability to set parameters
/// on the decoder.
#[cfg(feature = "jpeg")]
#[inline]
pub fn jpeg<C, P, F, R>(input: R, func: F) -> error::Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Luma>,
	      F: FnOnce(&mut decoder::jpeg::Decoder<R>),
	      R: Read
{
	let mut decoder = decoder::jpeg::Decoder::new(input);
	func(&mut decoder);
	decoder.frame()
}

/// Load a BMP image from an input stream, with the ability to set parameters
/// on the decoder.
#[cfg(feature = "bmp")]
#[inline]
pub fn bmp<C, P, F, R>(input: R, func: F) -> error::Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba>,
	      F: FnOnce(&mut decoder::bmp::Decoder<R>),
	      R: Read + Seek
{
	let mut decoder = decoder::bmp::Decoder::new(input);
	func(&mut decoder);
	decoder.frame()
}

/// Load a TGA image from an input stream, with the ability to set parameters
/// on the decoder.
#[cfg(feature = "tga")]
#[inline]
pub fn tga<C, P, F, R>(input: R, func: F) -> error::Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      F: FnOnce(&mut decoder::tga::Decoder<R>),
	      R: Read + Seek
{
	let mut decoder = decoder::tga::Decoder::new(input);
	func(&mut decoder);
	decoder.frame()
}

/// Load a GIF image from an input stream, with the ability to set parameters
/// on the decoder.
#[cfg(feature = "gif")]
#[inline]
pub fn gif<C, P, F, R>(input: R, func: F) -> error::Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      F: FnOnce(&mut decoder::gif::Decoder<R>),
	      R: Read
{
	let mut decoder = decoder::gif::Decoder::new(input);
	func(&mut decoder);
	decoder.frame()
}

/// Load an XYZ image from an input stream, with the ability to set parameters
/// on the decoder.
#[cfg(feature = "xyz")]
#[inline]
pub fn xyz<C, P, F, R>(input: R, func: F) -> error::Result<Buffer<C, P, Vec<C>>>
	where C: pixel::Channel,
	      P: pixel::Write<C>,
	      P: From<color::Rgb> + From<color::Rgba>,
	      F: FnOnce(&mut decoder::xyz::Decoder<R>),
	      R: Read
{
	let mut decoder = decoder::xyz::Decoder::new(input);
	func(&mut decoder);
	decoder.frame()
}
