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

use crate::{
	buffer::Buffer,
	color::Rgba,
	pixel,
	processing::{sample, sampler::gaussian},
	view,
};

/// Trait for blurrable types.
pub trait Blur<P, C>
where
	P: From<Rgba> + Into<Rgba>,
	P: pixel::Read<C> + pixel::Write<C>,
	C: pixel::Channel,
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
	/// image.blur(1.0);
	/// ```
	fn blur(self, sigma: f32) -> Buffer<P, C, Vec<C>>;
}

impl<'i, P, C, I> Blur<P, C> for I
where
	P: From<Rgba> + Into<Rgba>,
	P: pixel::Read<C> + pixel::Write<C>,
	C: pixel::Channel,
	I: Into<view::Read<'i, P, C>>,
{
	#[inline]
	fn blur(self, sigma: f32) -> Buffer<P, C, Vec<C>> {
		by::<_, P, C, P, C>(self, sigma)
	}
}

/// Blur by the given radius.
#[inline]
pub fn by<'i, I, PI, CI, PO, CO>(input: I, mut sigma: f32) -> Buffer<PO, CO, Vec<CO>>
where
	PO: From<Rgba> + Into<Rgba>,
	PO: pixel::Read<CO> + pixel::Write<CO>,
	CO: pixel::Channel,
	PI: Into<Rgba>,
	PI: pixel::Read<CI>,
	CI: pixel::Channel,
	I: Into<view::Read<'i, PI, CI>>,
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
