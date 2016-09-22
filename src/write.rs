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
use std::path::Path;
use std::fs::File;
use std::ops::Deref;

use encoder::{self, Encoder};
use color;
use pixel::{self, Pixel};
use buffer::Buffer;
use format::Format;
use error::{self, Error};

#[inline]
pub fn to<C, P, D, W>(output: W, buffer: &Buffer<C, P, D>) -> error::Result<()>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C>,
	      P: Into<color::Rgb> + Into<color::Rgba> + Into<color::Luma> + Into<color::Lumaa>,
	      D: Deref<Target = [C]>,
	      W: Write
{
	with_format(output, Format::Png, buffer)
}

#[inline]
pub fn to_path<C, P, D, W>(path: W, buffer: &Buffer<C, P, D>) -> error::Result<()>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C>,
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

	with_format(try!(File::create(path)), format, buffer)
}

#[inline]
pub fn with_format<C, P, D, W>(output: W, format: Format, buffer: &Buffer<C, P, D>) -> error::Result<()>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C>,
	      P: Into<color::Rgb> + Into<color::Rgba> + Into<color::Luma> + Into<color::Lumaa>,
	      D: Deref<Target = [C]>,
	      W: Write
{
	match format {
		#[cfg(feature = "png")]
		Format::Png =>
			encoder::png::Encoder::new(output).frame(buffer),

		#[cfg(feature = "bmp")]
		Format::Bmp =>
			encoder::bmp::Encoder::new(output).frame(buffer),

		#[cfg(feature = "tga")]
		Format::Tga =>
			encoder::tga::Encoder::new(output).frame(buffer),

		#[cfg(feature = "gif")]
		Format::Gif =>
			encoder::gif::Encoder::new(output).frame(buffer),

		_ =>
			Err(Error::Unsupported("unsupported image format".into()))
	}
}
