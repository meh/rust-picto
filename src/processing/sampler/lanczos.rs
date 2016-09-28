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

pub struct Lanczos2;
pub struct Lanczos3;

impl<T: Float + FloatConst> super::Sampler<T> for Lanczos2 {
	#[inline]
	fn kernel(x: T) -> T {
		function(x, num!(2.0))
	}

	#[inline]
	fn support() -> T {
		num!(2.0)
	}
}

impl<T: Float + FloatConst> super::Sampler<T> for Lanczos3 {
	#[inline]
	fn kernel(x: T) -> T {
		function(x, num!(3.0))
	}

	#[inline]
	fn support() -> T {
		num!(3.0)
	}
}

/// The Lanczos function.
#[inline]
pub fn function<T: Float + FloatConst>(x: T, t: T) -> T {
	if x.abs() < t {
		sinc(x) * sinc(x / t)
	}
	else {
		zero!()
	}
}

/// The Sinc function.
#[inline]
pub fn sinc<T: Float + FloatConst>(t: T) -> T {
	let a = t * T::PI();

	if t == zero!() {
		one!()
	}
	else {
		a.sin() / a
	}
}
