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

use pixel;
use view;
use color::{Limited, Rgba};
use super::Sampler;
use super::util::GetClamped;

#[inline]
pub fn vertically<'i, 'o, A, CO, PO, CI, PI, I, O>(input: I, output: O)
	where A:  Sampler,
	      CO: pixel::Channel,
	      PO: pixel::Write<CO>,
	      PO: From<Rgba>,
	      CI: pixel::Channel,
	      PI: pixel::Read<CI>,
	      PI: Into<Rgba>,
	      I:  Into<view::Read<'i, CI, PI>>,
	      O:  Into<view::Write<'o, CO, PO>>
{
	vertically_with(input, output, A::support(), A::kernel)
}

pub fn vertically_with<'i, 'o, CO, PO, CI, PI, I, O, F>(input: I, output: O, support: f32, mut kernel: F)
	where CO: pixel::Channel,
	      PO: pixel::Write<CO>,
	      PO: From<Rgba>,
	      CI: pixel::Channel,
	      PI: pixel::Read<CI>,
	      PI: Into<Rgba>,
	      I:  Into<view::Read<'i, CI, PI>>,
	      O:  Into<view::Write<'o, CO, PO>>,
	      F:  FnMut(f32) -> f32
{
	let     input  = input.into();
	let     input  = &input;
	let mut output = output.into();

	debug_assert_eq!(input.width(), output.width());

	let ratio  = input.height() as f32 / output.height() as f32;
	let scale  = if ratio > 1.0 { ratio } else { 1.0 };
	let radius = (support * scale).ceil();

	for x in 0 .. input.width() as i64 {
		for y_out in 0 .. output.height() {
			let y_in = (y_out as f32 + 0.5) * ratio;

			let left  = (y_in - radius) as i64;
			let right = (y_in + radius) as i64;

			let mut sum = (0.0, 0.0, 0.0, 0.0);
			let mut t   = (0.0, 0.0, 0.0, 0.0);

			for i in left .. right + 1 {
				let w = kernel((i as f32 - y_in) / scale);
				let w = (w, w, w, w);

				sum.0 += w.0;
				sum.1 += w.1;
				sum.2 += w.2;
				sum.3 += w.3;

				let p: (f32, f32, f32, f32) = input.get_clamped(x, i).into().to_pixel();

				t.0 += p.0 * w.0;
				t.1 += p.1 * w.1;
				t.2 += p.2 * w.2;
				t.3 += p.3 * w.3;
			}

			output.set(x as u32, y_out, &Rgba::new(t.0 / sum.0, t.1 / sum.1, t.2 / sum.2, t.3 / sum.3).clamp().into());
		}
	}
}

#[inline]
pub fn horizontally<'i, 'o, A, CO, PO, CI, PI, I, O>(input: I, output: O)
	where A:  Sampler,
	      CO: pixel::Channel,
	      PO: pixel::Write<CO>,
	      PO: From<Rgba>,
	      CI: pixel::Channel,
	      PI: pixel::Read<CI>,
	      PI: Into<Rgba>,
	      I:  Into<view::Read<'i, CI, PI>>,
	      O:  Into<view::Write<'o, CO, PO>>
{
	horizontally_with(input, output, A::support(), A::kernel)
}

pub fn horizontally_with<'i, 'o, CO, PO, CI, PI, I, O, F>(input: I, output: O, support: f32, mut kernel: F)
	where CO: pixel::Channel,
	      PO: pixel::Write<CO>,
	      PO: From<Rgba>,
	      CI: pixel::Channel,
	      PI: pixel::Read<CI>,
	      PI: Into<Rgba>,
	      I:  Into<view::Read<'i, CI, PI>>,
	      O:  Into<view::Write<'o, CO, PO>>,
	      F:  FnMut(f32) -> f32
{
	let     input  = input.into();
	let     input  = &input;
	let mut output = output.into();

	debug_assert_eq!(input.height(), output.height());

	let ratio  = input.width() as f32 / output.width() as f32;
	let scale  = if ratio > 1.0 { ratio } else { 1.0 };
	let radius = (support * scale).ceil();

	for y in 0 .. input.height() as i64 {
		for x_out in 0 .. output.width() {
			let x_in = (x_out as f32 + 0.5) * ratio;

			let left  = (x_in - radius) as i64;
			let right = (x_in + radius) as i64;

			let mut sum = (0.0, 0.0, 0.0, 0.0);
			let mut t   = (0.0, 0.0, 0.0, 0.0);

			for i in left .. right + 1 {
				let w = kernel((i as f32 - x_in) / scale);
				let w = (w, w, w, w);

				sum.0 += w.0;
				sum.1 += w.1;
				sum.2 += w.2;
				sum.3 += w.3;

				let p: (f32, f32, f32, f32) = input.get_clamped(i, y).into().to_pixel();

				t.0 += p.0 * w.0;
				t.1 += p.1 * w.1;
				t.2 += p.2 * w.2;
				t.3 += p.3 * w.3;
			}

			output.set(x_out, y as u32, &Rgba::new(t.0 / sum.0, t.1 / sum.1, t.2 / sum.2, t.3 / sum.3).clamp().into());
		}
	}
}
