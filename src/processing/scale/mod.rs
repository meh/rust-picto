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

use num::Float;
use buffer::Buffer;
use pixel::{self, Pixel};
use view;

pub trait Scaler<CI, PI, CO, PO, T: Float = f32>
	where CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      CO: pixel::Channel,
	      PO: Pixel<CO> + pixel::Write<CO>,
	      PO: From<PI>
{
	fn scale(input: view::Ref<CI, PI>, output: view::Mut<CO, PO>);
}

mod nearest;
pub use self::nearest::Nearest;

pub trait Scale<CI, PI>
	where CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
{
	fn resize<A, CO, PO>(self, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>
		where A:  Scaler<CI, PI, CO, PO>,
		      CO: pixel::Channel,
		      PO: Pixel<CO> + pixel::Write<CO>,
		      PO: From<PI>;
}

impl<'i, CI, PI, I> Scale<CI, PI> for I
	where CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      I:  Into<view::Ref<'i, CI, PI>>
{
	fn resize<A, CO, PO>(self, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>
		where A:  Scaler<CI, PI, CO, PO>,
		      CO: pixel::Channel,
		      PO: Pixel<CO> + pixel::Write<CO>,
		      PO: From<PI>
	{
		resize::<A, CO, PO, CI, PI, I>(self, width, height)
	}
}

pub fn resize<'i, A, CO, PO, CI, PI, I>(input: I, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>>
	where A:  Scaler<CI, PI, CO, PO>,
	      CO: pixel::Channel,
	      PO: Pixel<CO> + pixel::Write<CO>,
	      PO: From<PI>,
	      CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      I:  Into<view::Ref<'i, CI, PI>>
{
	let mut result = Buffer::<CO, PO, _>::new(width, height);
	A::scale(input.into(), result.as_mut(Default::default()));

	result
}

#[cfg(test)]
mod test {
	use super::*;
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
