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

use pixel;
use area::Area;
use iter::Coordinates;

/// Mutable iterator over pixels.
pub struct Iter<'a, P, C>
	where P: pixel::Read<C> + pixel::Write<C>,
	      C: pixel::Channel,
{
	inner: Coordinates,
	owner: Area,

	pixel:   PhantomData<P>,
	channel: PhantomData<C>,
	data:  &'a mut [C],
}

impl<'a, P, C> Iter<'a, P, C>
	where P: pixel::Read<C> + pixel::Write<C>,
	      C: pixel::Channel,
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &mut [C], owner: Area, area: Area) -> Iter<P, C> {
		Iter {
			inner: Coordinates::new(area),
			owner: owner,

			pixel:   PhantomData,
			channel: PhantomData,
			data:    data,
		}
	}
}

/// A readable and writable pixel from the iterator.
#[derive(Eq, PartialEq, Debug)]
pub struct Item<'a, P, C>
	where P: pixel::Read<C> + pixel::Write<C>,
	      C: pixel::Channel,
{
	pixel:   PhantomData<P>,
	channel: PhantomData<C>,
	data:    &'a mut [C],
}

impl<'a, P, C> Item<'a, P, C>
	where P: pixel::Read<C> + pixel::Write<C>,
	      C: pixel::Channel,
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &mut [C]) -> Item<P, C> {
		Item {
			pixel:   PhantomData,
			channel: PhantomData,
			data:    data,
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

impl<'a, P, C> Iterator for Iter<'a, P, C>
	where P: pixel::Read<C> + pixel::Write<C>,
	      C: pixel::Channel,
{
	type Item = (u32, u32, Item<'a, P, C>);

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

impl<'a, P, C> ExactSizeIterator for Iter<'a, P, C>
	where P: pixel::Read<C> + pixel::Write<C>,
	      C: pixel::Channel,
{
	#[inline]
	fn len(&self) -> usize {
		self.inner.len()
	}
}
