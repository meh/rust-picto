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

use super::Ditherer;
use crate::{buffer::Buffer, pixel, view};

/// Trait for ditherable types.
pub trait Dither<P, C>
where
	P: pixel::Read<C> + pixel::Write<C>,
	C: pixel::Channel,
{
	/// Dither to the given number of colors.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::color::Rgb;
	/// use picto::processing::prelude::*;
	///
	/// let image    = read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();
	/// let dithered = image.dither::<ditherer::NeuQuant>(256);
	/// ```
	fn dither<A>(self, colors: u32) -> Buffer<P, C, Vec<C>>
	where
		A: Ditherer<P, C, P, C>;
}

impl<'i, P, C, I> Dither<P, C> for I
where
	P: pixel::Read<C> + pixel::Write<C>,
	C: pixel::Channel,
	I: Into<view::Read<'i, P, C>>,
{
	fn dither<A>(self, colors: u32) -> Buffer<P, C, Vec<C>>
	where
		A: Ditherer<P, C, P, C>,
	{
		it::<A, _, P, C, P, C>(self, colors)
	}
}

/// Dither to the given number of colors.
#[inline]
pub fn it<'i, A, I, PI, CI, PO, CO>(input: I, colors: u32) -> Buffer<PO, CO, Vec<CO>>
where
	A: Ditherer<PI, CI, PO, CO>,
	PO: From<PI>,
	PO: pixel::Write<CO>,
	CO: pixel::Channel,
	PI: pixel::Read<CI>,
	CI: pixel::Channel,
	I: Into<view::Read<'i, PI, CI>>,
{
	A::dither(&input.into(), colors)
}
