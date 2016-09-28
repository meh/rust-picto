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
pub trait Blur<CI, PI>
	where CI: pixel::Channel,
	      PI: pixel::Read<CI>,
	      PI: Into<Rgba>
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
	/// let image = read::from_path::<u8, Rgb, _>("tests/boat.xyz").unwrap();
	///
	/// image.blur::<u8, Rgb>(1.0);
	/// ```
	fn blur<CO, PO>(self, sigma: f32) -> Buffer<CO, PO, Vec<CO>>
		where CO: pixel::Channel,
		      PO: pixel::Read<CO> + pixel::Write<CO>,
		      PO: From<Rgba> + Into<Rgba>;
}

impl<'i, CI, PI, I> Blur<CI, PI> for I
	where CI: pixel::Channel,
	      PI: pixel::Read<CI>,
	      PI: Into<Rgba>,
	      I:  Into<view::Read<'i, CI, PI>>
{
	#[inline]
	fn blur<CO, PO>(self, sigma: f32) -> Buffer<CO, PO, Vec<CO>>
		where CO: pixel::Channel,
		      PO: pixel::Read<CO> + pixel::Write<CO>,
		      PO: From<Rgba> + Into<Rgba>
	{
		by::<CO, PO, CI, PI, _>(self, sigma)
	}
}

/// Blur by the given radius.
#[inline]
pub fn by<'i, CO, PO, CI, PI, I>(input: I, mut sigma: f32) -> Buffer<CO, PO, Vec<CO>>
	where CO: pixel::Channel,
	      PO: pixel::Read<CO> + pixel::Write<CO>,
	      PO: From<Rgba> + Into<Rgba>,
	      CI: pixel::Channel,
	      PI: pixel::Read<CI>,
	      PI: Into<Rgba>,
	      I:  Into<view::Read<'i, CI, PI>>
{
	let input = input.into();

	if sigma < 0.0 {
		sigma = 1.0;
	}

	let mut tmp = Buffer::<CO, PO, _>::new(input.width(), input.height());
	sample::vertically_with(&input, &mut tmp, sigma * 2.0, |x| gaussian::function(x, sigma));

	let mut out = Buffer::<CO, PO, _>::new(input.width(), input.height());
	sample::horizontally_with(&tmp, &mut out, sigma * 2.0, |x| gaussian::function(x, sigma));

	out
}
