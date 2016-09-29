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
use pixel;
use view;
use color::{Rgba, ComponentWise, Limited};
use processing::Blur;

/// Trait for blurrable types.
pub trait Sharpen<PI, CI>
	where PI: Into<Rgba>,
	      PI: pixel::Read<CI>,
	      CI: pixel::Channel,
{
	/// Sharpen by the given radius and threshold.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::color::Rgb;
	/// use picto::processing::prelude::*;
	///
	/// let image = read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();
	///
	/// image.sharpen::<Rgb, u8>(4.0, 0.02);
	/// ```
	fn sharpen<PO, CO>(self, sigma: f32, threshold: f32) -> Buffer<PO, CO, Vec<CO>>
		where PO: From<Rgba> + Into<Rgba>,
		      PO: pixel::Read<CO> + pixel::Write<CO>,
		      CO: pixel::Channel;
}

impl<'i, PI, CI, I> Sharpen<PI, CI> for I
	where PI: Into<Rgba>,
	      PI: pixel::Read<CI>,
	      CI: pixel::Channel,
	      I:  Into<view::Read<'i, PI, CI>>
{
	#[inline]
	fn sharpen<PO, CO>(self, sigma: f32, threshold: f32) -> Buffer<PO, CO, Vec<CO>>
		where PO: From<Rgba> + Into<Rgba>,
		      PO: pixel::Read<CO> + pixel::Write<CO>,
		      CO: pixel::Channel,
	{
		by::<PO, CO, PI, CI, _>(self, sigma, threshold)
	}
}

/// Sharpen by the given radius and threshold.
#[inline]
pub fn by<'i, PO, CO, PI, CI, I>(input: I, sigma: f32, threshold: f32) -> Buffer<PO, CO, Vec<CO>>
	where PO: From<Rgba> + Into<Rgba>,
	      PO: pixel::Read<CO> + pixel::Write<CO>,
	      CO: pixel::Channel,
	      PI: Into<Rgba>,
	      PI: pixel::Read<CI>,
	      CI: pixel::Channel,
	      I:  Into<view::Read<'i, PI, CI>>
{
	let     input  = input.into();
	let mut output = (&input).blur::<PO, CO>(sigma);

	for ((_, _, i), (_, _, mut o)) in input.pixels().zip(output.pixels_mut()) {
		let a = i.get().into();
		let b = o.get().into();

		o.set(&a.component_wise(&b, |a, b| {
			let diff = (a - b).abs();

			if diff > threshold {
				a + diff
			}
			else {
				a
			}
		}).clamp().into())
	}

	output
}
