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
use iter::pixel::Iter as Pixels;

/// An immutable view into a `Buffer`.
pub struct Ref<'a, C: pixel::Channel, P: Pixel<C>> {
	area: Area,
	data: &'a [C],

	_channel: PhantomData<C>,
	_pixel:   PhantomData<P>,
}

impl<'a, C, P> Ref<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C>
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &[C], area: Area) -> Ref<C, P> {
		Ref {
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

impl<'a, C, P> Ref<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C>
{
	/// Get the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it will panic.
	#[inline]
	pub fn get(&self, x: u32, y: u32) -> P {
		if x >= self.area.width || y >= self.area.height {
			panic!("out of bounds");
		}

		let channels = P::channels();
		let index    = channels * ((self.area.y + y) as usize * self.area.width as usize + (self.area.x + x) as usize);

		P::read(&self.data[index .. index + channels])
	}

	/// Get an immutable iterator over the view's pixels.
	pub fn pixels(&self) -> Pixels<C, P> {
		Pixels::new(self.data, self.area)
	}
}

#[cfg(test)]
mod test {
	use buffer::*;
	use color::*;

	#[test]
	fn get() {
		assert_eq!(Rgb::new(1.0, 0.0, 1.0),
			Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![255, 0, 255]).unwrap().get(0, 0));

		assert_eq!(Rgba::new(0.0, 1.0, 1.0, 0.0),
			Buffer::<u8, Rgba, _>::from_raw(1, 2, vec![255, 0, 255, 0, 0, 255, 255, 0]).unwrap().get(0, 1));
	}
}
