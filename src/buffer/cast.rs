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

use std::borrow::Cow;
use pixel::{self, Pixel};
use buffer::Buffer;

pub trait Into<C, P>
	where C: pixel::Channel,
	      P: Pixel<C>
{
	fn into(self) -> Buffer<C, P, Vec<C>>;
}

pub trait Bytes<P>
	where P: Pixel<u8>
{
	fn bytes(&self) -> Cow<[u8]>;
}

#[cfg(not(feature = "nightly"))]
mod stable {
	use std::borrow::Cow;
	use std::ops::Deref;
	use pixel;
	use buffer::Buffer;

	impl<CI, PI, DI, CO, PO> super::Into<CO, PO> for Buffer<CI, PI, DI>
		where CI: pixel::Channel,
		      PI: pixel::Read<CI>,
		      DI: Deref<Target = [CI]>,
		      CO: pixel::Channel,
		      PO: pixel::Write<CO>,
		      PO: From<PI>
	{
		#[inline]
		fn into(self) -> Buffer<CO, PO, Vec<CO>> {
			self.convert::<CO, PO>()
		}
	}

	impl<CI, PI, DI, PO> super::Bytes<PO> for Buffer<CI, PI, DI>
		where CI: pixel::Channel,
		      PI: pixel::Read<CI>,
		      PI: Into<PO>,
		      DI: Deref<Target = [CI]>,
		      PO: pixel::Write<u8>
	{
		#[inline]
		fn bytes(&self) -> Cow<[u8]> {
			Cow::Owned(self.convert::<u8, PO>().into_raw())
		}
	}
}

#[cfg(feature = "nightly")]
mod nightly {
	use std::slice;
	use std::mem;
	use std::borrow::Cow;
	use std::ops::Deref;

	use num::Float;
	use pixel;
	use buffer::Buffer;
	use color::{Luma, Rgb, Hsl, Hsv, Hwb, Lab, Lch, Xyz, Yxy};
	use color::{Lumaa, Rgba, Hsla, Hsva, Hwba, Laba, Lcha, Xyza, Yxya};

	impl<CI, PI, DI, CO, PO> super::Into<CO, PO> for Buffer<CI, PI, DI>
		where CI: pixel::Channel,
		      PI: pixel::Read<CI>,
		      DI: Deref<Target = [CI]>,
		      CO: pixel::Channel,
		      PO: pixel::Write<CO>,
		      PO: From<PI>
	{
		#[inline]
		default
		fn into(self) -> Buffer<CO, PO, Vec<CO>> {
			self.convert::<CO, PO>()
		}
	}

	impl<CI, PI, DI, PO> super::Bytes<PO> for Buffer<CI, PI, DI>
		where CI: pixel::Channel,
		      PI: pixel::Read<CI>,
		      PI: Into<PO>,
		      DI: Deref<Target = [CI]>,
		      PO: pixel::Write<u8>
	{
		#[inline]
		default
		fn bytes(&self) -> Cow<[u8]> {
			Cow::Owned(self.convert::<u8, PO>().into_raw())
		}
	}

	macro_rules! impl_for {
		()     => ();
		(impl) => ();

		(impl ($ch:ident, $px:ident)) => (
			impl<T: Float + Copy + 'static> super::Into<$ch, $px<T>> for Buffer<$ch, $px<T>, Vec<$ch>> {
				#[inline]
				fn into(self) -> Buffer<$ch, $px<T>, Vec<$ch>> {
					self
				}
			}

			impl<T: Float + Copy + 'static> super::Bytes<$px<T>> for Buffer<$ch, $px<T>, Vec<$ch>> {
				#[inline]
				fn bytes(&self) -> Cow<[u8]> {
					let slice: &[$ch] = &*self;

					Cow::Borrowed(unsafe {
						slice::from_raw_parts(slice.as_ptr() as *const _, slice.len() * mem::size_of::<$ch>())
					})
				}
			}
		);

		(impl ($ch:ident, $px:ident), $($rest:tt)*) => (
			impl_for!(impl ($ch, $px));
			impl_for!(impl $($rest)*);
		);

		($($rest:tt)*) => (
			impl_for!(impl $($rest)*);
		);
	}

	impl_for! {
		(u8, Luma),  (u8, Rgb),  (u8, Hsl),  (u8, Hsv),  (u8, Hwb),  (u8, Lab),  (u8, Lch),  (u8, Xyz),  (u8, Yxy),
		(u8, Lumaa), (u8, Rgba), (u8, Hsla), (u8, Hsva), (u8, Hwba), (u8, Laba), (u8, Lcha), (u8, Xyza), (u8, Yxya),
	}

	impl_for! {
		(u16, Luma),  (u16, Rgb),  (u16, Hsl),  (u16, Hsv),  (u16, Hwb),  (u16, Lab),  (u16, Lch),  (u16, Xyz),  (u16, Yxy),
		(u16, Lumaa), (u16, Rgba), (u16, Hsla), (u16, Hsva), (u16, Hwba), (u16, Laba), (u16, Lcha), (u16, Xyza), (u16, Yxya),
	}
}
