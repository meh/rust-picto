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

use std::ops::Range;

use view;
use buffer::Buffer;
use pixel::{self, Pixel};
use color::{Rgba, Limited};
use processing::Scaler;
use processing::util::{clamp, GetClamped};

pub struct Super;

impl<CI, PI, CO, PO> Scaler<CI, PI, CO, PO> for Super
	where CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      PI: Into<Rgba>,
	      CO: pixel::Channel,
	      PO: Pixel<CO> + pixel::Write<CO> + pixel::Read<CO>,
	      PO: From<Rgba> + Into<Rgba> + From<PI>
{
	#[inline]
	fn scale(input: &view::Ref<CI, PI>, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>> {
		let x_factor = width as f32 / input.width() as f32;
		let y_factor = height as f32 / input.height() as f32;
		let factor   = x_factor as u32;

		debug_assert!(x_factor != y_factor || x_factor.fract() != 0.0 || (factor & (factor - 1)) != 0);

		let mut factor = factor / 2;
		let mut output = scale::<CI, PI, CO, PO>(input);

		while factor >= 2 {
			output  = scale::<CO, PO, CO, PO>(&output.as_ref(Default::default()));
			factor /= 2;
		}

		output
	}
}

#[allow(non_snake_case)]
fn scale<CI, PI, CO, PO>(input: &view::Ref<CI, PI>) -> Buffer<CO, PO, Vec<CO>>
	where CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      PI: Into<Rgba> + Into<PO>,
	      CO: pixel::Channel,
	      PO: Pixel<CO> + pixel::Write<CO> + pixel::Read<CO>,
	      PO: From<Rgba> + Into<Rgba>
{
	const WEIGHT1: f32 = 0.129633;
	const WEIGHT2: f32 = 0.175068;
	const W1:      f32 = -WEIGHT1;
	const W2:      f32 = WEIGHT1 + 0.5;
	const W3:      f32 = -WEIGHT2;
	const W4:      f32 = WEIGHT2 + 0.5;

	let mut output = Buffer::<CO, PO, _>::new(input.width() * 2, input.height() * 2);

	// First pass.
	for y in 0 .. input.height() {
		for x in 0 .. input.width() {
			// Central pixels in source image.
			let cx = x as i64;
			let cy = y as i64;

			// Step by two.
			let x = x * 2;
			let y = y * 2;

			// Sample supporting pixels in source image.
			let (r, g, b, a, Y) = sample(input,
				1, -1 .. 3, |sx, sy| (sx + cx, sy + cy));

			let (r_min, r_max) = minmax(&r);
			let (g_min, g_max) = minmax(&g);
			let (b_min, b_max) = minmax(&b);
			let (a_min, a_max) = minmax(&a);

			let edge         = diagonal(&Y, &[2.0, 1.0, -1.0, 4.0, -1.0, 1.0]);
			let (r, g, b, a) = pick(edge, W1, W2, &r, &g, &b, &a);

			// Anti-ringing clamp.
			let rgba = Rgba::new(
				clamp(r, r_min, r_max),
				clamp(g, g_min, g_max),
				clamp(b, b_min, b_max),
				clamp(a, a_min, a_max)).clamp();

			output.set(x,     y,     &input.get_clamped(cx, cy).into());
			output.set(x + 1, y,     &input.get_clamped(cx, cy).into());
			output.set(x,     y + 1, &input.get_clamped(cx, cy).into());
			output.set(x + 1, y + 1, &rgba.into());
		}
	}

	// Second pass.
	for y in 0 .. input.height() {
		for x in 0 .. input.width() {
			// Step by two.
			let x = x * 2;
			let y = y * 2;

			let (r, g, b, a, Y) = sample(&output.as_ref(Default::default()),
				1, -1 .. 3, |sx, sy| (sx + sy + x as i64, sx - sy + y as i64));

			let (r_min, r_max) = minmax(&r);
			let (g_min, g_max) = minmax(&g);
			let (b_min, b_max) = minmax(&b);
			let (a_min, a_max) = minmax(&a);

			let edge         = diagonal(&Y, &[2.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
			let (r, g, b, a) = pick(edge, W3, W4, &r, &g, &b, &a);

			// Anti-ringing clamp.
			let rgba = Rgba::new(
				clamp(r, r_min, r_max),
				clamp(g, g_min, g_max),
				clamp(b, b_min, b_max),
				clamp(a, a_min, a_max)).clamp();

			output.set(x + 1, y, &rgba.into());

			let (r, g, b, a, Y) = sample(&output.as_ref(Default::default()),
				1, -1 .. 3, |sx, sy| (sx + sy - 1 + x as i64, sx - sy + 1 + y as i64));

			let edge         = diagonal(&Y, &[2.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
			let (r, g, b, a) = pick(edge, W3, W4, &r, &g, &b, &a);

			// Anti-ringing clamp.
			let rgba = Rgba::new(
				clamp(r, r_min, r_max),
				clamp(g, g_min, g_max),
				clamp(b, b_min, b_max),
				clamp(a, a_min, a_max)).clamp();

			output.set(x, y + 1, &rgba.into());
		}
	}

	// Third pass.
	for y in (0 .. input.height() * 2).rev() {
		for x in (0 .. input.width() * 2).rev() {
			let (r, g, b, a, Y) = sample(&output.as_ref(Default::default()),
				2, -2 .. 2, |sx, sy| (sx + x as i64, sy + y as i64));

			let (r_min, r_max) = minmax(&r);
			let (g_min, g_max) = minmax(&g);
			let (b_min, b_max) = minmax(&b);
			let (a_min, a_max) = minmax(&a);

			let edge         = diagonal(&Y, &[2.0, 1.0, -1.0, 4.0, -1.0, 1.0]);
			let (r, g, b, a) = pick(edge, W1, W2, &r, &g, &b, &a);

			// Anti-ringing clamp.
			let rgba = Rgba::new(
				clamp(r, r_min, r_max),
				clamp(g, g_min, g_max),
				clamp(b, b_min, b_max),
				clamp(a, a_min, a_max)).clamp();

			output.set(x, y, &rgba.into());
		}
	}

	output
}

type Matrix = [[f32; 4]; 4];

#[inline]
#[allow(non_snake_case)]
fn sample<CI, PI, F>(input: &view::Ref<CI, PI>, offset: i64, range: Range<i64>, func: F) -> (Matrix, Matrix, Matrix, Matrix, Matrix)
	where CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      PI: Into<Rgba>,
	      F:  Fn(i64, i64) -> (i64, i64)
{
	let mut r = [[0.0f32; 4]; 4];
	let mut g = [[0.0f32; 4]; 4];
	let mut b = [[0.0f32; 4]; 4];
	let mut a = [[0.0f32; 4]; 4];
	let mut Y = [[0.0f32; 4]; 4];

	// Sample supporting pixels in source image.
	for sx in range.start .. range.end {
		for sy in range.start .. range.end {
			let (x, y) = func(sx, sy);
			let p      = input.get_clamped(x, y).into();

			// Add weighted components.
			r[(sx + offset) as usize][(sy + offset) as usize] = p.red;
			g[(sx + offset) as usize][(sy + offset) as usize] = p.green;
			b[(sx + offset) as usize][(sy + offset) as usize] = p.blue;
			a[(sx + offset) as usize][(sy + offset) as usize] = p.alpha;
			Y[(sx + offset) as usize][(sy + offset) as usize] = (0.2126 * p.red) + (0.7152 * p.green) + (0.0722 * p.blue);
		}
	}

	(r, g, b, a, Y)
}

#[inline]
fn minmax(m: &Matrix) -> (f32, f32) {
	(m[1][1].min(m[2][1]).min(m[1][2]).min(m[2][2]),
	 m[1][1].max(m[2][1]).max(m[1][2]).max(m[2][2]))
}

#[inline]
fn diagonal(m: &[[f32; 4]; 4], w: &[f32; 6]) -> f32 {
	#[inline]
	fn df(a: f32, b: f32) -> f32 {
		(a - b).abs()
	}

	let first =
		w[0] * (df(m[0][2], m[1][1]) + df(m[1][1], m[2][0]) + df(m[1][3], m[2][2]) + df(m[2][2], m[3][1])) +
		w[1] * (df(m[0][3], m[1][2]) + df(m[2][1], m[3][0])) +
		w[2] * (df(m[0][3], m[2][1]) + df(m[1][2], m[3][0])) +
		w[3] * (df(m[1][2], m[2][1])) +
		w[4] * (df(m[0][2], m[2][0]) + df(m[1][3], m[3][1])) +
		w[5] * (df(m[0][1], m[1][0]) + df(m[2][3], m[3][2]));

	let second =
		w[0] * (df(m[0][1], m[1][2]) + df(m[1][2], m[2][3]) + df(m[1][0], m[2][1]) + df(m[2][1], m[3][2])) +
		w[1] * (df(m[0][0], m[1][1]) + df(m[2][2], m[3][3])) +
		w[2] * (df(m[0][0], m[2][2]) + df(m[1][1], m[3][3])) +
		w[3] * (df(m[1][1], m[2][2])) +
		w[4] * (df(m[1][0], m[3][2]) + df(m[0][1], m[2][3])) +
		w[5] * (df(m[0][2], m[1][3]) + df(m[2][0], m[3][1]));

	first - second
}

#[inline]
#[allow(non_snake_case)]
fn pick(edge: f32, W1: f32, W2: f32, r: &Matrix, g: &Matrix, b: &Matrix, a: &Matrix) -> (f32, f32, f32, f32) {
	if edge <= 0.0 {
		(W1 * (r[0][3] + r[3][0]) + W2 * (r[1][2] + r[2][1]),
		 W1 * (g[0][3] + g[3][0]) + W2 * (g[1][2] + g[2][1]),
		 W1 * (b[0][3] + b[3][0]) + W2 * (b[1][2] + b[2][1]),
		 W1 * (a[0][3] + a[3][0]) + W2 * (a[1][2] + a[2][1]))
	}
	else {
		(W1 * (r[0][0] + r[3][3]) + W2 * (r[1][1] + r[2][2]),
		 W1 * (g[0][0] + g[3][3]) + W2 * (g[1][1] + g[2][2]),
		 W1 * (b[0][0] + b[3][3]) + W2 * (b[1][1] + b[2][2]),
		 W1 * (a[0][0] + a[3][3]) + W2 * (a[1][1] + a[2][2]))
	}
}
