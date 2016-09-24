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
use pixel::{self, Pixel};

pub trait Sampler<CI, PI, CO, PO>
	where CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      CO: pixel::Channel,
	      PO: Pixel<CO> + pixel::Write<CO>
{
	fn sample(from: &view::Ref<CI, PI>, u: f32, v: f32) -> PO;
}

mod nearest;
pub use self::nearest::Nearest;

mod linear;
pub use self::linear::Linear;

mod cubic;
pub use self::cubic::Cubic;
