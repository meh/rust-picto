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
use buffer::Buffer;
use area::{self, Area};
use iter::pixel::Iter as Pixels;

/// A read-only view into a `Buffer`.
#[derive(PartialEq, Debug)]
pub struct Read<'a, C, P>
	where C: pixel::Channel,
	      P: pixel::Read<C>
{
	owner: Area,
	area:  Area,
	data:  &'a [C],

	_channel: PhantomData<C>,
	_pixel:   PhantomData<P>,
}

impl<'a, C, P> Read<'a, C, P>
	where C: pixel::Channel,
	      P: pixel::Read<C>
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &[C], owner: Area, area: Area) -> Read<C, P> {
		Read {
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
		if x >= self.area.width || y >= self.area.height {
			panic!("out of bounds");
		}

		let channels = P::channels();
		let index    = channels * ((self.area.y + y) as usize * self.owner.width as usize + (self.area.x + x) as usize);

		P::read(&self.data[index .. index + channels])
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

	/// Get an immutable iterator over the view's pixels.
	pub fn pixels(&self) -> Pixels<C, P> {
		Pixels::new(self.data, self.owner, self.area)
	}

	/// Convert the `Buffer` to another `Buffer` with different channel and pixel type.
	#[inline]
	pub fn convert<CO, PO>(&self) -> Buffer<CO, PO, Vec<CO>>
		where CO: pixel::Channel,
		      PO: pixel::Write<CO>,
		      P: Into<PO>
	{
		let mut result = Buffer::<CO, PO, Vec<_>>::new(self.area.width, self.area.height);

		for (x, y) in self.area.absolute() {
			result.set(x, y, &self.get(x, y).into());
		}

		result
	}
}

impl<'a, C, P> From<&'a Read<'a, C, P>> for Read<'a, C, P>
	where C: pixel::Channel,
	      P: pixel::Read<C>
{
	#[inline]
	fn from(value: &'a Read<'a, C, P>) -> Read<'a, C, P> {
		Read::new(value.data, value.owner, value.area)
	}
}

#[cfg(test)]
mod test {
	use buffer::*;
	use color::*;
	use area::Area;

	#[test]
	fn get() {
		let image = Buffer::<u8, Rgba, _>::from_fn(20, 20, |x, y| {
			let w = (x as f32 + y as f32) / 40.0;
			Rgba::new(w, w, w, w)
		});

		let view = image.readable(Area::new().x(10).y(10).width(10).height(10));
		assert_relative_eq!(Rgba::new(0.5, 0.5, 0.5, 0.5),
			view.get(0, 0), epsilon = 0.01);
	}

	#[test]
	fn readable() {
		let image = Buffer::<u8, Rgb, Vec<_>>::new(50, 50);
		let image = image.readable(Area::new().x(10).y(10).width(4).height(4));

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
}
