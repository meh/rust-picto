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

use num::Float;

pub struct Nearest;

impl<T: Float> super::Sampler<T> for Nearest {
	#[inline]
	fn kernel(x: T) -> T {
		if x.abs() <= num!(0.5) {
			one!()
		}
		else {
			zero!()
		}
	}

	#[inline]
	fn support() -> T {
		num!(0.5)
	}
}
