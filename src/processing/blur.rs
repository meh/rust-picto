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
use color::Rgba;
use processing::sampler::gaussian;
use processing::sample;

/// Trait for blurrable types.
pub trait Blur<PI, CI>
	where PI: Into<Rgba>,
	      PI: pixel::Read<CI>,
	      CI: pixel::Channel,
{
	/// Blur by the given radius.
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
	/// image.blur::<Rgb, u8>(1.0);
	/// ```
	fn blur<PO, CO>(self, sigma: f32) -> Buffer<PO, CO, Vec<CO>>
		where PO: From<Rgba> + Into<Rgba>,
		      PO: pixel::Read<CO> + pixel::Write<CO>,
		      CO: pixel::Channel;
}

impl<'i, PI, CI, I> Blur<PI, CI> for I
	where PI: Into<Rgba>,
	      PI: pixel::Read<CI>,
	      CI: pixel::Channel,
	      I:  Into<view::Read<'i, PI, CI>>
{
	#[inline]
	fn blur<PO, CO>(self, sigma: f32) -> Buffer<PO, CO, Vec<CO>>
		where PO: From<Rgba> + Into<Rgba>,
		      PO: pixel::Read<CO> + pixel::Write<CO>,
		      CO: pixel::Channel,
	{
		by::<PO, CO, PI, CI, _>(self, sigma)
	}
}

/// Blur by the given radius.
#[inline]
pub fn by<'i, PO, CO, PI, CI, I>(input: I, mut sigma: f32) -> Buffer<PO, CO, Vec<CO>>
	where PO: From<Rgba> + Into<Rgba>,
	      PO: pixel::Read<CO> + pixel::Write<CO>,
	      CO: pixel::Channel,
	      PI: Into<Rgba>,
	      PI: pixel::Read<CI>,
	      CI: pixel::Channel,
	      I:  Into<view::Read<'i, PI, CI>>
{
	let input = input.into();

	if sigma < 0.0 {
		sigma = 1.0;
	}

	let mut tmp = Buffer::<PO, CO, _>::new(input.width(), input.height());
	sample::vertically_with(&input, &mut tmp, sigma * 2.0, |x| gaussian::function(x, sigma));

	let mut out = Buffer::<PO, CO, _>::new(input.width(), input.height());
	sample::horizontally_with(&tmp, &mut out, sigma * 2.0, |x| gaussian::function(x, sigma));

	out
}
