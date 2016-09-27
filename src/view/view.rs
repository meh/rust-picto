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

use pixel;
use area::{self, Area};
use buffer::Buffer;
use iter::pixel::{Iter as Pixels, IterMut as PixelsMut};
use super::{Read, Write};

/// A view into a `Buffer`.
#[derive(PartialEq, Debug)]
pub struct View<'a, C, P>
	where C: pixel::Channel,
	      P: pixel::Read<C> + pixel::Write<C>
{
	owner: Area,
	area:  Area,
	data:  &'a mut [C],

	_channel: PhantomData<C>,
	_pixel:   PhantomData<P>,
}

impl<'a, C, P> View<'a, C, P>
	where C: pixel::Channel,
	      P: pixel::Read<C> + pixel::Write<C>
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &mut [C], owner: Area, area: Area) -> View<C, P> {
		View {
			owner: owner,
			area:  area,
			data:  data,

			_channel: PhantomData,
			_pixel:   PhantomData,
		}
	}

	/// Get the area.
	#[inline]
	pub fn area(&self) -> Area {
		self.area
	}

	/// Get the width.
	#[inline]
	pub fn width(&self) -> u32 {
		self.area.width
	}

	/// Get the height.
	#[inline]
	pub fn height(&self) -> u32 {
		self.area.height
	}

	/// Get the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it will panic.
	#[inline]
	pub fn get(&self, x: u32, y: u32) -> P {
		Read::new(self.data, self.owner, self.area).get(x, y)
	}

	/// Set the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it will panic.
	#[inline]
	pub fn set(&mut self, x: u32, y: u32, pixel: &P) {
		Write::new(self.data, self.owner, self.area).set(x, y, pixel)
	}

	/// Get a read-only view of the given area.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`, otherwise it will panic.
	#[inline]
	pub fn readable(&self, area: area::Builder) -> Read<C, P> {
		let area = area.complete(Area::from(0, 0, self.area.width, self.area.height));

		if area.x + area.width > self.area.width || area.y + area.height > self.area.height {
			panic!("out of bounds");
		}

		Read::new(&self.data, self.owner, Area { x: area.x + self.area.x, y: area.y + self.area.y, .. area })
	}

	/// Get a write-only view of the given area.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`, otherwise it will panic.
	#[inline]
	pub fn writable(&mut self, area: area::Builder) -> Write<C, P> {
		let area = area.complete(Area::from(0, 0, self.area.width, self.area.height));

		if area.x + area.width > self.area.width || area.y + area.height > self.area.height {
			panic!("out of bounds");
		}

		Write::new(&mut self.data, self.owner, Area { x: area.x + self.area.x, y: area.y + self.area.y, .. area })
	}

	/// Get a mutable view of the given sub-image.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`, otherwise it will panic.
	#[inline]
	pub fn view(&mut self, area: area::Builder) -> View<C, P> {
		let area = area.complete(Area::from(0, 0, self.area.width, self.area.height));

		if area.x + area.width > self.area.width || area.y + area.height > self.area.height {
			panic!("out of bounds");
		}

		View::new(&mut self.data, self.owner, Area { x: area.x + self.area.x, y: area.y + self.area.y, .. area })
	}

	/// Get a mutable iterator over the view's pixels.
	pub fn pixels(&self) -> Pixels<C, P> {
		Pixels::new(self.data, self.owner, self.area)
	}

	/// Get a mutable iterator over the view's pixels.
	pub fn pixels_mut(&mut self) -> PixelsMut<C, P> {
		PixelsMut::new(self.data, self.owner, self.area)
	}

	/// Create a `Buffer` from the `View`.
	pub fn into_owned(&self) -> Buffer<C, P, Vec<C>> {
		let mut buffer = Buffer::new(self.area.width, self.area.height);

		for (x, y, px) in self.pixels() {
			buffer.set(x, y, &px.get());
		}

		buffer
	}

	/// Convert the `View` to a `Buffer` with different channel and pixel type.
	#[inline]
	pub fn convert<CO, PO>(&self) -> Buffer<CO, PO, Vec<CO>>
		where CO: pixel::Channel,
		      PO: pixel::Write<CO>,
		      P: Into<PO>
	{
		Read::<C, P>::new(self.data, self.owner, self.area).convert()
	}
}

impl<'a, C, P> From<&'a mut View<'a, C, P>> for View<'a, C, P>
	where C: pixel::Channel,
	      P: pixel::Read<C> + pixel::Write<C>
{
	#[inline]
	fn from(value: &'a mut View<'a, C, P>) -> View<'a, C, P> {
		View::new(value.data, value.owner, value.area)
	}
}

impl<'a, C, P> From<View<'a, C, P>> for Read<'a, C, P>
	where C: pixel::Channel,
	      P: pixel::Read<C> + pixel::Write<C>
{
	#[inline]
	fn from(value: View<'a, C, P>) -> Read<'a, C, P> {
		Read::new(value.data, value.owner, value.area)
	}
}

impl<'a, C, P> From<&'a View<'a, C, P>> for Read<'a, C, P>
	where C: pixel::Channel,
	      P: pixel::Read<C> + pixel::Write<C>
{
	#[inline]
	fn from(value: &'a View<'a, C, P>) -> Read<'a, C, P> {
		Read::new(value.data, value.owner, value.area)
	}
}

impl<'a, C, P> From<View<'a, C, P>> for Write<'a, C, P>
	where C: pixel::Channel,
	      P: pixel::Read<C> + pixel::Write<C>
{
	#[inline]
	fn from(value: View<'a, C, P>) -> Write<'a, C, P> {
		Write::new(value.data, value.owner, value.area)
	}
}

impl<'a, C, P> From<&'a mut View<'a, C, P>> for Write<'a, C, P>
	where C: pixel::Channel,
	      P: pixel::Read<C> + pixel::Write<C>
{
	#[inline]
	fn from(value: &'a mut View<'a, C, P>) -> Write<'a, C, P> {
		Write::new(value.data, value.owner, value.area)
	}
}

#[cfg(test)]
mod test {
	use buffer::*;
	use color::*;
	use area::Area;

	#[test]
	fn pixels_mut() {
		let mut buffer = Buffer::<u8, Rgb, _>::from_raw(2, 2, vec![0, 255, 0, 255, 0, 255, 255, 255, 255, 0, 0, 0]).unwrap();
		let mut view = buffer.view(Default::default());

		for (x, y, mut px) in view.pixels_mut() {
			assert!(x <= 1 && y <= 1);

			let inverted = Rgb::new(1.0, 1.0, 1.0) - px.get();
			px.set(&inverted);
		}

		assert_eq!(view.get(0, 0), Rgb::new(1.0, 0.0, 1.0));
		assert_eq!(view.get(1, 0), Rgb::new(0.0, 1.0, 0.0));
		assert_eq!(view.get(0, 1), Rgb::new(0.0, 0.0, 0.0));
		assert_eq!(view.get(1, 1), Rgb::new(1.0, 1.0, 1.0));
	}

	#[test]
	fn readable() {
		let mut image = Buffer::<u8, Rgb, Vec<_>>::new(50, 50);
		let     image = image.view(Area::new().x(10).y(10).width(4).height(4));

		assert_eq!(vec![
			(10, 10), (11, 10), (12, 10), (13, 10),
			(10, 11), (11, 11), (12, 11), (13, 11),
			(10, 12), (11, 12), (12, 12), (13, 12),
			(10, 13), (11, 13), (12, 13), (13, 13),
		], image.area().relative().collect::<Vec<_>>());

		let image = image.readable(Area::new().x(1).y(1).width(2).height(2));

		assert_eq!(vec![
			(11, 11), (12, 11),
			(11, 12), (12, 12),
		], image.area().relative().collect::<Vec<_>>());

		let image = image.readable(Area::new().width(2).height(1));

		assert_eq!(vec![
			(11, 11), (12, 11),
		], image.area().relative().collect::<Vec<_>>());
	}

	#[test]
	fn writable() {
		let mut image = Buffer::<u8, Rgb, Vec<_>>::new(50, 50);
		let mut image = image.view(Area::new().x(10).y(10).width(4).height(4));

		assert_eq!(vec![
			(10, 10), (11, 10), (12, 10), (13, 10),
			(10, 11), (11, 11), (12, 11), (13, 11),
			(10, 12), (11, 12), (12, 12), (13, 12),
			(10, 13), (11, 13), (12, 13), (13, 13),
		], image.area().relative().collect::<Vec<_>>());

		let mut image = image.writable(Area::new().x(1).y(1).width(2).height(2));

		assert_eq!(vec![
			(11, 11), (12, 11),
			(11, 12), (12, 12),
		], image.area().relative().collect::<Vec<_>>());

		let image = image.writable(Area::new().width(2).height(1));

		assert_eq!(vec![
			(11, 11), (12, 11),
		], image.area().relative().collect::<Vec<_>>());
	}

	#[test]
	fn view() {
		let mut image = Buffer::<u8, Rgb, Vec<_>>::new(50, 50);
		let mut image = image.view(Area::new().x(10).y(10).width(4).height(4));

		assert_eq!(vec![
			(10, 10), (11, 10), (12, 10), (13, 10),
			(10, 11), (11, 11), (12, 11), (13, 11),
			(10, 12), (11, 12), (12, 12), (13, 12),
			(10, 13), (11, 13), (12, 13), (13, 13),
		], image.area().relative().collect::<Vec<_>>());

		let mut image = image.view(Area::new().x(1).y(1).width(2).height(2));

		assert_eq!(vec![
			(11, 11), (12, 11),
			(11, 12), (12, 12),
		], image.area().relative().collect::<Vec<_>>());

		let image = image.view(Area::new().width(2).height(1));

		assert_eq!(vec![
			(11, 11), (12, 11),
		], image.area().relative().collect::<Vec<_>>());
	}
}
