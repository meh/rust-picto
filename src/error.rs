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

use std::{error, fmt, io};

/// Error enumeration type.
#[derive(Debug)]
pub enum Error {
	/// An IO error.
	Io(io::Error),

	/// An image format error.
	Format(String),

	/// An unsupported error.
	Unsupported(String),
}

/// Results from the library.
pub type Result<T> = ::std::result::Result<T, Error>;

impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
		Error::Io(value)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
		f.write_str(error::Error::description(self))
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::Io(ref err) => err.description(),

			Error::Format(ref err) | Error::Unsupported(ref err) => err,
		}
	}
}

#[cfg(feature = "png")]
mod png {
	use png;

	use super::Error;

	impl From<png::DecodingError> for Error {
		fn from(value: png::DecodingError) -> Self {
			match value {
				png::DecodingError::IoError(err) => Error::Io(err),
				png::DecodingError::Format(desc) => Error::Format(desc.into_owned()),
				png::DecodingError::InvalidSignature => Error::Format("invalid signature".into()),
				png::DecodingError::CrcMismatch { .. } => Error::Format("CRC error".into()),
				png::DecodingError::Other(desc) => Error::Format(desc.into_owned()),
				png::DecodingError::CorruptFlateStream => Error::Format("compressed data stream corrupted".into()),
			}
		}
	}

	impl From<png::EncodingError> for Error {
		fn from(value: png::EncodingError) -> Self {
			match value {
				png::EncodingError::IoError(err) => Error::Io(err),
				png::EncodingError::Format(desc) => Error::Format(desc.into_owned()),
			}
		}
	}
}

#[cfg(feature = "jpeg")]
mod jpeg {
	use jpeg_decoder as jpeg;

	use super::Error;

	impl From<jpeg::Error> for Error {
		fn from(value: jpeg::Error) -> Self {
			match value {
				jpeg::Error::Format(desc) => Error::Format(desc),
				jpeg::Error::Unsupported(desc) => Error::Unsupported(format!("{:?}", desc)),
				jpeg::Error::Io(err) => Error::Io(err),
				jpeg::Error::Internal(err) => Error::Format(err.description().to_owned()),
			}
		}
	}
}

#[cfg(any(feature = "bmp", feature = "tga"))]
mod imagefmt {
	use imagefmt;

	use super::Error;

	impl From<imagefmt::Error> for Error {
		fn from(value: imagefmt::Error) -> Self {
			match value {
				imagefmt::Error::Io(err) => Error::Io(err),
				imagefmt::Error::InvalidData(desc) | imagefmt::Error::InvalidArg(desc) | imagefmt::Error::Internal(desc) => {
					Error::Format(desc.into())
				}
				imagefmt::Error::Unsupported(desc) => Error::Unsupported(desc.into()),
			}
		}
	}
}

#[cfg(feature = "gif")]
mod gif {
	use gif;

	use super::Error;

	impl From<gif::DecodingError> for Error {
		fn from(value: gif::DecodingError) -> Self {
			match value {
				gif::DecodingError::Io(err) => Error::Io(err),
				gif::DecodingError::Format(desc) => Error::Format(desc.into()),
				gif::DecodingError::Internal(desc) => Error::Format(desc.into()),
			}
		}
	}
}
