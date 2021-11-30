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

use super::Scaler;
use crate::{buffer::Buffer, pixel, view};

/// Trait for scalable types.
pub trait Scale<P, C>
where
	P: pixel::Read<C> + pixel::Write<C>,
	C: pixel::Channel,
{
	/// Resize to the given width and height.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::color::Rgb;
	/// use picto::processing::prelude::*;
	///
	/// let image   = read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();
	/// let resized = image.resize::<scaler::Lanczos3>(100, 100);
	///
	/// assert_eq!(resized.width(), 100);
	/// assert_eq!(resized.height(), 100);
	/// ```
	fn resize<A>(self, width: u32, height: u32) -> Buffer<P, C, Vec<C>>
	where
		A: Scaler<P, C, P, C>;

	/// Scale by the given factor.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::color::Rgb;
	/// use picto::processing::prelude::*;
	///
	/// let image   = read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();
	/// let resized = image.scale_by::<scaler::Lanczos3>(0.5);
	///
	/// assert_eq!(resized.width(), 160);
	/// assert_eq!(resized.height(), 120);
	/// ```
	fn scale_by<A>(self, factor: f32) -> Buffer<P, C, Vec<C>>
	where
		A: Scaler<P, C, P, C>;

	/// Scale to the given width and height, maintaining the aspect ratio.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::color::Rgb;
	/// use picto::processing::prelude::*;
	///
	/// let image   = read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();
	/// let resized = image.scale_to::<scaler::Lanczos3>(160, 160);
	///
	/// assert_eq!(resized.width(), 160);
	/// assert_eq!(resized.height(), 120);
	/// ```
	fn scale_to<A>(self, width: u32, height: u32) -> Buffer<P, C, Vec<C>>
	where
		A: Scaler<P, C, P, C>;
}

impl<'i, P, C, I> Scale<P, C> for I
where
	P: pixel::Read<C> + pixel::Write<C>,
	C: pixel::Channel,
	I: Into<view::Read<'i, P, C>>,
{
	#[inline]
	fn resize<A>(self, width: u32, height: u32) -> Buffer<P, C, Vec<C>>
	where
		A: Scaler<P, C, P, C>,
	{
		resize::<A, _, P, C, P, C>(self, width, height)
	}

	#[inline]
	fn scale_by<A>(self, factor: f32) -> Buffer<P, C, Vec<C>>
	where
		A: Scaler<P, C, P, C>,
	{
		by::<A, _, P, C, P, C>(self, factor)
	}

	#[inline]
	fn scale_to<A>(self, width: u32, height: u32) -> Buffer<P, C, Vec<C>>
	where
		A: Scaler<P, C, P, C>,
	{
		to::<A, _, P, C, P, C>(self, width, height)
	}
}

/// Resize to the given width and height.
#[inline]
pub fn resize<'i, A, I, PI, CI, PO, CO>(input: I, width: u32, height: u32) -> Buffer<PO, CO, Vec<CO>>
where
	A: Scaler<PI, CI, PO, CO>,
	PO: From<PI>,
	PO: pixel::Write<CO>,
	CO: pixel::Channel,
	PI: pixel::Read<CI>,
	CI: pixel::Channel,
	I: Into<view::Read<'i, PI, CI>>,
{
	let input = input.into();

	if input.width() == width && input.height() == height {
		return input.convert::<PO, CO>();
	}

	A::scale(&input.into(), width, height)
}

/// Scale by the given factor.
#[inline]
pub fn by<'i, A, I, PI, CI, PO, CO>(input: I, factor: f32) -> Buffer<PO, CO, Vec<CO>>
where
	A: Scaler<PI, CI, PO, CO>,
	PO: From<PI>,
	PO: pixel::Write<CO>,
	CO: pixel::Channel,
	PI: pixel::Read<CI>,
	CI: pixel::Channel,
	I: Into<view::Read<'i, PI, CI>>,
{
	let input = input.into();
	let width = input.width() as f32 * factor;
	let height = input.height() as f32 * factor;

	resize::<A, _, PI, CI, PO, CO>(input, width as u32, height as u32)
}

/// Scale to the given width and height, maintaining the aspect ratio.
#[inline]
pub fn to<'i, A, I, PI, CI, PO, CO>(input: I, width: u32, height: u32) -> Buffer<PO, CO, Vec<CO>>
where
	A: Scaler<PI, CI, PO, CO>,
	PO: From<PI>,
	PO: pixel::Write<CO>,
	CO: pixel::Channel,
	PI: pixel::Read<CI>,
	CI: pixel::Channel,
	I: Into<view::Read<'i, PI, CI>>,
{
	let input = input.into();
	let r_old = input.width() as f32 / input.height() as f32;
	let r_new = width as f32 / height as f32;

	let scale = if r_new > r_old {
		height as f32 / input.height() as f32
	}
	else {
		width as f32 / input.width() as f32
	};

	let width = input.width() as f32 * scale;
	let height = input.height() as f32 * scale;

	resize::<A, _, PI, CI, PO, CO>(input, width as u32, height as u32)
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{buffer, color::Rgb, processing::scaler::Nearest};

	#[test]
	fn nearest() {
		let mut buffer = buffer::Rgb::new(2, 2);

		buffer.set(0, 0, &Rgb::new(1.0, 0.0, 0.0));
		buffer.set(1, 0, &Rgb::new(0.0, 1.0, 0.0));
		buffer.set(0, 1, &Rgb::new(0.0, 0.0, 1.0));
		buffer.set(1, 1, &Rgb::new(1.0, 0.0, 1.0));

		let result = buffer.resize::<Nearest>(4, 4);

		assert_eq!(Rgb::new(1.0, 0.0, 0.0), result.get(0, 0));
		assert_eq!(Rgb::new(1.0, 0.0, 0.0), result.get(1, 0));
		assert_eq!(Rgb::new(1.0, 0.0, 0.0), result.get(0, 1));
		assert_eq!(Rgb::new(1.0, 0.0, 0.0), result.get(1, 1));

		assert_eq!(Rgb::new(0.0, 1.0, 0.0), result.get(2, 0));
		assert_eq!(Rgb::new(0.0, 1.0, 0.0), result.get(3, 0));
		assert_eq!(Rgb::new(0.0, 1.0, 0.0), result.get(2, 1));
		assert_eq!(Rgb::new(0.0, 1.0, 0.0), result.get(3, 1));

		assert_eq!(Rgb::new(0.0, 0.0, 1.0), result.get(0, 2));
		assert_eq!(Rgb::new(0.0, 0.0, 1.0), result.get(1, 2));
		assert_eq!(Rgb::new(0.0, 0.0, 1.0), result.get(0, 3));
		assert_eq!(Rgb::new(0.0, 0.0, 1.0), result.get(1, 3));

		assert_eq!(Rgb::new(1.0, 0.0, 1.0), result.get(2, 2));
		assert_eq!(Rgb::new(1.0, 0.0, 1.0), result.get(3, 2));
		assert_eq!(Rgb::new(1.0, 0.0, 1.0), result.get(2, 3));
		assert_eq!(Rgb::new(1.0, 0.0, 1.0), result.get(3, 3));
	}
}
