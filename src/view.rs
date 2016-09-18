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

/// A view into a `Buffer`.
pub struct View<'a, C: pixel::Channel, P: Pixel<C>> {
	x: u32,
	y: u32,

	width:  u32,
	height: u32,

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
	pub fn new(data: &mut [C], x: u32, y: u32, width: u32, height: u32) -> View<C, P> {
		View {
			x: x,
			y: y,

			width:  width,
			height: height,

			data:    data,
			channel: PhantomData,
			pixel:   PhantomData,
		}
	}

	/// Get the offset.
	#[inline]
	pub fn x(&self) -> u32 {
		self.x
	}

	/// Get the offset.
	#[inline]
	pub fn y(&self) -> u32 {
		self.y
	}

	/// Get the width.
	#[inline]
	pub fn width(&self) -> u32 {
		self.width
	}

	/// Get the height.
	#[inline]
	pub fn height(&self) -> u32 {
		self.height
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
		Ref::new(self.data, self.x, self.y, self.width, self.height).get(x, y)
	}

	/// Set the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it will panic.
	#[inline]
	pub fn set(&mut self, x: u32, y: u32, pixel: &P) {
		Mut::new(self.data, self.x, self.y, self.width, self.height).set(x, y, pixel)
	}

	/// Transform the pixels within the view.
	///
	/// The passed function takes the `x`, `y` and the pixel value and returns a
	/// new pixel value.
	#[inline]
	pub fn transform<T: Into<P>, F: FnMut(u32, u32, P) -> T>(&mut self, mut func: F) {
		for x in 0 .. self.width {
			for y in 0 .. self.height {
				let px = self.get(x, y);
				self.set(x, y, &func(x, y, px).into());
			}
		}
	}
}

/// An immutable view into a `Buffer`.
pub struct Ref<'a, C: pixel::Channel, P: Pixel<C>> {
	x: u32,
	y: u32,

	width:  u32,
	height: u32,

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
	pub fn new(data: &[C], x: u32, y: u32, width: u32, height: u32) -> Ref<C, P> {
		Ref {
			x: x,
			y: y,

			width:  width,
			height: height,

			data:    data,
			channel: PhantomData,
			pixel:   PhantomData,
		}
	}

	/// Get the offset.
	#[inline]
	pub fn x(&self) -> u32 {
		self.x
	}

	/// Get the offset.
	#[inline]
	pub fn y(&self) -> u32 {
		self.y
	}

	/// Get the width.
	#[inline]
	pub fn width(&self) -> u32 {
		self.width
	}

	/// Get the height.
	#[inline]
	pub fn height(&self) -> u32 {
		self.height
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
		if x >= self.width || y >= self.height {
			panic!("out of bounds");
		}

		let channels = P::channels();
		let index    = channels * ((self.y + y) as usize * self.width as usize + (self.x + x) as usize);

		P::read(&self.data[index .. index + channels])
	}
}

/// A mutable view into a `Buffer`.
pub struct Mut<'a, C: pixel::Channel, P: Pixel<C>> {
	x: u32,
	y: u32,

	width:  u32,
	height: u32,

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
	pub fn new(data: &mut [C], x: u32, y: u32, width: u32, height: u32) -> Mut<C, P> {
		Mut {
			x: x,
			y: y,

			width:  width,
			height: height,

			data:    data,
			channel: PhantomData,
			pixel:   PhantomData,
		}
	}

	/// Get the offset.
	#[inline]
	pub fn x(&self) -> u32 {
		self.x
	}

	/// Get the offset.
	#[inline]
	pub fn y(&self) -> u32 {
		self.y
	}

	/// Get the width.
	#[inline]
	pub fn width(&self) -> u32 {
		self.width
	}

	/// Get the height.
	#[inline]
	pub fn height(&self) -> u32 {
		self.height
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
		if x >= self.width || y >= self.height {
			panic!("out of bounds");
		}

		let channels = P::channels();
		let index    = channels * ((self.y + y) as usize * self.width as usize + (self.x + x) as usize);

		value.write(&mut self.data[index .. index + channels]);
	}
}

#[cfg(test)]
mod test {
	use buffer::*;
	use color::*;

	#[test]
	fn transform() {
		let mut buffer = Buffer::<u8, Rgb, _>::from_raw(2, 2, vec![0, 255, 0, 255, 0, 255, 255, 255, 255, 0, 0, 0]).unwrap();
		let mut view   = buffer.view(0, 0, 2, 2);

		view.transform(|_, _, px| {
			Rgb::new(1.0, 1.0, 1.0) - px
		});

		assert_eq!(view.get(0, 0), Rgb::new(1.0, 0.0, 1.0));
	}
}
