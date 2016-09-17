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

/// Buffer for an image.
pub struct Buffer<C: pixel::Channel, P: Pixel<C>, D> {
	width:  u32,
	height: u32,

	data:    D,
	channel: PhantomData<C>,
	pixel:   PhantomData<P>,
}

impl<C, P> Buffer<C, P, Vec<C>>
	where C: pixel::Channel,
	      P: Pixel<C>,
{
	/// Create a new `Buffer` with the requested space allocated.
	pub fn new(width: u32, height: u32) -> Self {
		Buffer {
			width:  width,
			height: height,

			data:    vec![Zero::zero(); width as usize * height as usize * P::channels()],
			channel: PhantomData,
			pixel:   PhantomData,
		}
	}
}

impl<C, P, D> Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Read<C>,
	      D: Deref<Target = [C]>
{
	/// Use an existing container as backing storage for an image `Buffer`.
	pub fn from_raw(width: u32, height: u32, data: D) -> Result<Self, ()> {
		if width as usize * height as usize * P::channels() != data.len() {
			return Err(());
		}

		Ok(Buffer {
			width:  width,
			height: height,

			data:    data,
			channel: PhantomData,
			pixel:   PhantomData,
		})
	}

	/// Get the `Pixel` at the given coordinates.
	pub fn get(&self, x: u32, y: u32) -> Option<P> {
		if x >= self.width || y >= self.height {
			return None;
		}

		let channels = P::channels();
		let index    = channels * (y as usize * self.width as usize + x as usize);

		Some(P::read(&self.data[index .. index + channels]))
	}
}

impl<C, P, D> Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C> + pixel::Write<C>,
	      D: DerefMut<Target = [C]>
{
	/// Set the `Pixel` at the given coordinates.
	pub fn set(&mut self, x: u32, y: u32, value: &P) -> Result<(), ()> {
		if x >= self.width || y >= self.height {
			return Err(());
		}

		let channels = P::channels();
		let index    = channels * (y as usize * self.width as usize + x as usize);

		value.write(&mut self.data[index .. index + channels]);

		Ok(())
	}
}

impl<C, P, D> Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C>
{
	/// Get the backing storage of the `Buffer`.
	pub fn into_raw(self) -> D {
		self.data
	}

	/// Get the dimensions.
	pub fn dimensions(&self) -> (u32, u32) {
		(self.width, self.height)
	}

	/// Get the width.
	pub fn width(&self) -> u32 {
		self.width
	}

	/// Get the height.
	pub fn height(&self) -> u32 {
		self.height
	}
}

impl<C, P, D> Deref for Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C>,
	      D: Deref<Target = [C]>
{
	type Target = D::Target;

	fn deref(&self) -> &Self::Target {
		&self.data
	}
}


impl<C, P, D> DerefMut for Buffer<C, P, D>
	where C: pixel::Channel,
	      P: Pixel<C>,
	      D: DerefMut<Target = [C]>
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
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
	fn get() {
		assert_eq!(Some(Rgb::new(1.0, 0.0, 1.0)),
			Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![255, 0, 255]).unwrap().get(0, 0));

		assert_eq!(Some(Rgba::new(0.0, 1.0, 1.0, 0.0)),
			Buffer::<u8, Rgba, _>::from_raw(1, 2, vec![255, 0, 255, 0, 0, 255, 255, 0]).unwrap().get(0, 1));
	}

	#[test]
	fn set() {
		let mut image = Buffer::<u8, Rgb, Vec<_>>::new(2, 2);
		assert!(image.set(0, 0, &Rgb::new(1.0, 0.0, 1.0)).is_ok());

		assert_eq!(Some(Rgb::new(1.0, 0.0, 1.0)),
			image.get(0, 0));
	}

	#[test]
	fn deref() {
		assert!(Buffer::<u8, Rgb, _>::from_raw(1, 1, vec![0, 0, 0]).unwrap().len() == 3);
	}
}
