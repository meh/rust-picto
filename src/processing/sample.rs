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

use super::Sampler;
use crate::{
	color::{Limited, Rgba},
	orientation::Orientation,
	pixel,
	util::GetClamped,
	view,
};

/// Trait for samplable types.
pub trait Sample<PI, CI, PO, CO>
where
	PI: pixel::Read<CI>,
	CI: pixel::Channel,
	PO: pixel::Write<CO>,
	CO: pixel::Channel,
{
	/// Sample in the given direction.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::buffer;
	/// use picto::color::Rgb;
	/// use picto::processing::prelude::*;
	///
	/// let     image    = read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();
	/// let mut vertical = buffer::Rgb::new(image.width(), image.height() * 2);
	/// let mut resized  = buffer::Rgb::new(image.width() * 2, image.height() * 2);
	///
	/// image.sample::<sampler::Gaussian, _>(&mut vertical, sample::Vertically);
	/// vertical.sample::<sampler::Gaussian, _>(&mut resized, sample::Horizontally);
	/// ```
	fn sample<'o, A, O>(self, output: O, mode: Orientation)
	where
		A: Sampler,
		O: Into<view::Write<'o, PO, CO>>;

	/// Sample in the given direction with the given support and kernel function.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::buffer;
	/// use picto::color::Rgb;
	/// use picto::processing::prelude::*;
	///
	/// let     image    = read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();
	/// let mut vertical = buffer::Rgb::new(image.width(), image.height() * 2);
	/// let mut resized  = buffer::Rgb::new(image.width() * 2, image.height() * 2);
	///
	/// // Nearest neighbor sampling.
	/// image.sample_with(&mut vertical, sample::Vertically, 0.5, |x| if x.abs() <= 0.5 { 1.0 } else { 0.0 });
	/// vertical.sample_with(&mut resized, sample::Horizontally, 0.5, |x| if x.abs() <= 0.5 { 1.0 } else { 0.0 });
	/// ```
	fn sample_with<'o, F, O>(self, output: O, mode: Orientation, support: f32, kernel: F)
	where
		F: FnMut(f32) -> f32,
		O: Into<view::Write<'o, PO, CO>>;
}

impl<'i, PI, CI, PO, CO, I> Sample<PI, CI, PO, CO> for I
where
	PI: Into<Rgba>,
	PI: pixel::Read<CI>,
	CI: pixel::Channel,
	PO: From<Rgba>,
	PO: pixel::Write<CO>,
	CO: pixel::Channel,
	I: Into<view::Read<'i, PI, CI>>,
{
	fn sample<'o, A, O>(self, output: O, mode: Orientation)
	where
		A: Sampler,
		O: Into<view::Write<'o, PO, CO>>,
	{
		match mode {
			Orientation::Vertical => vertically::<A, PO, CO, PI, CI, _, _>(self, output),

			Orientation::Horizontal => horizontally::<A, PO, CO, PI, CI, _, _>(self, output),
		}
	}

	fn sample_with<'o, F, O>(self, output: O, mode: Orientation, support: f32, kernel: F)
	where
		F: FnMut(f32) -> f32,
		O: Into<view::Write<'o, PO, CO>>,
	{
		match mode {
			Orientation::Vertical => vertically_with::<PO, CO, PI, CI, _, _, _>(self, output, support, kernel),

			Orientation::Horizontal => horizontally_with::<PO, CO, PI, CI, _, _, _>(self, output, support, kernel),
		}
	}
}

/// Sample vertically with the given `Sampler`.
#[inline]
pub fn vertically<'i, 'o, A, PO, CO, PI, CI, I, O>(input: I, output: O)
where
	A: Sampler,
	PO: From<Rgba>,
	PO: pixel::Write<CO>,
	CO: pixel::Channel,
	PI: Into<Rgba>,
	PI: pixel::Read<CI>,
	CI: pixel::Channel,
	I: Into<view::Read<'i, PI, CI>>,
	O: Into<view::Write<'o, PO, CO>>,
{
	vertically_with(input, output, A::support(), A::kernel)
}

/// Sample vertically with the given support and kernel function.
pub fn vertically_with<'i, 'o, PO, CO, PI, CI, I, O, F>(input: I, output: O, support: f32, mut kernel: F)
where
	PO: From<Rgba>,
	PO: pixel::Write<CO>,
	CO: pixel::Channel,
	PI: Into<Rgba>,
	PI: pixel::Read<CI>,
	CI: pixel::Channel,
	I: Into<view::Read<'i, PI, CI>>,
	O: Into<view::Write<'o, PO, CO>>,
	F: FnMut(f32) -> f32,
{
	let input = input.into();
	let input = &input;
	let mut output = output.into();

	debug_assert_eq!(input.width(), output.width());

	let ratio = input.height() as f32 / output.height() as f32;
	let scale = if ratio > 1.0 { ratio } else { 1.0 };
	let radius = (support * scale).ceil();

	for x in 0..input.width() as i64 {
		for y_out in 0..output.height() {
			let y_in = (y_out as f32 + 0.5) * ratio;

			let left = (y_in - radius) as i64;
			let right = (y_in + radius) as i64;

			let mut sum = (0.0, 0.0, 0.0, 0.0);
			let mut t = (0.0, 0.0, 0.0, 0.0);

			for i in left..right + 1 {
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

			output.set(
				x as u32,
				y_out,
				&Rgba::new(t.0 / sum.0, t.1 / sum.1, t.2 / sum.2, t.3 / sum.3)
					.clamp()
					.into(),
			);
		}
	}
}

/// Sample horizontally with the given `Sampler`.
#[inline]
pub fn horizontally<'i, 'o, A, PO, CO, PI, CI, I, O>(input: I, output: O)
where
	A: Sampler,
	PO: From<Rgba>,
	PO: pixel::Write<CO>,
	CO: pixel::Channel,
	PI: Into<Rgba>,
	PI: pixel::Read<CI>,
	CI: pixel::Channel,
	I: Into<view::Read<'i, PI, CI>>,
	O: Into<view::Write<'o, PO, CO>>,
{
	horizontally_with(input, output, A::support(), A::kernel)
}

/// Sample horizontally with the given support and kernel function.
pub fn horizontally_with<'i, 'o, PO, CO, PI, CI, I, O, F>(input: I, output: O, support: f32, mut kernel: F)
where
	PO: From<Rgba>,
	PO: pixel::Write<CO>,
	CO: pixel::Channel,
	PI: Into<Rgba>,
	PI: pixel::Read<CI>,
	CI: pixel::Channel,
	I: Into<view::Read<'i, PI, CI>>,
	O: Into<view::Write<'o, PO, CO>>,
	F: FnMut(f32) -> f32,
{
	let input = input.into();
	let input = &input;
	let mut output = output.into();

	debug_assert_eq!(input.height(), output.height());

	let ratio = input.width() as f32 / output.width() as f32;
	let scale = if ratio > 1.0 { ratio } else { 1.0 };
	let radius = (support * scale).ceil();

	for y in 0..input.height() as i64 {
		for x_out in 0..output.width() {
			let x_in = (x_out as f32 + 0.5) * ratio;

			let left = (x_in - radius) as i64;
			let right = (x_in + radius) as i64;

			let mut sum = (0.0, 0.0, 0.0, 0.0);
			let mut t = (0.0, 0.0, 0.0, 0.0);

			for i in left..right + 1 {
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

			output.set(
				x_out,
				y as u32,
				&Rgba::new(t.0 / sum.0, t.1 / sum.1, t.2 / sum.2, t.3 / sum.3)
					.clamp()
					.into(),
			);
		}
	}
}
