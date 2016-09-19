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

use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

use num::Zero;
use pixel::{self, Pixel};
use view::{self, View};
use area::{self, Area};
use iter::pixel::{Iter as Pixels, IterMut as PixelsMut};

/// Buffer for an image.
#[derive(PartialEq, Debug)]
pub struct Buffer<C: pixel::Channel, P: Pixel<C>, D> {
	width:  u32,
	height: u32,

	data: D,

	_channel: PhantomData<C>,
	_pixel:   PhantomData<P>,
}

impl<C, P> Buffer<C, P, Vec<C>>
	where C: pixel::Channel,
	      P: Pixel<C>,
{
	/// Create a new `Buffer` with the requested space allocated.
	#[inline]
	pub fn new(width: u32, height: u32) -> Self {
		Buffer {
			width:  width,
			height: height,

			data: vec![Zero::zero(); width as usize * height as usize * P::channels()],

			_channel: PhantomData,
			_pixel:   PhantomData,
		}
	}
}

impl<C, P> Buffer<C, P, Vec<C>>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Write<C>,
{
	/// Create a new `Buffer` with the request space allocated and filled with
	/// the given pixel.
	pub fn from_pixel(width: u32, height: u32, pixel: P) -> Self {
		let mut buffer = Self::new(width, height);

		for x in 0 .. width {
			for y in 0 .. height {
				buffer.set(x, y, &pixel);
			}
		}

		buffer
	}

	/// Create a new `Buffer` with the request space allocated and filled with
	/// the pixel returned by the given function.
	///
	/// The function takes the coordinates and returns a pixel.
	pub fn from_fn<T, F>(width: u32, height: u32, mut func: F) -> Self
		where T: Into<P>,
		      F: FnMut(u32, u32) -> T
	{
		let mut buffer = Self::new(width, height);

		for x in 0 .. width {
			for y in 0 .. height {
				buffer.set(x, y, &func(x, y).into());
			}
		}

		buffer
	}
}

impl<C, P, D> Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C>,
	      D: Deref<Target = [C]>
{
	/// Use an existing container as backing storage for an image `Buffer`.
	#[inline]
	pub fn from_raw(width: u32, height: u32, data: D) -> Result<Self, ()> {
		if width as usize * height as usize * P::channels() != data.len() {
			return Err(());
		}

		Ok(Buffer {
			width:  width,
			height: height,

			data: data,

			_channel: PhantomData,
			_pixel:   PhantomData,
		})
	}
}

impl<C, P, D> Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C>
{
	/// Get the backing storage of the `Buffer`.
	#[inline]
	pub fn into_raw(self) -> D {
		self.data
	}

	#[inline]
	pub fn area(&self) -> Area {
		Area::from(0, 0, self.width, self.height)
	}

	/// Get the dimensions.
	#[inline]
	pub fn dimensions(&self) -> (u32, u32) {
		(self.width, self.height)
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

impl<C, P, D> Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C>,
	      D: Deref<Target = [C]>
{
	/// Get the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it will panic.
	#[inline]
	pub fn get(&self, x: u32, y: u32) -> P {
		view::Ref::new(&self.data, self.area()).get(x, y)
	}

	/// Get an immutable view of the given sub-image.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`, otherwise it will panic.
	#[inline]
	pub fn as_ref(&self, area: area::Builder) -> view::Ref<C, P> {
		let area = area.complete(0, 0, self.width, self.height);

		if area.x + area.width > self.width || area.y + area.height > self.height {
			panic!("out of bounds");
		}

		view::Ref::new(&self.data, area)
	}

	#[inline]
	pub fn pixels(&self) -> Pixels<C, P> {
		Pixels::new(&self.data, self.area())
	}
}

impl<C, P, D> Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Write<C>,
	      D: DerefMut<Target = [C]>
{
	/// Set the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it will panic.
	#[inline]
	pub fn set(&mut self, x: u32, y: u32, pixel: &P) {
		let area = self.area();
		view::Mut::new(&mut self.data, area).set(x, y, pixel)
	}

	/// Get a mutable view of the given sub-image.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`, otherwise it will panic.
	#[inline]
	pub fn as_mut(&mut self, area: area::Builder) -> view::Mut<C, P> {
		let area = area.complete(0, 0, self.width, self.height);

		if area.x + area.width > self.width || area.y + area.height > self.height {
			panic!("out of bounds");
		}

		view::Mut::new(&mut self.data, area)
	}
}

impl<C, P, D> Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Write<C> + pixel::Read<C>,
	      D: DerefMut<Target = [C]>
{
	/// Get a view of the given sub-image.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`, otherwise it will panic.
	#[inline]
	pub fn view(&mut self, area: area::Builder) -> View<C, P> {
		let area = area.complete(0, 0, self.width, self.height);

		if area.x + area.width > self.width || area.y + area.height > self.height {
			panic!("out of bounds");
		}

		View::new(&mut self.data, area)
	}

	#[inline]
	pub fn pixels_mut(&mut self) -> PixelsMut<C, P> {
		let area = self.area();
		PixelsMut::new(&mut self.data, area)
	}
}

impl<C, P, D> Deref for Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C>,
	      D: Deref<Target = [C]>
{
	type Target = D::Target;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<C, P, D> DerefMut for Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C>,
	      D: DerefMut<Target = [C]>
{
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}

impl<C, P, D> Clone for Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C>,
	      D: Clone
{
	#[inline]
	fn clone(&self) -> Self {
		Buffer {
			width:  self.width,
			height: self.height,

			data: self.data.clone(),

			_channel: PhantomData,
			_pixel:   PhantomData,
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use color::*;

	#[test]
	fn new() {
		assert_eq!(3, Buffer::<u8, Rgb, Vec<_>>::new(1, 1).into_raw().len());
		assert_eq!(6, Buffer::<u8, Rgb, Vec<_>>::new(1, 2).into_raw().len());
		assert_eq!(6, Buffer::<u8, Rgb, Vec<_>>::new(2, 1).into_raw().len());
		assert_eq!(12, Buffer::<u8, Rgb, Vec<_>>::new(2, 2).into_raw().len());
	}

	#[test]
	fn from_raw() {
		assert!(Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![0, 0, 0]).is_ok());
		assert!(Buffer::<u8, Rgb, _>::from_raw(1, 2, vec![0, 0, 0, 0, 0, 0]).is_ok());
		assert!(Buffer::<u8, Rgb, _>::from_raw(2, 1, vec![0, 0, 0, 0, 0, 0]).is_ok());
		assert!(Buffer::<u8, Rgb, _>::from_raw(2, 2, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).is_ok());

		assert!(Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![0, 0, 0, 0]).is_err());
		assert!(Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![0, 0, 0, 0]).is_err());
	}

	#[test]
	fn into_raw() {
		assert_eq!(vec![1, 2, 3],
			Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![1, 2, 3]).unwrap().into_raw());

		assert_eq!(vec![0, 0, 0],
			Buffer::<u8, Rgb, Vec<_>>::new(1, 1).into_raw());
	}

	#[test]
	fn deref() {
		assert!(Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![0, 0, 0]).unwrap().len() == 3);
	}

	#[test]
	fn clone() {
		let a = Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![0, 0, 0]).unwrap();
		let b = a.clone();

		assert_eq!(a.get(0, 0), b.get(0, 0));
	}

	#[test]
	fn eq() {
		let a = Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![0, 0, 0]).unwrap();
		let b = a.clone();

		assert_eq!(a, b);
	}
}
