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

/// An image decoder.
pub trait Decoder<C: pixel::Channel, P: Pixel<C>> {
	/// The format the decoder is going to return.
	fn format(&mut self) -> error::Result<Format>;

	/// Decode a frame from the stream.
	fn frame(&mut self) -> error::Result<Buffer<C, P, Vec<C>>>;
}

macro_rules! cast {
	()  => ();
	(@) => ();

	(@ ($ch:ident, $px:ident)) => (
		use color::$px;

		impl<T: Float + Copy + 'static> Cast<$ch, $px<T>> for Buffer<$ch, $px<T>, Vec<$ch>> {
			fn cast(self) -> Buffer<$ch, $px<T>, Vec<$ch>> {
				self
			}
		}
	);

	(@ ($ch:ident, $px:ident), $($rest:tt)*) => (
		cast!(@ ($ch, $px));
		cast!(@ $($rest)*);
	);

	($($rest:tt)*) => (
		trait Cast<C: pixel::Channel, P: Pixel<C>> {
			fn cast(self) -> Buffer<C, P, Vec<C>>;
		}

		#[cfg(not(feature = "nightly"))]
		mod stable {
			use std::ops::Deref;

			use pixel::{self, Pixel};
			use buffer::Buffer;
			use super::Cast;

			impl<CI, PI, DI, CO, PO> Cast<CO, PO> for Buffer<CI, PI, DI>
				where CI: pixel::Channel,
				      PI: Pixel<CI> + pixel::Read<CI>,
				      DI: Deref<Target = [CI]>,
				      CO: pixel::Channel,
				      PO: Pixel<CO> + pixel::Write<CO>,
				      PO: From<PI>
			{
				fn cast(self) -> Buffer<CO, PO, Vec<CO>> {
					self.convert::<CO, PO>()
				}
			}
		}

		#[cfg(feature = "nightly")]
		mod nightly {
			use std::ops::Deref;

			use num::Float;
			use pixel::{self, Pixel};
			use buffer::Buffer;
			use super::Cast;

			impl<CI, PI, DI, CO, PO> Cast<CO, PO> for Buffer<CI, PI, DI>
				where CI: pixel::Channel,
				      PI: Pixel<CI> + pixel::Read<CI>,
				      DI: Deref<Target = [CI]>,
				      CO: pixel::Channel,
				      PO: Pixel<CO> + pixel::Write<CO>,
				      PO: From<PI>
			{
				default
				fn cast(self) -> Buffer<CO, PO, Vec<CO>> {
					self.convert::<CO, PO>()
				}
			}

			cast!(@ $($rest)*);
		}
	);
}

#[cfg(feature = "png")]
pub mod png;

#[cfg(feature = "jpeg")]
pub mod jpeg;
