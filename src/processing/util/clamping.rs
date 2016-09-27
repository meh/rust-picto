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

pub trait Clamped<C, P>: Get<C, P> + Set<C, P>
	where C: pixel::Channel,
	      P: pixel::Read<C> + pixel::Write<C>
{ }

pub trait Get<C, P>
	where C: pixel::Channel,
	      P: pixel::Read<C>
{
	fn get_clamped(self, x: i64, y: i64) -> P;
}

impl<'a, C, P, T> Get<C, P> for T
	where C: pixel::Channel,
	      P: pixel::Read<C>,
	      T: Into<view::Ref<'a, C, P>>
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

pub trait Set<C, P>
	where C: pixel::Channel,
	      P: pixel::Write<C>
{
	fn set_clamped(self, x: i64, y: i64, value: &P);
}

impl<'a, C, P, T> Set<C, P> for T
	where C: pixel::Channel,
	      P: pixel::Write<C>,
	      T: Into<view::Mut<'a, C, P>>
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
