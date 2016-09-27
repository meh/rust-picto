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
use pixel;
use view;
use super::Scaler;

/// Trait for scalable types.
pub trait Scale<CI, PI>
	where CI: pixel::Channel,
	      PI: pixel::Read<CI>
{
	/// Resize to the given width and height.
	fn resize<A, CO, PO>(self, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>
		where A:  Scaler<CI, PI, CO, PO>,
		      CO: pixel::Channel,
		      PO: pixel::Write<CO>,
		      PO: From<PI>;

	/// Scale by the given factor.
	fn scale_by<A, CO, PO>(self, factor: f32) -> Buffer<CO, PO, Vec<CO>>
		where A:  Scaler<CI, PI, CO, PO>,
		      CO: pixel::Channel,
		      PO: pixel::Write<CO>,
		      PO: From<PI>;

	/// Scale to the given width and height, maintaining the aspect ratio.
	fn scale_to<A, CO, PO>(self, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>
		where A:  Scaler<CI, PI, CO, PO>,
		      CO: pixel::Channel,
		      PO: pixel::Write<CO>,
		      PO: From<PI>;
}

impl<'i, CI, PI, I> Scale<CI, PI> for I
	where CI: pixel::Channel,
	      PI: pixel::Read<CI>,
	      I:  Into<view::Read<'i, CI, PI>>
{
	#[inline]
	fn resize<A, CO, PO>(self, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>
		where A:  Scaler<CI, PI, CO, PO>,
		      CO: pixel::Channel,
		      PO: pixel::Write<CO>,
		      PO: From<PI>
	{
		resize::<A, CO, PO, CI, PI, I>(self, width, height)
	}

	#[inline]
	fn scale_by<A, CO, PO>(self, factor: f32) -> Buffer<CO, PO, Vec<CO>>
		where A:  Scaler<CI, PI, CO, PO>,
		      CO: pixel::Channel,
		      PO: pixel::Write<CO>,
		      PO: From<PI>
	{
		by::<A, CO, PO, CI, PI, I>(self, factor)
	}

	#[inline]
	fn scale_to<A, CO, PO>(self, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>
		where A:  Scaler<CI, PI, CO, PO>,
		      CO: pixel::Channel,
		      PO: pixel::Write<CO>,
		      PO: From<PI>
	{
		to::<A, CO, PO, CI, PI, I>(self, width, height)
	}
}

/// Resize to the given width and height.
#[inline]
pub fn resize<'i, A, CO, PO, CI, PI, I>(input: I, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>
	where A:  Scaler<CI, PI, CO, PO>,
	      CO: pixel::Channel,
	      PO: pixel::Write<CO>,
	      PO: From<PI>,
	      CI: pixel::Channel,
	      PI: pixel::Read<CI>,
	      I:  Into<view::Read<'i, CI, PI>>
{
	let input = input.into();

	if input.width() == width && input.height() == height {
		return input.convert::<CO, PO>();
	}

	A::scale(&input.into(), width, height)
}

/// Scale by the given factor.
#[inline]
pub fn by<'i, A, CO, PO, CI, PI, I>(input: I, factor: f32) -> Buffer<CO, PO, Vec<CO>>
	where A:  Scaler<CI, PI, CO, PO>,
	      CO: pixel::Channel,
	      PO: pixel::Write<CO>,
	      PO: From<PI>,
	      CI: pixel::Channel,
	      PI: pixel::Read<CI>,
	      I:  Into<view::Read<'i, CI, PI>>
{
	let input  = input.into();
	let width  = input.width() as f32 * factor;
	let height = input.height() as f32 * factor;

	resize::<A, CO, PO, CI, PI, _>(input, width as u32, height as u32)
}

/// Scale to the given width and height, maintaining the aspect ratio.
#[inline]
pub fn to<'i, A, CO, PO, CI, PI, I>(input: I, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>
	where A:  Scaler<CI, PI, CO, PO>,
	      CO: pixel::Channel,
	      PO: pixel::Write<CO>,
	      PO: From<PI>,
	      CI: pixel::Channel,
	      PI: pixel::Read<CI>,
	      I:  Into<view::Read<'i, CI, PI>>
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

	let width  = input.width() as f32 * scale;
	let height = input.height() as f32 * scale;

	resize::<A, CO, PO, CI, PI, _>(input, width as u32, height as u32)
}

#[cfg(test)]
mod test {
	use super::*;
	use processing::scaler::Nearest;
	use buffer::Buffer;
	use color::Rgb;

	#[test]
	fn nearest() {
		let mut buffer = Buffer::<u8, Rgb, _>::new(2, 2);

		buffer.set(0, 0, &Rgb::new(1.0, 0.0, 0.0));
		buffer.set(1, 0, &Rgb::new(0.0, 1.0, 0.0));
		buffer.set(0, 1, &Rgb::new(0.0, 0.0, 1.0));
		buffer.set(1, 1, &Rgb::new(1.0, 0.0, 1.0));

		let result = buffer.resize::<Nearest, u8, Rgb>(4, 4);

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
