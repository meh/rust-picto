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
use area::{self, Area};
use buffer::Buffer;
use iter::pixel::{Iter as Pixels, IterMut as PixelsMut};
use super::{Ref, Mut};

/// A view into a `Buffer`.
#[derive(PartialEq, Debug)]
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

	/// Get an immutable view of the given sub-image.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`, otherwise it will panic.
	#[inline]
	pub fn as_ref(&self, area: area::Builder) -> Ref<C, P> {
		let area = area.complete(Area::from(0, 0, self.area.width, self.area.height));

		if area.x + area.width > self.area.width || area.y + area.height > self.area.height {
			panic!("out of bounds");
		}

		Ref::new(&self.data, Area { x: area.x + self.area.x, y: area.y + self.area.y, .. area })
	}

	/// Get a mutable view of the given sub-image.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`, otherwise it will panic.
	#[inline]
	pub fn as_mut(&mut self, area: area::Builder) -> Mut<C, P> {
		let area = area.complete(Area::from(0, 0, self.area.width, self.area.height));

		if area.x + area.width > self.area.width || area.y + area.height > self.area.height {
			panic!("out of bounds");
		}

		Mut::new(&mut self.data, Area { x: area.x + self.area.x, y: area.y + self.area.y, .. area })
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

		View::new(&mut self.data, Area { x: area.x + self.area.x, y: area.y + self.area.y, .. area })
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

	/// Convert the `View` to a `Buffer` with different channel and pixel type.
	#[inline]
	pub fn convert<CO, PO>(&self) -> Buffer<CO, PO, Vec<CO>>
		where CO: pixel::Channel,
		      PO: Pixel<CO> + pixel::Write<CO>,
		      P: Into<PO>
	{
		Ref::<C, P>::new(self.data, self.area).convert()
	}
}

impl<'a, C, P> From<&'a mut View<'a, C, P>> for View<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C> + pixel::Write<C>
{
	#[inline]
	fn from(value: &'a mut View<'a, C, P>) -> View<'a, C, P> {
		View::new(value.data, value.area)
	}
}

impl<'a, C, P> From<View<'a, C, P>> for Ref<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C> + pixel::Write<C>
{
	#[inline]
	fn from(value: View<'a, C, P>) -> Ref<'a, C, P> {
		Ref::new(value.data, value.area)
	}
}

impl<'a, C, P> From<&'a View<'a, C, P>> for Ref<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C> + pixel::Write<C>
{
	#[inline]
	fn from(value: &'a View<'a, C, P>) -> Ref<'a, C, P> {
		Ref::new(value.data, value.area)
	}
}

impl<'a, C, P> From<View<'a, C, P>> for Mut<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C> + pixel::Write<C>
{
	#[inline]
	fn from(value: View<'a, C, P>) -> Mut<'a, C, P> {
		Mut::new(value.data, value.area)
	}
}

impl<'a, C, P> From<&'a mut View<'a, C, P>> for Mut<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C> + pixel::Write<C>
{
	#[inline]
	fn from(value: &'a mut View<'a, C, P>) -> Mut<'a, C, P> {
		Mut::new(value.data, value.area)
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
	fn as_ref() {
		let mut image = Buffer::<u8, Rgb, Vec<_>>::new(50, 50);
		let     image = image.view(Area::new().x(10).y(10).width(4).height(4));

		assert_eq!(vec![
			(10, 10), (11, 10), (12, 10), (13, 10),
			(10, 11), (11, 11), (12, 11), (13, 11),
			(10, 12), (11, 12), (12, 12), (13, 12),
			(10, 13), (11, 13), (12, 13), (13, 13),
		], image.area().relative().collect::<Vec<_>>());

		let image = image.as_ref(Area::new().x(1).y(1).width(2).height(2));

		assert_eq!(vec![
			(11, 11), (12, 11),
			(11, 12), (12, 12),
		], image.area().relative().collect::<Vec<_>>());

		let image = image.as_ref(Area::new().width(2).height(1));

		assert_eq!(vec![
			(11, 11), (12, 11),
		], image.area().relative().collect::<Vec<_>>());
	}

	#[test]
	fn as_mut() {
		let mut image = Buffer::<u8, Rgb, Vec<_>>::new(50, 50);
		let mut image = image.view(Area::new().x(10).y(10).width(4).height(4));

		assert_eq!(vec![
			(10, 10), (11, 10), (12, 10), (13, 10),
			(10, 11), (11, 11), (12, 11), (13, 11),
			(10, 12), (11, 12), (12, 12), (13, 12),
			(10, 13), (11, 13), (12, 13), (13, 13),
		], image.area().relative().collect::<Vec<_>>());

		let mut image = image.as_mut(Area::new().x(1).y(1).width(2).height(2));

		assert_eq!(vec![
			(11, 11), (12, 11),
			(11, 12), (12, 12),
		], image.area().relative().collect::<Vec<_>>());

		let image = image.as_mut(Area::new().width(2).height(1));

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
