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
use color::{Limited, Rgba};

pub struct Linear;

impl<CI, PI, CO, PO> super::Sampler<CI, PI, CO, PO> for Linear
	where CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      PI: Into<Rgba>,
	      CO: pixel::Channel,
	      PO: Pixel<CO> + pixel::Write<CO>,
	      PO: From<Rgba>
{
	#[inline]
	fn sample(from: &view::Ref<CI, PI>, u: f32, v: f32) -> PO {
		let x = u * from.width() as f32 - 0.5;
		let y = v * from.height() as f32 - 0.5;

		let x_fract = x - x.floor();
		let y_fract = y - y.floor();

		let x = x as i64;
		let y = y as i64;

		let p00 = from.get_clamped(x + 0, y + 0).into();
		let p01 = from.get_clamped(x + 1, y + 0).into();
		let p10 = from.get_clamped(x + 0, y + 1).into();
		let p11 = from.get_clamped(x + 1, y + 1).into();

		let c0 = lerp(&p00, &p01, x_fract);
		let c1 = lerp(&p10, &p11, x_fract);

		lerp(&c0, &c1, y_fract).into()
	}
}

#[inline]
#[allow(non_snake_case)]
fn lerp(A: &Rgba, B: &Rgba, t: f32) -> Rgba {
	#[inline(always)]
	fn it(A: f32, B: f32, t: f32) -> f32 {
		A * (1.0 - t) + B * t
	}

	Rgba::new(
		it(A.red, B.red, t),
		it(A.green, B.green, t),
		it(A.blue, B.blue, t),
		it(A.alpha, B.alpha, t)
	).clamp()
}
