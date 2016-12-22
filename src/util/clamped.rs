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

use pixel;
use view;

/// Clamped getter.
pub trait Get<P, C>
	where P: pixel::Read<C>,
	      C: pixel::Channel,
{
	/// Get the pixel at the coordinates clamped to the width and height.
	fn get_clamped(self, x: i64, y: i64) -> P;
}

impl<'a, P, C, T> Get<P, C> for T
	where P: pixel::Read<C>,
	      C: pixel::Channel,
	      T: Into<view::Read<'a, P, C>>,
{
	#[inline]
	fn get_clamped(self, x: i64, y: i64) -> P {
		let view   = self.into();
		let width  = view.width() as i64;
		let height = view.height() as i64;

		view.get(clamp(x, 0, width - 1)  as u32,
		         clamp(y, 0, height - 1) as u32)
	}
}

/// Clamped setter.
pub trait Set<P, C>
	where P: pixel::Write<C>,
	      C: pixel::Channel,
{
	/// Set the pixel at the coordinates clamped to the width and height.
	fn set_clamped(self, x: i64, y: i64, value: &P);
}

impl<'a, P, C, T> Set<P, C> for T
	where P: pixel::Write<C>,
	      C: pixel::Channel,
	      T: Into<view::Write<'a, P, C>>,
{
	#[inline]
	fn set_clamped(self, x: i64, y: i64, value: &P) {
		let mut view   = self.into();
		let     width  = view.width() as i64;
		let     height = view.height() as i64;

		view.set(clamp(x, 0, width - 1) as u32,
		         clamp(y, 0, height - 1) as u32,
		         value);
	}
}

/// Clamp the given value between `min` and `max`.
#[inline]
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
	if value > max {
		max
	}
	else if value < min {
		min
	}
	else {
		value
	}
}
