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

pub trait Ditherer<PI, CI, PO, CO>
	where PI: pixel::Read<CI>,
	      CI: pixel::Channel,
	      PO: pixel::Write<CO>,
	      CO: pixel::Channel,
{
	fn dither(input: &view::Read<PI, CI>, colors: u32) -> Buffer<PO, CO, Vec<CO>>;
}

pub mod neuquant;
pub use self::neuquant::Good as NeuQuant;

pub mod palette;
pub use self::palette::Palette;
