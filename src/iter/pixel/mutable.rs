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
use iter::Coordinates;

/// Mutable iterator over pixels.
pub struct Iter<'a, C: pixel::Channel, P: Pixel<C> + pixel::Read<C> + pixel::Write<C>> {
	inner: Coordinates,
	owner: Area,
	data:  &'a mut [C],

	_channel: PhantomData<C>,
	_pixel:   PhantomData<P>,
}

impl<'a, C, P> Iter<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C> + pixel::Write<C>
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &mut [C], owner: Area, area: Area) -> Iter<C, P> {
		Iter {
			inner: Coordinates::new(area),
			owner: owner,
			data:  data,

			_channel: PhantomData,
			_pixel:   PhantomData,
		}
	}
}

/// A readable and writable pixel from the iterator.
#[derive(Eq, PartialEq, Debug)]
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
	#[inline]
	pub fn get(&self) -> P {
		P::read(self.data)
	}

	/// Set the pixel value.
	#[inline]
	pub fn set(&mut self, pixel: &P) {
		pixel.write(self.data)
	}
}

impl<'a, C, P> Iterator for Iter<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C> + pixel::Write<C>
{
	type Item = (u32, u32, Item<'a, C, P>);

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		let (x, y) = if let Some((x, y)) = self.inner.next() {
			(x, y)
		}
		else {
			return None;
		};

		let channels = P::channels();
		let index    = channels * (y as usize * self.owner.width as usize + x as usize);

		Some((
			x - self.inner.area().x,
			y - self.inner.area().y,

			Item::new(unsafe {
				let slice = &self.data[index .. index + channels];
				slice::from_raw_parts_mut(slice.as_ptr() as *mut _, slice.len())
			})
		))
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.inner.size_hint()
	}
}

impl<'a, C, P> ExactSizeIterator for Iter<'a, C, P>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C> + pixel::Write<C>
{
	#[inline]
	fn len(&self) -> usize {
		self.inner.len()
	}
}
