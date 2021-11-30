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

use crate::view;
use crate::buffer::Buffer;
use crate::pixel;
use crate::color::Rgba;
use crate::processing::{sample, Sampler};

impl<A, PI, CI, PO, CO> super::Scaler<PI, CI, PO, CO> for A
	where A:  Sampler,
	      PI: Into<Rgba>,
	      PI: pixel::Read<CI>,
	      CI: pixel::Channel,
	      PO: From<Rgba>,
	      PO: pixel::Write<CO>,
	      CO: pixel::Channel,
{
	#[inline]
	fn scale(input: &view::Read<PI, CI>, width: u32, height: u32) -> Buffer<PO, CO, Vec<CO>> {
		let mut tmp = Buffer::<Rgba, u8, _>::new(input.width(), height);
		sample::vertically::<A, _, _, _, _, _, _>(input, &mut tmp);

		let mut out = Buffer::<PO, CO, _>::new(width, height);
		sample::horizontally::<A, _, _, _, _, _, _>(&tmp, &mut out);

		out
	}
}
