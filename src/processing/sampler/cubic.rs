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

pub struct Cubic;

impl<T: Float> super::Sampler<T> for Cubic {
	#[inline]
	fn kernel(x: T) -> T {
		spline(x, zero!(), num!(0.5))
	}

	#[inline]
	fn support() -> T {
		num!(2.0)
	}
}

/// The Catmull-Rom cubic spline.
pub fn spline<T: Float>(x: T, b: T, c: T) -> T {
	let a = x.abs();
	let k = if a < one!() {
		(num!(12.0 => T) - num!(9.0 => T) * b - num!(6.0 => T) * c) * a.powi(3) +
		(num!(-18.0 => T) + num!(12.0 => T) * b + num!(6.0 => T) * c) * a.powi(2) +
		(num!(6.0 => T) - num!(2.0 => T) * b)
	}
	else if a < num!(2.0) {
		(-b - num!(6.0 => T) * c) * a.powi(3) +
		(num!(6.0 => T) * b + num!(30.0 => T) * c) * a.powi(2) +
		(num!(-12.0 => T) * b - num!(48.0 => T) * c) * a +
		(num!(8.0 => T) * b + num!(24.0 => T) * c)
	}
	else {
		zero!()
	};

	k / num!(6.0)
}
