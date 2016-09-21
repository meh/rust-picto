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

use buffer::Buffer;
use pixel::{self, Pixel};
use error;

/// A decoder parameter.
pub trait Parameter<T>: Sized {
	fn get(from: &mut T) -> error::Result<Self>;
}

/// An image decoder.
pub trait Decoder<C: pixel::Channel, P: Pixel<C>>: Sized {
	/// Get information from the decoder.
	fn get<T: Parameter<Self>>(&mut self) -> error::Result<T> {
		T::get(self)
	}

	/// Decode a frame from the stream.
	fn frame(&mut self) -> error::Result<Buffer<C, P, Vec<C>>>;
}

macro_rules! cast {
	()     => ();
	(impl) => ();

	(impl ($ch:ident, $px:ident)) => (
		impl<T: Float + Copy + 'static> Cast<$ch, $px<T>> for Buffer<$ch, $px<T>, Vec<$ch>> {
			#[inline]
			fn cast(self) -> Buffer<$ch, $px<T>, Vec<$ch>> {
				self
			}
		}
	);

	(impl ($ch:ident, $px:ident), $($rest:tt)*) => (
		cast!(impl ($ch, $px));
		cast!(impl $($rest)*);
	);

	($($rest:tt)*) => (
		trait Cast<C: pixel::Channel, P: Pixel<C>> {
			fn cast(self) -> Buffer<C, P, Vec<C>>;
		}

		#[cfg(not(feature = "nightly"))]
		mod stable_cast {
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
				#[inline]
				fn cast(self) -> Buffer<CO, PO, Vec<CO>> {
					self.convert::<CO, PO>()
				}
			}
		}

		#[cfg(feature = "nightly")]
		mod nightly_cast {
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
				#[inline]
				default
				fn cast(self) -> Buffer<CO, PO, Vec<CO>> {
					self.convert::<CO, PO>()
				}
			}

			#[allow(unused_imports)]
			use color::{
				Luma, Rgb, Hsl, Hsv, Hwb, Lab, Lch, Xyz, Yxy,
			  Lumaa, Rgba, Hsla, Hsva, Hwba, Laba, Lcha, Xyza, Yxya
			};

			#[allow(unused_imports)]
			use color::pixel::Srgb;

			cast!(impl $($rest)*);
		}
	);
}

#[cfg(feature = "png")]
pub mod png;

#[cfg(feature = "jpeg")]
pub mod jpeg;

#[cfg(feature = "bmp")]
pub mod bmp;

#[cfg(feature = "tga")]
pub mod tga;
