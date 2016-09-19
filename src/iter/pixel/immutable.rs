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

/// Immutable iterator over pixels.
pub struct Iter<'a, C: pixel::Channel, P: Pixel<C> + pixel::Read<C>> {
	x: u32,
	y: u32,

	area: Area,
	data: &'a [C],

	_channel: PhantomData<C>,
	_pixel:   PhantomData<P>,
}

impl<'a, C, P> Iter<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C>
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &[C], area: Area) -> Iter<C, P> {
		Iter {
			x: area.x,
			y: area.y,

			area: area,
			data: data,

			_channel: PhantomData,
			_pixel:   PhantomData,
		}
	}
}

/// A readable pixel from the iterator.
pub struct Item<'a, C: pixel::Channel, P: Pixel<C> + pixel::Read<C>> {
	data: &'a [C],

	_channel: PhantomData<C>,
	_pixel:   PhantomData<P>,
}

impl<'a, C, P> Item<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C>
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &[C]) -> Item<C, P> {
		Item {
			data: data,

			_channel: PhantomData,
			_pixel:   PhantomData,
		}
	}

	/// Get the pixel value.
	pub fn get(&self) -> P {
		P::read(self.data)
	}
}

impl<'a, C, P> Iterator for Iter<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C>
{
	type Item = (u32, u32, Item<'a, C, P>);

	fn next(&mut self) -> Option<Self::Item> {
		if self.x == self.area.width {
			self.x  = 0;
			self.y += 1;
		}

		if self.y == self.area.height {
			return None;
		}

		let channels = P::channels();
		let index    = channels * ((self.area.y + self.y) as usize * self.area.width as usize + (self.area.x + self.x) as usize);
		let item     = (
			self.x - self.area.x,
			self.y - self.area.y,

			Item::new(&self.data[index .. index + channels])
		);

		self.x += 1;
		Some(item)
	}
}
