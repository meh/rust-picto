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

use buffer::Buffer;
use pixel::{self, Pixel};
use error;

/// An image encoder.
pub trait Encoder<C: pixel::Channel, P: Pixel<C>, D: Deref<Target = [C]>> {
	/// A frame for the image, respecting the previously defined metadata.
	fn frame(&mut self, buffer: &Buffer<C, P, D>) -> error::Result<()>;
}

macro_rules! cast {
	()     => ();
	(impl) => ();

	(impl ($ch:ident, $px:ident)) => (
		impl<T: Float + Copy + 'static> Cast<$px<T>> for Buffer<$ch, $px<T>, Vec<$ch>> {
			#[inline]
			fn cast(&self) -> Cow<[u8]> {
				let slice: &[$ch] = &*self;

				Cow::Borrowed(unsafe {
					slice::from_raw_parts(slice.as_ptr() as *const _, slice.len() * mem::size_of::<$ch>())
				})
			}
		}
	);

	(impl ($ch:ident, $px:ident), $($rest:tt)*) => (
		cast!(impl ($ch, $px));
		cast!(impl $($rest)*);
	);

	($($rest:tt)*) => (
		use std::borrow::Cow;

		trait Cast<P: Pixel<u8>> {
			fn cast(&self) -> Cow<[u8]>;
		}
		
		#[cfg(not(feature = "nightly"))]
		mod stable_cast {
			use std::ops::Deref;
			use std::borrow::Cow;

			use pixel::{self, Pixel};
			use buffer::Buffer;
			use super::Cast;

			impl<CI, PI, DI, PO> Cast<PO> for Buffer<CI, PI, DI>
				where CI: pixel::Channel,
				      PI: Pixel<CI> + pixel::Read<CI>,
				      PI: Into<PO>,
				      DI: Deref<Target = [CI]>,
				      PO: Pixel<u8> + pixel::Write<u8>
			{
				#[inline]
				fn cast(&self) -> Cow<[u8]> {
					Cow::Owned(self.convert::<u8, PO>().into_raw())
				}
			}
		}
		
		#[cfg(feature = "nightly")]
		mod nightly_cast {
			use std::ops::Deref;
			use std::borrow::Cow;
			use std::slice;
			use std::mem;

			use num::Float;
			use pixel::{self, Pixel};
			use buffer::Buffer;
			use super::Cast;

			impl<CI, PI, DI, PO> Cast<PO> for Buffer<CI, PI, DI>
				where CI: pixel::Channel,
				      PI: Pixel<CI> + pixel::Read<CI>,
				      PI: Into<PO>,
				      DI: Deref<Target = [CI]>,
				      PO: Pixel<u8> + pixel::Write<u8>
			{
				#[inline]
				default
				fn cast(&self) -> Cow<[u8]> {
					Cow::Owned(self.convert::<u8, PO>().into_raw())
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

#[cfg(feature = "bmp")]
pub mod bmp;

#[cfg(feature = "tga")]
pub mod tga;

#[cfg(feature = "gif")]
pub mod gif;
