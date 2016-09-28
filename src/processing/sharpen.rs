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
pub trait Sharpen<CI, PI>
	where CI: pixel::Channel,
	      PI: pixel::Read<CI>,
	      PI: Into<Rgba>
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
	/// let image = read::from_path::<u8, Rgb, _>("tests/boat.xyz").unwrap();
	///
	/// image.sharpen::<u8, Rgb>(4.0, 0.02);
	/// ```
	fn sharpen<CO, PO>(self, sigma: f32, threshold: f32) -> Buffer<CO, PO, Vec<CO>>
		where CO: pixel::Channel,
		      PO: pixel::Read<CO> + pixel::Write<CO>,
		      PO: From<Rgba> + Into<Rgba>;
}

impl<'i, CI, PI, I> Sharpen<CI, PI> for I
	where CI: pixel::Channel,
	      PI: pixel::Read<CI>,
	      PI: Into<Rgba>,
	      I:  Into<view::Read<'i, CI, PI>>
{
	#[inline]
	fn sharpen<CO, PO>(self, sigma: f32, threshold: f32) -> Buffer<CO, PO, Vec<CO>>
		where CO: pixel::Channel,
		      PO: pixel::Read<CO> + pixel::Write<CO>,
		      PO: From<Rgba> + Into<Rgba>
	{
		by::<CO, PO, CI, PI, _>(self, sigma, threshold)
	}
}

/// Sharpen by the given radius and threshold.
#[inline]
pub fn by<'i, CO, PO, CI, PI, I>(input: I, sigma: f32, threshold: f32) -> Buffer<CO, PO, Vec<CO>>
	where CO: pixel::Channel,
	      PO: pixel::Read<CO> + pixel::Write<CO>,
	      PO: From<Rgba> + Into<Rgba>,
	      CI: pixel::Channel,
	      PI: pixel::Read<CI>,
	      PI: Into<Rgba>,
	      I:  Into<view::Read<'i, CI, PI>>
{
	let     input  = input.into();
	let mut output = (&input).blur::<CO, PO>(sigma);

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