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

use std::io::{Write, BufWriter};
use std::path::Path;
use std::fs::File;
use std::ops::Deref;

use encoder::{self, Encoder};
use color;
use pixel;
use buffer::Buffer;
use format::Format;
use error::{self, Error};

/// Write the buffer to the output stream in PNG format.
#[inline]
pub fn to<C, P, D, W>(output: W, buffer: &Buffer<C, P, D>) -> error::Result<()>
	where C: pixel::Channel,
	      P: pixel::Read<C>,
	      P: Into<color::Rgb> + Into<color::Rgba> + Into<color::Luma> + Into<color::Lumaa>,
	      D: Deref<Target = [C]>,
	      W: Write
{
	with_format(output, Format::Png, buffer)
}

/// Write the buffer to the given path guessing the format based on the file
/// extension.
#[inline]
pub fn to_path<C, P, D, W>(path: W, buffer: &Buffer<C, P, D>) -> error::Result<()>
	where C: pixel::Channel,
	      P: pixel::Read<C>,
	      P: Into<color::Rgb> + Into<color::Rgba> + Into<color::Luma> + Into<color::Lumaa>,
	      D: Deref<Target = [C]>,
	      W: AsRef<Path>
{
	let path      = path.as_ref();
	let extension = path.extension().and_then(|p| p.to_str()).map(|p| p.to_lowercase());
	let format    = match extension.as_ref().map(|p| p.as_ref()) {
		Some("png") =>
			Format::Png,

		Some("jpg") | Some("jpeg") =>
			Format::Jpeg,

		Some("bmp") =>
			Format::Bmp,

		Some("tga") =>
			Format::Tga,

		Some("gif") =>
			Format::Gif,

		_ =>
			return Err(Error::Unsupported("unsupported image format".into()))
	};

	with_format(BufWriter::new(try!(File::create(path))), format, buffer)
}

/// Write the buffer to the output stream using the given format.
#[inline]
pub fn with_format<C, P, D, W>(output: W, format: Format, buffer: &Buffer<C, P, D>) -> error::Result<()>
	where C: pixel::Channel,
	      P: pixel::Read<C>,
	      P: Into<color::Rgb> + Into<color::Rgba> + Into<color::Luma> + Into<color::Lumaa>,
	      D: Deref<Target = [C]>,
	      W: Write
{
	match format {
		#[cfg(feature = "png")]
		Format::Png =>
			png(output, buffer, |_| { }),

		#[cfg(feature = "bmp")]
		Format::Bmp =>
			bmp(output, buffer, |_| { }),

		#[cfg(feature = "tga")]
		Format::Tga =>
			tga(output, buffer, |_| { }),

		#[cfg(feature = "gif")]
		Format::Gif =>
			gif(output, buffer, |_| { }),

		_ =>
			Err(Error::Unsupported("unsupported image format".into()))
	}
}

/// Write a PNG image to an output stream, with the ability to set the
/// parameters on the encoder.
#[cfg(feature = "png")]
#[inline]
pub fn png<C, P, D, F, W>(output: W, buffer: &Buffer<C, P, D>, func: F) -> error::Result<()>
	where C: pixel::Channel,
	      P: pixel::Read<C>,
	      P: Into<color::Rgb> + Into<color::Rgba> + Into<color::Luma> + Into<color::Lumaa>,
	      D: Deref<Target = [C]>,
	      F: FnOnce(&mut encoder::png::Encoder<W>),
	      W: Write
{
	let mut encoder = encoder::png::Encoder::new(output);
	func(&mut encoder);
	encoder.frame(buffer)
}

/// Write a BMP image to an output stream, with the ability to set the
/// parameters on the encoder.
#[cfg(feature = "bmp")]
#[inline]
pub fn bmp<C, P, D, F, W>(output: W, buffer: &Buffer<C, P, D>, func: F) -> error::Result<()>
	where C: pixel::Channel,
	      P: pixel::Read<C>,
	      P: Into<color::Rgb> + Into<color::Rgba>,
	      D: Deref<Target = [C]>,
	      F: FnOnce(&mut encoder::bmp::Encoder<W>),
	      W: Write
{
	let mut encoder = encoder::bmp::Encoder::new(output);
	func(&mut encoder);
	encoder.frame(buffer)
}

/// Write a TGA image to an output stream, with the ability to set the
/// parameters on the encoder.
#[cfg(feature = "tga")]
#[inline]
pub fn tga<C, P, D, F, W>(output: W, buffer: &Buffer<C, P, D>, func: F) -> error::Result<()>
	where C: pixel::Channel,
	      P: pixel::Read<C>,
	      P: Into<color::Rgb> + Into<color::Rgba> + Into<color::Luma> + Into<color::Lumaa>,
	      D: Deref<Target = [C]>,
	      F: FnOnce(&mut encoder::tga::Encoder<W>),
	      W: Write
{
	let mut encoder = encoder::tga::Encoder::new(output);
	func(&mut encoder);
	encoder.frame(buffer)
}

/// Write a GIF image to an output stream, with the ability to set the
/// parameters on the encoder.
#[cfg(feature = "gif")]
#[inline]
pub fn gif<C, P, D, F, W>(output: W, buffer: &Buffer<C, P, D>, func: F) -> error::Result<()>
	where C: pixel::Channel,
	      P: pixel::Read<C>,
	      P: Into<color::Rgb> + Into<color::Rgba> + Into<color::Luma> + Into<color::Lumaa>,
	      D: Deref<Target = [C]>,
	      F: FnOnce(&mut encoder::gif::Encoder<W>),
	      W: Write
{
	let mut encoder = encoder::gif::Encoder::new(output);
	func(&mut encoder);
	encoder.frame(buffer)
}
