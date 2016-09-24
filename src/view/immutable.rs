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
use buffer::Buffer;
use area::{self, Area};
use iter::pixel::Iter as Pixels;

/// An immutable view into a `Buffer`.
#[derive(PartialEq, Debug)]
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

	/// Get an immutable iterator over the view's pixels.
	pub fn pixels(&self) -> Pixels<C, P> {
		Pixels::new(self.data, self.area)
	}

	/// Convert the `Buffer` to another `Buffer` with different channel and pixel type.
	#[inline]
	pub fn convert<CO, PO>(&self) -> Buffer<CO, PO, Vec<CO>>
		where CO: pixel::Channel,
		      PO: Pixel<CO> + pixel::Write<CO>,
		      P: Into<PO>
	{
		let mut result = Buffer::<CO, PO, Vec<_>>::new(self.area.width, self.area.height);

		for (x, y) in self.area.absolute() {
			result.set(x, y, &self.get(x, y).into());
		}

		result
	}
}

impl<'a, C, P> From<&'a Ref<'a, C, P>> for Ref<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C>
{
	#[inline]
	fn from(value: &'a Ref<'a, C, P>) -> Ref<'a, C, P> {
		Ref::new(value.data, value.area)
	}
}

#[cfg(test)]
mod test {
	use buffer::*;
	use color::*;
	use area::Area;

	#[test]
	fn get() {
		assert_eq!(Rgb::new(1.0, 0.0, 1.0),
			Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![255, 0, 255]).unwrap().get(0, 0));

		assert_eq!(Rgba::new(0.0, 1.0, 1.0, 0.0),
			Buffer::<u8, Rgba, _>::from_raw(1, 2, vec![255, 0, 255, 0, 0, 255, 255, 0]).unwrap().get(0, 1));
	}

	#[test]
	fn as_ref() {
		let image = Buffer::<u8, Rgb, Vec<_>>::new(50, 50);
		let image = image.as_ref(Area::new().x(10).y(10).width(4).height(4));

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
}
