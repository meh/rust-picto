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

use num::{Float, Zero};

use color::{Luma, Rgb, Hsl, Hsv, Hwb, Lab, Lch, Xyz, Yxy};
use color::{Lumaa, Rgba, Hsla, Hsva, Hwba, Laba, Lcha, Xyza, Yxya};
use color::pixel::Srgb;
use color::RgbHue;

/// Trait for the `Channel` within a `Pixel`, this is typically the primitive
/// type within a `Buffer` as well.
pub trait Channel: Zero + Copy + 'static {
	fn from<T: Float + 'static>(value: T) -> Self;
}

impl Channel for u8 {
	fn from<T: Float + 'static>(value: T) -> Self {
		num!(value * num!(u8::max_value()))
	}
}

impl Channel for u16 {
	fn from<T: Float + 'static>(value: T) -> Self {
		num!(value * num!(u16::max_value()))
	}
}

impl Channel for u32 {
	fn from<T: Float + 'static>(value: T) -> Self {
		num!(value * num!(u32::max_value()))
	}
}

impl Channel for f32 {
	fn from<T: Float + 'static>(value: T) -> Self {
		num!(value)
	}
}

impl Channel for f64 {
	fn from<T: Float + 'static>(value: T) -> Self {
		num!(value)
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
pub trait Read<C: Channel>: Pixel<C> {
	/// Read a `Pixel` from the slice.
	fn read(data: &[C]) -> Self;
}

macro_rules! impl_for {
	(integer $ch:ident 1 -> $ty:ident) => (
		impl<T: Float + 'static> Read<$ch> for $ty<T> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new(
					num!(data[0] => T) / num!($ch::max_value())
				)
			}
		}
	);

	(integer $ch:ident 2 -> $ty:ident) => (
		impl<T: Float + 'static> Read<$ch> for $ty<T> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new(
					num!(data[0] => T) / num!($ch::max_value()),
					num!(data[1] => T) / num!($ch::max_value())
				)
			}
		}
	);

	(integer $ch:ident 3 -> $ty:ident) => (
		impl<T: Float + 'static> Read<$ch> for $ty<T> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new(
					num!(data[0] => T) / num!($ch::max_value()),
					num!(data[1] => T) / num!($ch::max_value()),
					num!(data[2] => T) / num!($ch::max_value()),
				)
			}
		}
	);

	(integer $ch:ident 4 -> $ty:ident) => (
		impl<T: Float + 'static> Read<$ch> for $ty<T> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new(
					num!(data[0] => T) / num!($ch::max_value()),
					num!(data[1] => T) / num!($ch::max_value()),
					num!(data[2] => T) / num!($ch::max_value()),
					num!(data[3] => T) / num!($ch::max_value()),
				)
			}
		}
	);

	(float $ch:ident 1 -> $ty:ident) => (
		impl<T: Float + 'static> Read<$ch> for $ty<T> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new(num!(data[0]))
			}
		}
	);

	(float $ch:ident 2 -> $ty:ident) => (
		impl<T: Float + 'static> Read<$ch> for $ty<T> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new(num!(data[0]), num!(data[1]))
			}
		}
	);

	(float $ch:ident 3 -> $ty:ident) => (
		impl<T: Float + 'static> Read<$ch> for $ty<T> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new(num!(data[0]), num!(data[1]), num!(data[2]))
			}
		}
	);

	(float $ch:ident 4 -> $ty:ident) => (
		impl<T: Float + 'static> Read<$ch> for $ty<T> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new(num!(data[0]), num!(data[1]), num!(data[2]), num!(data[3]))
			}
		}
	);

	(u8  $n:tt -> $ty:ident) => (impl_for!(integer u8  $n -> $ty););
	(u16 $n:tt -> $ty:ident) => (impl_for!(integer u16 $n -> $ty););
	(u32 $n:tt -> $ty:ident) => (impl_for!(integer u32 $n -> $ty););

	(f32 $n:tt -> $ty:ident) => (impl_for!(float f32 $n -> $ty););
	(f64 $n:tt -> $ty:ident) => (impl_for!(float f64 $n -> $ty););

	($ch:ident $n:tt -> $ty:ident, $($rest:ident),*) => (
		impl_for!($ch $n -> $ty);
		impl_for!($ch $n -> $($rest),*);
	);

	(hue($hue:ident) $ch:ident 3 -> $ty:ident) => (
		impl<T: Float + 'static> Read<$ch> for $ty<T> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new($hue::from_radians(num!(data[0])), num!(data[1]), num!(data[2]))
			}
		}
	);

	(hue($hue:ident) $ch:ident 4 -> $ty:ident) => (
		impl<T: Float + 'static> Read<$ch> for $ty<T> {
			#[inline]
			fn read(data: &[$ch]) -> Self {
				$ty::new($hue::from_radians(num!(data[0])), num!(data[1]), num!(data[2]), num!(data[3]))
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
impl_for!(u8 3 -> Rgb, Srgb, Lab, Xyz, Yxy);
impl_for!(u8 4 -> Rgba, Laba, Xyza, Yxya);

impl_for!(u16 1 -> Luma);
impl_for!(u16 2 -> Lumaa);
impl_for!(u16 3 -> Rgb, Srgb, Lab, Xyz, Yxy);
impl_for!(u16 4 -> Rgba, Laba, Xyza, Yxya);

impl_for!(u32 1 -> Luma);
impl_for!(u32 2 -> Lumaa);
impl_for!(u32 3 -> Rgb, Srgb, Lab, Xyz, Yxy);
impl_for!(u32 4 -> Rgba, Laba, Xyza, Yxya);

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
pub trait Write<C: Channel>: Pixel<C> {
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
