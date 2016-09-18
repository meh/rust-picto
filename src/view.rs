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

/// A view into a `Buffer`.
pub struct View<'a, C: pixel::Channel, P: Pixel<C>> {
	area:    Area,
	data:    &'a mut [C],
	channel: PhantomData<C>,
	pixel:   PhantomData<P>,
}

impl<'a, C, P> View<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C>
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &mut [C], area: Area) -> View<C, P> {
		View {
			area:    area,
			data:    data,
			channel: PhantomData,
			pixel:   PhantomData,
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

	/// Transform the pixels within the view.
	///
	/// The passed function takes the `x`, `y` and the pixel value and returns a
	/// new pixel value.
	#[inline]
	pub fn transform<T, F>(&mut self, mut func: F)
		where T: Into<P>,
		      F: FnMut(u32, u32, P) -> T
	{
		for x in 0 .. self.area.width {
			for y in 0 .. self.area.height {
				let px = self.get(x, y);
				self.set(x, y, &func(x, y, px).into());
			}
		}
	}
}

/// An immutable view into a `Buffer`.
pub struct Ref<'a, C: pixel::Channel, P: Pixel<C>> {
	area:    Area,
	data:    &'a [C],
	channel: PhantomData<C>,
	pixel:   PhantomData<P>,
}

impl<'a, C, P> Ref<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C>
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &[C], area: Area) -> Ref<C, P> {
		Ref {
			area:    area,
			data:    data,
			channel: PhantomData,
			pixel:   PhantomData,
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
}

/// A mutable view into a `Buffer`.
pub struct Mut<'a, C: pixel::Channel, P: Pixel<C>> {
	area:    Area,
	data:    &'a mut [C],
	channel: PhantomData<C>,
	pixel:   PhantomData<P>,
}

impl<'a, C, P> Mut<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C>
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &mut [C], area: Area) -> Mut<C, P> {
		Mut {
			area:    area,
			data:    data,
			channel: PhantomData,
			pixel:   PhantomData,
		}
	}

	/// Get the area.
	#[inline]
	pub fn area(&self) -> Area {
		self.area
	}
}

impl<'a, C, P> Mut<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Write<C>
{
	/// Set the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it will panic.
	#[inline]
	pub fn set(&mut self, x: u32, y: u32, value: &P) {
		if x >= self.area.width || y >= self.area.height {
			panic!("out of bounds");
		}

		let channels = P::channels();
		let index    = channels * ((self.area.y + y) as usize * self.area.width as usize + (self.area.x + x) as usize);

		value.write(&mut self.data[index .. index + channels]);
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

	#[test]
	fn set() {
		let mut image = Buffer::<u8, Rgb, Vec<_>>::new(2, 2);
		image.set(0, 0, &Rgb::new(1.0, 0.0, 1.0));

		assert_eq!(Rgb::new(1.0, 0.0, 1.0),
			image.get(0, 0));
	}

	#[test]
	fn transform() {
		let mut buffer = Buffer::<u8, Rgb, _>::from_raw(2, 2, vec![0, 255, 0, 255, 0, 255, 255, 255, 255, 0, 0, 0]).unwrap();
		let mut view   = buffer.view(Default::default());

		view.transform(|_, _, px| {
			Rgb::new(1.0, 1.0, 1.0) - px
		});

		assert_eq!(view.get(0, 0), Rgb::new(1.0, 0.0, 1.0));
	}
}
