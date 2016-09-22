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

use num::{NumCast, Float, Zero};

use color::{Luma, Rgb, Hsl, Hsv, Hwb, Lab, Lch, Xyz, Yxy};
use color::{Lumaa, Rgba, Hsla, Hsva, Hwba, Laba, Lcha, Xyza, Yxya};
use color::pixel::Srgb;
use color::RgbHue;

/// Type for the `Channel` in a `Pixel`, this is typically the type for the
/// element in a buffer as well.
pub trait Channel: Zero + Copy + 'static {
	fn from<T: Float + 'static>(value: T) -> Self;
}

impl Channel for u8 {
	fn from<T: Float + 'static>(value: T) -> Self {
		NumCast::from(value * NumCast::from(u8::max_value()).unwrap()).unwrap()
	}
}

impl Channel for u16 {
	fn from<T: Float + 'static>(value: T) -> Self {
		NumCast::from(value * NumCast::from(u16::max_value()).unwrap()).unwrap()
	}
}

impl Channel for f32 {
	fn from<T: Float + 'static>(value: T) -> Self {
		NumCast::from(value).unwrap()
	}
}

impl Channel for f64 {
	fn from<T: Float + 'static>(value: T) -> Self {
		NumCast::from(value).unwrap()
	}
}

/// The type for a `Pixel`.
pub trait Pixel<C: Channel>: Copy + 'static {
	/// The number of channels in the `Pixel`.
	fn channels() -> usize;
}

macro_rules! impl_for {
	($n:expr, $ty:ident) => (
		impl<C: Channel, T: Float + 'static> Pixel<C> for $ty<T> {
			#[inline]
			fn channels() -> usize {
				$n
			}
		}
	);

	($n:expr, $ty:ident, $($rest:ident),*) => (
		impl_for!($n, $ty);
		impl_for!($n, $($rest),*);
	);
}

impl_for!(1, Luma);
impl_for!(2, Lumaa);
impl_for!(3, Rgb, Srgb, Hsl, Hsv, Hwb, Lab, Lch, Xyz, Yxy);
impl_for!(4, Rgba, Hsla, Hsva, Hwba, Laba, Lcha, Xyza, Yxya);

/// A `Pixel` readable from a slice.
pub trait Read<C: Channel> {
	/// Read a `Pixel` from the slice.
	fn read(data: &[C]) -> Self;
}

macro_rules! impl_for {
	(u8 1 -> $ty:ident) => (
		impl<T: Float + 'static> Read<u8> for $ty<T> {
			#[inline]
			fn read(data: &[u8]) -> Self {
				$ty::new_u8(data[0])
			}
		}
	);

	(u8 2 -> $ty:ident) => (
		impl<T: Float + 'static> Read<u8> for $ty<T> {
			#[inline]
			fn read(data: &[u8]) -> Self {
				$ty::new_u8(data[0], data[1])
			}
		}
	);

	(u8 3 -> $ty:ident) => (
		impl<T: Float + 'static> Read<u8> for $ty<T> {
			#[inline]
			fn read(data: &[u8]) -> Self {
				$ty::new_u8(data[0], data[1], data[2])
			}
		}
	);

	(u8 4 -> $ty:ident) => (
		impl<T: Float + 'static> Read<u8> for $ty<T> {
			#[inline]
			fn read(data: &[u8]) -> Self {
				$ty::new_u8(data[0], data[1], data[2], data[3])
			}
		}
	);

	(u16 1 -> $ty:ident) => (
		impl<T: Float + 'static> Read<u16> for $ty<T> {
			#[inline]
			fn read(data: &[u16]) -> Self {
				$ty::new(
					T::from(data[0]).unwrap() / T::from(u16::max_value()).unwrap()
				)
			}
		}
	);

	(u16 2 -> $ty:ident) => (
		impl<T: Float + 'static> Read<u16> for $ty<T> {
			#[inline]
			fn read(data: &[u16]) -> Self {
				$ty::new(
					T::from(data[0]).unwrap() / T::from(u16::max_value()).unwrap(),
					T::from(data[1]).unwrap() / T::from(u16::max_value()).unwrap()
				)
			}
		}
	);

	(u16 3 -> $ty:ident) => (
		impl<T: Float + 'static> Read<u16> for $ty<T> {
			#[inline]
			fn read(data: &[u16]) -> Self {
				$ty::new(
					T::from(data[0]).unwrap() / T::from(u16::max_value()).unwrap(),
					T::from(data[1]).unwrap() / T::from(u16::max_value()).unwrap(),
					T::from(data[2]).unwrap() / T::from(u16::max_value()).unwrap(),
				)
			}
		}
	);

	(u16 4 -> $ty:ident) => (
		impl<T: Float + 'static> Read<u16> for $ty<T> {
			#[inline]
			fn read(data: &[u16]) -> Self {
				$ty::new(
					T::from(data[0]).unwrap() / T::from(u16::max_value()).unwrap(),
					T::from(data[1]).unwrap() / T::from(u16::max_value()).unwrap(),
					T::from(data[2]).unwrap() / T::from(u16::max_value()).unwrap(),
					T::from(data[3]).unwrap() / T::from(u16::max_value()).unwrap(),
				)
			}
		}
	);

	($ch:ident 1 -> $ty:ident) => (
		impl Read<$ch> for $ty<$ch> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new(data[0])
			}
		}
	);

	($ch:ident 2 -> $ty:ident) => (
		impl Read<$ch> for $ty<$ch> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new(data[0], data[1])
			}
		}
	);

	($ch:ident 3 -> $ty:ident) => (
		impl Read<$ch> for $ty<$ch> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new(data[0], data[1], data[2])
			}
		}
	);

	($ch:ident 4 -> $ty:ident) => (
		impl Read<$ch> for $ty<$ch> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new(data[0], data[1], data[2], data[3])
			}
		}
	);

	($ch:ident $n:tt -> $ty:ident, $($rest:ident),*) => (
		impl_for!($ch $n -> $ty);
		impl_for!($ch $n -> $($rest),*);
	);

	(hue($hue:ident) $ch:ident 3 -> $ty:ident) => (
		impl Read<$ch> for $ty<$ch> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new($hue::from_radians(data[0]), data[1], data[2])
			}
		}
	);

	(hue($hue:ident) $ch:ident 4 -> $ty:ident) => (
		impl Read<$ch> for $ty<$ch> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new($hue::from_radians(data[0]), data[1], data[2], data[3])
			}
		}
	);

	(hue($hue:ident) $ch:ident $n:tt -> $ty:ident, $($rest:ident),*) => (
		impl_for!(hue($hue) $ch $n -> $ty);
		impl_for!(hue($hue) $ch $n -> $($rest),*);
	);
}

impl_for!(u8 1 -> Luma);
impl_for!(u8 2 -> Lumaa);
impl_for!(u8 3 -> Rgb, Srgb);
impl_for!(u8 4 -> Rgba);

impl_for!(u16 1 -> Luma);
impl_for!(u16 2 -> Lumaa);
impl_for!(u16 3 -> Rgb, Srgb);
impl_for!(u16 4 -> Rgba);

impl_for!(f32 1 -> Luma);
impl_for!(f32 2 -> Lumaa);
impl_for!(f32 3 -> Rgb, Srgb, Lab, Xyz, Yxy);
impl_for!(f32 4 -> Rgba, Laba, Xyza, Yxya);

impl_for!(f64 1 -> Luma);
impl_for!(f64 2 -> Lumaa);
impl_for!(f64 3 -> Rgb, Srgb, Lab, Xyz, Yxy);
impl_for!(f64 4 -> Rgba, Laba, Xyza, Yxya);

impl_for!(hue(RgbHue) f32 3 -> Hsl, Hsv, Hwb);
impl_for!(hue(RgbHue) f32 4 -> Hsla, Hsva, Hwba);

impl_for!(hue(RgbHue) f64 3 -> Hsl, Hsv, Hwb);
impl_for!(hue(RgbHue) f64 4 -> Hsla, Hsva, Hwba);

/// A `Pixel` writable to a slice.
pub trait Write<C: Channel> {
	/// Write the `Pixel` to the slice.
	fn write(&self, data: &mut [C]);
}

macro_rules! impl_for {
	($ty:ident -> $a:ident) => (
		impl<C: Channel, T: Float + 'static> Write<C> for $ty<T> {
			#[inline]
			fn write(&self, data: &mut [C]) {
				data[0] = C::from(self.$a);
			}
		}
	);

	($ty:ident -> $a:ident, $b:ident) => (
		impl<C: Channel, T: Float + 'static> Write<C> for $ty<T> {
			#[inline]
			fn write(&self, data: &mut [C]) {
				data[0] = C::from(self.$a);
				data[1] = C::from(self.$b);
			}
		}
	);

	($ty:ident -> $a:ident, $b:ident, $c:ident) => (
		impl<C: Channel, T: Float + 'static> Write<C> for $ty<T> {
			#[inline]
			fn write(&self, data: &mut [C]) {
				data[0] = C::from(self.$a);
				data[1] = C::from(self.$b);
				data[2] = C::from(self.$c);
			}
		}
	);

	($ty:ident -> $a:ident, $b:ident, $c:ident, $d:ident) => (
		impl<C: Channel, T: Float + 'static> Write<C> for $ty<T> {
			#[inline]
			fn write(&self, data: &mut [C]) {
				data[0] = C::from(self.$a);
				data[1] = C::from(self.$b);
				data[2] = C::from(self.$c);
				data[3] = C::from(self.$d);
			}
		}
	);

	($ty:ident -> $a:ident($hue:ident), $b:ident, $c:ident) => (
		impl<C: Channel, T: Float + 'static> Write<C> for $ty<T> {
			#[inline]
			fn write(&self, data: &mut [C]) {
				data[0] = C::from(self.$a.to_radians());
				data[1] = C::from(self.$b);
				data[2] = C::from(self.$c);
			}
		}
	);

	($ty:ident -> $a:ident($hue:ident), $b:ident, $c:ident, $d:ident) => (
		impl<C: Channel, T: Float + 'static> Write<C> for $ty<T> {
			#[inline]
			fn write(&self, data: &mut [C]) {
				data[0] = C::from(self.$a.to_radians());
				data[1] = C::from(self.$b);
				data[2] = C::from(self.$c);
				data[3] = C::from(self.$d);
			}
		}
	);
}

impl_for!(Luma -> luma);
impl_for!(Lumaa -> luma, alpha);

impl_for!(Rgb -> red, green, blue);
impl_for!(Srgb -> red, green, blue);
impl_for!(Lab -> l, a, b);
impl_for!(Xyz -> x, y, z);
impl_for!(Yxy -> x, y, luma);

impl_for!(Rgba -> red, green, blue, alpha);
impl_for!(Laba -> l, a, b, alpha);
impl_for!(Xyza -> x, y, z, alpha);
impl_for!(Yxya -> x, y, luma, alpha);

impl_for!(Hsl -> hue(RgbHue), saturation, lightness);
impl_for!(Hsv -> hue(RgbHue), saturation, value);
impl_for!(Hwb -> hue(RgbHue), whiteness, blackness);

impl_for!(Hsla -> hue(RgbHue), saturation, lightness, alpha);
impl_for!(Hsva -> hue(RgbHue), saturation, value, alpha);
impl_for!(Hwba -> hue(RgbHue), whiteness, blackness, alpha);
