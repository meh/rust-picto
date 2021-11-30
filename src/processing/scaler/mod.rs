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

pub trait Scaler<PI, CI, PO, CO>
	where PI: pixel::Read<CI>,
	      CI: pixel::Channel,
	      PO: pixel::Write<CO>,
	      CO: pixel::Channel,
{
	fn scale(input: &view::Read<PI, CI>, width: u32, height: u32) -> Buffer<PO, CO, Vec<CO>>;
}

mod sampler;
pub use super::sampler::Linear;
pub use super::sampler::Cubic;
pub use super::sampler::Gaussian;
pub use super::sampler::Lanczos2;
pub use super::sampler::Lanczos3;

mod nearest;
pub use self::nearest::Nearest;

pub mod xbr;
