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
use buffer::Buffer;
use iter::pixel::{Iter as Pixels, IterMut as PixelsMut};
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

	/// Get a mutable iterator over the view's pixels.
	pub fn pixels(&self) -> Pixels<C, P> {
		Pixels::new(self.data, self.area)
	}

	/// Get a mutable iterator over the view's pixels.
	pub fn pixels_mut(&mut self) -> PixelsMut<C, P> {
		PixelsMut::new(self.data, self.area)
	}

	/// Create a `Buffer` from the `View`.
	pub fn into_owned(&self) -> Buffer<C, P, Vec<C>> {
		let mut buffer = Buffer::new(self.area.width, self.area.height);

		for (x, y, px) in self.pixels() {
			buffer.set(x, y, &px.get());
		}

		buffer
	}
}

#[cfg(test)]
mod test {
	use buffer::*;
	use color::*;

	#[test]
	fn pixels_mut() {
		let mut buffer = Buffer::<u8, Rgb, _>::from_raw(2, 2, vec![0, 255, 0, 255, 0, 255, 255, 255, 255, 0, 0, 0]).unwrap();
		let mut view = buffer.view(Default::default());

		for (x, y, mut px) in view.pixels_mut() {
			assert!(x <= 1 && y <= 1);

			let value = px.get::<Rgb>();
			px.set(Rgb::new(1.0, 1.0, 1.0) - value);
		}

		assert_eq!(view.get(0, 0), Rgb::new(1.0, 0.0, 1.0));
		assert_eq!(view.get(1, 0), Rgb::new(0.0, 1.0, 0.0));
		assert_eq!(view.get(0, 1), Rgb::new(0.0, 0.0, 0.0));
		assert_eq!(view.get(1, 1), Rgb::new(1.0, 1.0, 1.0));
	}
}
