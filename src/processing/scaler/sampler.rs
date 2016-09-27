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

use view;
use buffer::Buffer;
use pixel;
use color::Rgba;
use processing::{sample, Sampler};

impl<A, CI, PI, CO, PO> super::Scaler<CI, PI, CO, PO> for A
	where A:  Sampler,
	      CI: pixel::Channel,
	      PI: pixel::Read<CI>,
	      PI: Into<Rgba>,
	      CO: pixel::Channel,
	      PO: pixel::Write<CO>,
	      PO: From<Rgba>
{
	#[inline]
	fn scale(input: &view::Ref<CI, PI>, width: u32, height: u32) -> Buffer<CO, PO, Vec<CO>> {
		let mut tmp = Buffer::<u8, Rgba, _>::new(input.width(), height);
		sample::vertically::<A, _, _, _, _, _, _>(input, &mut tmp);

		let mut out = Buffer::<CO, PO, _>::new(width, height);
		sample::horizontally::<A, _, _, _, _, _, _>(&tmp, &mut out);

		out
	}
}
