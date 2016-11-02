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
use pixel;
use processing::util::GetClamped;

pub struct Nearest;

impl<PI, CI, PO, CO> super::Scaler<PI, CI, PO, CO> for Nearest
	where PI: pixel::Read<CI>,
	      CI: pixel::Channel,
	      PO: From<PI>,
	      PO: pixel::Write<CO>,
	      CO: pixel::Channel,
{
	#[inline]
	fn scale(input: &view::Read<PI, CI>, width: u32, height: u32) -> Buffer<PO, CO, Vec<CO>> {
		let mut output = Buffer::<PO, CO, _>::new(width, height);

		for (x, y) in output.region().absolute() {
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
