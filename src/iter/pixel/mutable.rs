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

use std::slice;
use std::marker::PhantomData;

use pixel::{self, Pixel};
use area::Area;

/// Mutable iterator over pixels.
pub struct Iter<'a, C: pixel::Channel, P: Pixel<C> + pixel::Read<C> + pixel::Write<C>> {
	x: u32,
	y: u32,

	area: Area,
	data: &'a mut [C],

	_channel: PhantomData<C>,
	_pixel:   PhantomData<P>,
}

impl<'a, C, P> Iter<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C> + pixel::Write<C>
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &mut [C], area: Area) -> Iter<C, P> {
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

/// A readable and writable pixel from the iterator.
pub struct Item<'a, C: pixel::Channel, P: Pixel<C> + pixel::Read<C> + pixel::Write<C>> {
	data: &'a mut [C],

	_channel: PhantomData<C>,
	_pixel:   PhantomData<P>,
}

impl<'a, C, P> Item<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C> + pixel::Write<C>
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &mut [C]) -> Item<C, P> {
		Item {
			data: data,

			_channel: PhantomData,
			_pixel:   PhantomData,
		}
	}

	/// Get the pixel value.
	pub fn get<T: From<P>>(&self) -> T {
		T::from(P::read(self.data))
	}

	/// Set the pixel value.
	pub fn set<T: Into<P>>(&mut self, pixel: T) {
		pixel.into().write(self.data)
	}
}

impl<'a, C, P> Iterator for Iter<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C> + pixel::Write<C>
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

			Item::new(unsafe {
				let slice = &self.data[index .. index + channels];
				slice::from_raw_parts_mut(slice.as_ptr() as *mut _, slice.len())
			})
		);

		self.x += 1;
		Some(item)
	}
}
