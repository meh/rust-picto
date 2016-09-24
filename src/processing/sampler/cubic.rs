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

pub struct Cubic;

impl<CI, PI, CO, PO> super::Sampler<CI, PI, CO, PO> for Cubic
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

		let p00 = from.get_clamped(x - 1, y - 1).into();
		let p10 = from.get_clamped(x + 0, y - 1).into();
		let p20 = from.get_clamped(x + 1, y - 1).into();
		let p30 = from.get_clamped(x + 2, y - 1).into();

		let p01 = from.get_clamped(x - 1, y + 0).into();
		let p11 = from.get_clamped(x + 0, y + 0).into();
		let p21 = from.get_clamped(x + 1, y + 0).into();
		let p31 = from.get_clamped(x + 2, y + 0).into();

		let p02 = from.get_clamped(x - 1, y + 1).into();
		let p12 = from.get_clamped(x + 0, y + 1).into();
		let p22 = from.get_clamped(x + 1, y + 1).into();
		let p32 = from.get_clamped(x + 2, y + 1).into();

		let p03 = from.get_clamped(x - 1, y + 2).into();
		let p13 = from.get_clamped(x + 0, y + 2).into();
		let p23 = from.get_clamped(x + 1, y + 2).into();
		let p33 = from.get_clamped(x + 2, y + 2).into();

		let c0 = hermite(&p00, &p10, &p20, &p30, x_fract);
		let c1 = hermite(&p01, &p11, &p21, &p31, x_fract);
		let c2 = hermite(&p02, &p12, &p22, &p32, x_fract);
		let c3 = hermite(&p03, &p13, &p23, &p33, x_fract);

		hermite(&c0, &c1, &c2, &c3, y_fract).into()
	}
}

#[inline]
#[allow(non_snake_case)]
fn hermite(A: &Rgba, B: &Rgba, C: &Rgba, D: &Rgba, t: f32) -> Rgba {
	#[inline(always)]
	fn it(A: f32, B: f32, C: f32, D: f32, t: f32) -> f32 {
    let a = (-A / 2.0) + ((3.0 * B) / 2.0) - ((3.0 * C) / 2.0) + (D / 2.0);
    let b = A - ((5.0 * B) / 2.0) + (2.0 * C) - (D / 2.0);
    let c = (-A / 2.0) + (C / 2.0);
    let d = B;

		(a * t * t * t) + (b * t * t) + (c * t) + d
	}

	Rgba::new(
		it(A.red, B.red, C.red, D.red, t),
		it(A.green, B.green, C.green, D.green, t),
		it(A.blue, B.blue, C.blue, D.blue, t),
		it(A.alpha, B.alpha, C.alpha, D.alpha, t),
	).clamp()
}
