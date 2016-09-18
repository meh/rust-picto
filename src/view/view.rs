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

use std::marker::PhantomData;

use pixel::{self, Pixel};
use area::Area;
use super::{Ref, Mut};

/// A view into a `Buffer`.
pub struct View<'a, C: pixel::Channel, P: Pixel<C>> {
	area: Area,
	data: &'a mut [C],

	_channel: PhantomData<C>,
	_pixel:   PhantomData<P>,
}

impl<'a, C, P> View<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C>
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &mut [C], area: Area) -> View<C, P> {
		View {
			area: area,
			data: data,

			_channel: PhantomData,
			_pixel:   PhantomData,
		}
	}

	/// Get the area.
	#[inline]
	pub fn area(&self) -> Area {
		self.area
	}
}

impl<'a, C, P> View<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C> + pixel::Write<C>
{
	/// Get the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it will panic.
	#[inline]
	pub fn get(&self, x: u32, y: u32) -> P {
		Ref::new(self.data, self.area).get(x, y)
	}

	/// Set the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it will panic.
	#[inline]
	pub fn set(&mut self, x: u32, y: u32, pixel: &P) {
		Mut::new(self.data, self.area).set(x, y, pixel)
	}
}
