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

use view;
use buffer::Buffer;
use pixel::{self, Pixel};
use processing::util::GetClamped;

pub struct Nearest;

impl<CI, PI, CO, PO> super::Scaler<CI, PI, CO, PO> for Nearest
	where CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      CO: pixel::Channel,
	      PO: Pixel<CO> + pixel::Write<CO>,
	      PO: From<PI>
{
	#[inline]
	fn scale(input: &view::Ref<CI, PI>, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>> {
		let mut output = Buffer::<CO, PO, _>::new(width, height);

		for (x, y) in output.area().absolute() {
			let v = y as f32 / (height - 1) as f32;
			let u = x as f32 / (width - 1) as f32;

			output.set(x, y, &input.get_clamped(
				(u * input.width() as f32) as i64,
				(v * input.height() as f32) as i64
			).into());
		}

		output
	}
}
