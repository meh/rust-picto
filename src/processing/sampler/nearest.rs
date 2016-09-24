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
use pixel::{self, Pixel};
use processing::util::GetClamped;

pub struct Nearest;

impl<CI, PI, CO, PO> super::Sampler<CI, PI, CO, PO> for Nearest
	where CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      CO: pixel::Channel,
	      PO: Pixel<CO> + pixel::Write<CO>,
	      PO: From<PI>
{
	#[inline]
	fn sample(from: &view::Ref<CI, PI>, u: f32, v: f32) -> PO {
		let width  = from.width() as f32;
		let height = from.height() as f32;

		from.get_clamped((u * width) as i64, (v * height) as i64).into()
	}
}
