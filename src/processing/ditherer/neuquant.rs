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

use color_quant::NeuQuant;
use view;
use buffer::{Buffer, cast};
use pixel;
use color::Rgba;

pub struct Best;
pub struct Good;
pub struct Bad;
pub struct Worst;

impl<PI, CI, PO, CO> super::Ditherer<PI, CI, PO, CO> for Best
	where PI: Into<Rgba>,
	      PI: pixel::Read<CI>,
	      CI: pixel::Channel,
	      PO: From<Rgba> + Into<Rgba> + From<PI>,
	      PO: pixel::Write<CO> + pixel::Read<CO>,
	      CO: pixel::Channel
{
	#[inline]
	fn dither(input: &view::Read<PI, CI>, colors: u32) -> Buffer<PO, CO, Vec<CO>> {
		cast::Into::<PO, CO>::into(quantize(1, colors, input))
	}
}

impl<PI, CI, PO, CO> super::Ditherer<PI, CI, PO, CO> for Good
	where PI: Into<Rgba>,
	      PI: pixel::Read<CI>,
	      CI: pixel::Channel,
	      PO: From<Rgba> + Into<Rgba> + From<PI>,
	      PO: pixel::Write<CO> + pixel::Read<CO>,
	      CO: pixel::Channel
{
	#[inline]
	fn dither(input: &view::Read<PI, CI>, colors: u32) -> Buffer<PO, CO, Vec<CO>> {
		cast::Into::<PO, CO>::into(quantize(10, colors, input))
	}
}

impl<PI, CI, PO, CO> super::Ditherer<PI, CI, PO, CO> for Bad
	where PI: Into<Rgba>,
	      PI: pixel::Read<CI>,
	      CI: pixel::Channel,
	      PO: From<Rgba> + Into<Rgba> + From<PI>,
	      PO: pixel::Write<CO> + pixel::Read<CO>,
	      CO: pixel::Channel
{
	#[inline]
	fn dither(input: &view::Read<PI, CI>, colors: u32) -> Buffer<PO, CO, Vec<CO>> {
		cast::Into::<PO, CO>::into(quantize(20, colors, input))
	}
}

impl<PI, CI, PO, CO> super::Ditherer<PI, CI, PO, CO> for Worst
	where PI: Into<Rgba>,
	      PI: pixel::Read<CI>,
	      CI: pixel::Channel,
	      PO: From<Rgba> + Into<Rgba> + From<PI>,
	      PO: pixel::Write<CO> + pixel::Read<CO>,
	      CO: pixel::Channel
{
	#[inline]
	fn dither(input: &view::Read<PI, CI>, colors: u32) -> Buffer<PO, CO, Vec<CO>> {
		cast::Into::<PO, CO>::into(quantize(30, colors, input))
	}
}

fn quantize<P, C>(samples: i32, colors: u32, input: &view::Read<P, C>) -> Buffer<Rgba, u8, Vec<u8>>
	where P: Into<Rgba>,
	      P: pixel::Read<C>,
	      C: pixel::Channel
{
	let mut buffer = input.convert();
	let     quant  = NeuQuant::new(samples, colors as usize, &buffer);

	for chunk in buffer.chunks_mut(4) {
		quant.map_pixel(chunk);
	}

	buffer
}
