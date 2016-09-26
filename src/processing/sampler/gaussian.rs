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
use num::traits::FloatConst;

pub struct Gaussian;

impl<T: Float + FloatConst> super::Sampler<T> for Gaussian {
	#[inline]
	fn kernel(x: T) -> T {
		function(x, num!(1.0))
	}

	#[inline]
	fn support() -> T {
		num!(3.0)
	}
}

#[inline]
pub fn function<T: Float + FloatConst>(x: T, r: T) -> T {
	((num!(2.0 => T) * T::PI()).sqrt() * r).recip() *
	(-x.powi(2) / (num!(2.0 => T) * r.powi(2))).exp()
}
