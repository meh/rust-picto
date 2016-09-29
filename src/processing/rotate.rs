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

/// Trait for rotatable types.
pub trait Rotate<PI, CI>
	where PI: pixel::Read<CI>,
	      CI: pixel::Channel,
{
	/// Rotate by the given degree, negative degrees will turn counter-clockwise.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::color::Rgb;
	/// use picto::processing::prelude::*;
	///
	/// let image   = read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();
	/// let rotated = image.rotate::<Rgb, u8>(90.0);
	///
	/// assert_eq!(rotated.width(), 240);
	/// assert_eq!(rotated.height(), 320);
	/// ```
	fn rotate<PO, CO>(self, by: f32) -> Buffer<PO, CO, Vec<CO>>
		where PO: From<PI>,
		      PO: pixel::Write<CO>,
		      CO: pixel::Channel;
}

impl<'i, PI, CI, I> Rotate<PI, CI> for I
	where PI: pixel::Read<CI>,
	      CI: pixel::Channel,
	      I:  Into<view::Read<'i, PI, CI>>
{
	#[inline]
	fn rotate<PO, CO>(self, by: f32) -> Buffer<PO, CO, Vec<CO>>
		where PO: From<PI>,
		      PO: pixel::Write<CO>,
		      CO: pixel::Channel,
	{
		it(self, by)
	}
}

/// Rotate by the given degree, negative degrees will turn counter-clockwise.
pub fn it<'i, PO, CO, PI, CI, I>(input: I, by: f32) -> Buffer<PO, CO, Vec<CO>>
	where PO: From<PI>,
	      PO: pixel::Write<CO>,
	      CO: pixel::Channel,
	      PI: pixel::Read<CI>,
	      CI: pixel::Channel,
	      I:  Into<view::Read<'i, PI, CI>>
{
	let input = input.into();
	let by    = if by.is_sign_positive() {
		by % 360.0
	}
	else {
		360.0 - (by % 360.0)
	} as u32;

	debug_assert!(by % 90 == 0);

	if by == 0 {
		return input.convert::<PO, CO>();
	}

	let mut output: Buffer<PO, CO, _>;

	match by {
		90 => {
			output = Buffer::new(input.height(), input.width());

			for (x, y, px) in input.pixels() {
				output.set(input.height() - 1 - y, x, &px.get().into());
			}
		}

		180 => {
			output = Buffer::new(input.width(), input.height());

			for (x, y, px) in input.pixels() {
				output.set(input.width() - 1 - x, input.height() - 1 - y, &px.get().into());
			}
		}

		270 => {
			output = Buffer::new(input.height(), input.width());

			for (x, y, px) in input.pixels() {
				output.set(y, input.width() - 1 - x, &px.get().into());
			}
		}

		_ => unreachable!()
	}

	output
}
