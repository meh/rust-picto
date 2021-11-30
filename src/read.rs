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

use crate::decoder::{self, Decoder};
use crate::color;
use crate::pixel;
use crate::buffer::Buffer;
use crate::format::{self, Format};
use crate::error::{self, Error};

/// Load an image from an input stream, guessing its format.
///
/// # Example
///
/// ```
/// use std::fs::File;
///
/// use picto::read;
/// use picto::color::Rgb;
///
/// read::from::<Rgb, u8, _>(File::open("tests/boat.xyz").unwrap()).unwrap();
/// ```
pub fn from<P, C, R>(mut input: R) -> error::Result<Buffer<P, C, Vec<C>>>
	where P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      P: pixel::Write<C>,
	      C: pixel::Channel,
	      R: Read + Seek
{
	let format = r#try!(format::guess(input.by_ref()).ok_or(Error::Format("unsupported image format".into())));
	with_format(input, format)
}

/// Load an image from memory, guessing its format.
///
/// # Example
///
/// ```
/// use std::fs::File;
/// use std::io::Read;
///
/// use picto::read;
/// use picto::color::Rgb;
///
/// let mut buffer = Vec::new();
/// let mut file   = File::open("tests/boat.xyz").unwrap();
/// file.read_to_end(&mut buffer).unwrap();
///
/// read::from_memory::<Rgb, u8, _>(buffer).unwrap();
/// ```
pub fn from_memory<P, C, R>(input: R) -> error::Result<Buffer<P, C, Vec<C>>>
	where P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      P: pixel::Write<C>,
	      C: pixel::Channel,
	      R: AsRef<[u8]>
{
	from(Cursor::new(input))
}

/// Load an image from the given path, guessing its format.
///
/// # Example
///
/// ```
/// use picto::read;
/// use picto::color::Rgb;
///
/// read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();
/// ```
pub fn from_path<P, C, R>(path: R) -> error::Result<Buffer<P, C, Vec<C>>>
	where P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      P: pixel::Write<C>,
	      C: pixel::Channel,
	      R: AsRef<Path>
{
	from(BufReader::new(r#try!(File::open(path))))
}

/// Load an image from an input stream with the given format.
///
/// # Example
///
/// ```
/// use std::fs::File;
///
/// use picto::read;
/// use picto::color::Rgb;
/// use picto::Format;
///
/// read::with_format::<Rgb, u8, _>(File::open("tests/boat.xyz").unwrap(), Format::Xyz).unwrap();
/// ```
pub fn with_format<P, C, R>(input: R, format: Format) -> error::Result<Buffer<P, C, Vec<C>>>
	where P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      P: pixel::Write<C>,
	      C: pixel::Channel,
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
pub fn png<P, C, F, R>(input: R, func: F) -> error::Result<Buffer<P, C, Vec<C>>>
	where P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      P: pixel::Write<C>,
	      C: pixel::Channel,
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
pub fn jpeg<P, C, F, R>(input: R, func: F) -> error::Result<Buffer<P, C, Vec<C>>>
	where P: From<color::Rgb> + From<color::Luma>,
	      P: pixel::Write<C>,
	      C: pixel::Channel,
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
pub fn bmp<P, C, F, R>(input: R, func: F) -> error::Result<Buffer<P, C, Vec<C>>>
	where P: From<color::Rgb> + From<color::Rgba>,
	      P: pixel::Write<C>,
	      C: pixel::Channel,
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
pub fn tga<P, C, F, R>(input: R, func: F) -> error::Result<Buffer<P, C, Vec<C>>>
	where P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      P: pixel::Write<C>,
	      C: pixel::Channel,
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
pub fn gif<P, C, F, R>(input: R, func: F) -> error::Result<Buffer<P, C, Vec<C>>>
	where P: From<color::Rgb> + From<color::Rgba> + From<color::Luma> + From<color::Lumaa>,
	      P: pixel::Write<C>,
	      C: pixel::Channel,
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
pub fn xyz<P, C, F, R>(input: R, func: F) -> error::Result<Buffer<P, C, Vec<C>>>
	where P: From<color::Rgb> + From<color::Rgba>,
	      P: pixel::Write<C>,
	      C: pixel::Channel,
	      F: FnOnce(&mut decoder::xyz::Decoder<R>),
	      R: Read
{
	let mut decoder = decoder::xyz::Decoder::new(input);
	func(&mut decoder);
	decoder.frame()
}
