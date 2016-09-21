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

use format::Format;
use buffer::Buffer;
use pixel::{self, Pixel};
use error;

/// An image encoder.
pub trait Encoder<C: pixel::Channel, P: Pixel<C>, D: Deref<Target = [C]>> {
	/// The format the encoder must use.
	fn format(&mut self, format: Format) -> error::Result<()>;

	/// A frame for the image, respecting the previously defined metadata.
	fn frame(&mut self, buffer: &Buffer<C, P, D>) -> error::Result<()>;
}

macro_rules! cast {
	()  => ();
	(@) => ();

	(@ ($ch:ident, $px:ident)) => (
		use color::$px;

		impl<T: Float + Copy + 'static> Cast<$ch, $px<T>> for Buffer<$ch, $px<T>, Vec<$ch>> {
			#[inline]
			fn cast(&self) -> Cow<[$ch]> {
				Cow::Borrowed(&*self)
			}
		}
	);

	(@ ($ch:ident, $px:ident), $($rest:tt)*) => (
		cast!(@ ($ch, $px));
		cast!(@ $($rest)*);
	);

	($($rest:tt)*) => (
		use std::borrow::Cow;

		trait Cast<C: pixel::Channel, P: Pixel<C>> {
			fn cast(&self) -> Cow<[C]>;
		}
		
		#[cfg(not(feature = "nightly"))]
		mod stable_cast {
			use std::ops::Deref;
			use std::borrow::Cow;

			use pixel::{self, Pixel};
			use buffer::Buffer;
			use super::Cast;

			impl<CI, PI, DI, CO, PO> Cast<CO, PO> for Buffer<CI, PI, DI>
				where CI: pixel::Channel,
				      PI: Pixel<CI> + pixel::Read<CI>,
				      PI: Into<PO>,
				      DI: Deref<Target = [CI]>,
				      CO: pixel::Channel,
				      PO: Pixel<CO> + pixel::Write<CO>
			{
				#[inline]
				fn cast(&self) -> Cow<[CO]> {
					Cow::Owned(self.convert::<CO, PO>().into_raw())
				}
			}
		}
		
		#[cfg(feature = "nightly")]
		mod nightly_cast {
			use std::ops::Deref;
			use std::borrow::Cow;

			use num::Float;
			use pixel::{self, Pixel};
			use buffer::Buffer;
			use super::Cast;

			impl<CI, PI, DI, CO, PO> Cast<CO, PO> for Buffer<CI, PI, DI>
				where CI: pixel::Channel,
				      PI: Pixel<CI> + pixel::Read<CI>,
				      PI: Into<PO>,
				      DI: Deref<Target = [CI]>,
				      CO: pixel::Channel,
				      PO: Pixel<CO> + pixel::Write<CO>
			{
				#[inline]
				default
				fn cast(&self) -> Cow<[CO]> {
					Cow::Owned(self.convert::<CO, PO>().into_raw())
				}
			}

			cast!(@ $($rest)*);
		}
	);
}



#[cfg(feature = "png")]
pub mod png;
