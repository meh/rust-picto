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
use processing::blur;

/// Trait for blurrable types.
pub trait Sharpen<P, C>
	where P: From<Rgba> + Into<Rgba>,
	      P: pixel::Read<C> + pixel::Write<C>,
	      C: pixel::Channel,
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
	/// image.sharpen(4.0, 0.02);
	/// ```
	fn sharpen(self, sigma: f32, threshold: f32) -> Buffer<P, C, Vec<C>>;
}

impl<'i, P, C, I> Sharpen<P, C> for I
	where P: From<Rgba> + Into<Rgba>,
	      P: pixel::Read<C> + pixel::Write<C>,
	      C: pixel::Channel,
	      I: Into<view::Read<'i, P, C>>
{
	#[inline]
	fn sharpen(self, sigma: f32, threshold: f32) -> Buffer<P, C, Vec<C>> {
		by::<_, P, C, P, C>(self, sigma, threshold)
	}
}

/// Sharpen by the given radius and threshold.
#[inline]
pub fn by<'i, I, PI, CI, PO, CO>(input: I, sigma: f32, threshold: f32) -> Buffer<PO, CO, Vec<CO>>
	where I:  Into<view::Read<'i, PI, CI>>,
	      PO: From<Rgba> + Into<Rgba>,
	      PO: pixel::Read<CO> + pixel::Write<CO>,
	      CO: pixel::Channel,
	      PI: Into<Rgba>,
	      PI: pixel::Read<CI>,
	      CI: pixel::Channel,
{
	let     input  = input.into();
	let mut output = blur::by::<_, PI, CI, PO, CO>(&input, sigma);

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
