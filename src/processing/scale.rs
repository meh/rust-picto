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
use view;

use processing::sampler::Sampler;

pub trait Scale<CI, PI>
	where CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
{
	fn resize<A, CO, PO>(self, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>
		where A:  Sampler<CI, PI, CO, PO>,
		      CO: pixel::Channel,
		      PO: Pixel<CO> + pixel::Write<CO>;

	fn scale_by<A, CO, PO>(self, factor: f32) -> Buffer<CO, PO, Vec<CO>>
		where A:  Sampler<CI, PI, CO, PO>,
		      CO: pixel::Channel,
		      PO: Pixel<CO> + pixel::Write<CO>;

	fn scale_to<A, CO, PO>(self, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>
		where A:  Sampler<CI, PI, CO, PO>,
		      CO: pixel::Channel,
		      PO: Pixel<CO> + pixel::Write<CO>;
}

impl<'i, CI, PI, I> Scale<CI, PI> for I
	where CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      I:  Into<view::Ref<'i, CI, PI>>
{
	#[inline]
	fn resize<A, CO, PO>(self, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>
		where A:  Sampler<CI, PI, CO, PO>,
		      CO: pixel::Channel,
		      PO: Pixel<CO> + pixel::Write<CO>
	{
		resize::<A, CO, PO, CI, PI, I>(self, width, height)
	}

	#[inline]
	fn scale_by<A, CO, PO>(self, factor: f32) -> Buffer<CO, PO, Vec<CO>>
		where A:  Sampler<CI, PI, CO, PO>,
		      CO: pixel::Channel,
		      PO: Pixel<CO> + pixel::Write<CO>
	{
		by::<A, CO, PO, CI, PI, I>(self, factor)
	}

	#[inline]
	fn scale_to<A, CO, PO>(self, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>
		where A:  Sampler<CI, PI, CO, PO>,
		      CO: pixel::Channel,
		      PO: Pixel<CO> + pixel::Write<CO>
	{
		to::<A, CO, PO, CI, PI, I>(self, width, height)
	}
}

pub fn resize<'i, A, CO, PO, CI, PI, I>(input: I, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>
	where A:  Sampler<CI, PI, CO, PO>,
	      CO: pixel::Channel,
	      PO: Pixel<CO> + pixel::Write<CO>,
	      CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      I:  Into<view::Ref<'i, CI, PI>>
{
	let     input  = input.into();
	let mut output = Buffer::<CO, PO, _>::new(width, height);

	for y in 0 .. height {
		let v = y as f32 / (height - 1) as f32;

		for x in 0 .. width {
			let u = x as f32 / (width - 1) as f32;

			output.set(x, y, &A::sample(&input, u, v));
		}
	}

	output
}

#[inline]
pub fn by<'i, A, CO, PO, CI, PI, I>(input: I, factor: f32) -> Buffer<CO, PO, Vec<CO>>
	where A:  Sampler<CI, PI, CO, PO>,
	      CO: pixel::Channel,
	      PO: Pixel<CO> + pixel::Write<CO>,
	      CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      I:  Into<view::Ref<'i, CI, PI>>
{
	let input  = input.into();
	let width  = input.width() as f32 * factor;
	let height = input.height() as f32 * factor;

	resize::<A, CO, PO, CI, PI, _>(input, width as u32, height as u32)
}

#[inline]
pub fn to<'i, A, CO, PO, CI, PI, I>(input: I, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>
	where A:  Sampler<CI, PI, CO, PO>,
	      CO: pixel::Channel,
	      PO: Pixel<CO> + pixel::Write<CO>,
	      CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      I:  Into<view::Ref<'i, CI, PI>>
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

	let width  = width as f32 * scale;
	let height = height as f32 * scale;

	resize::<A, CO, PO, CI, PI, _>(input, width as u32, height as u32)
}

#[cfg(test)]
mod test {
	use super::*;
	use processing::sampler::Nearest;
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
