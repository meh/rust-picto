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

use crate::pixel;
use crate::region::{self, Region};
use crate::buffer::Buffer;
use crate::iter::pixel::{Iter as Pixels, IterMut as PixelsMut};
use super::{Read, Write};

/// A view into a `Buffer`.
///
/// The `View` is a readable and writable borrowed region within a `Buffer` and
/// it's parametrized over two types, the `Pixel` and `Channel`.
///
/// The same details on those types from `Buffer` hold true for `View`, except
/// it doesn't own any `Data`.
///
/// There is no functional difference between a `Buffer` and a `View` that
/// encompasses the whole `Buffer` region.
#[derive(PartialEq, Debug)]
pub struct View<'a, P, C>
	where P: pixel::Read<C> + pixel::Write<C>,
	      C: pixel::Channel,
{
	data:   &'a mut [C],
	stride: usize,

	owner:  Region,
	region: Region,

	pixel:   PhantomData<P>,
	channel: PhantomData<C>,
}

impl<'a, P, C> View<'a, P, C>
	where P: pixel::Read<C> + pixel::Write<C>,
	      C: pixel::Channel,
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &mut [C], stride: usize, owner: Region, region: Region) -> View<P, C> {
		View {
			data:   data,
			stride: stride,

			owner:  owner,
			region: region,

			pixel:   PhantomData,
			channel: PhantomData,
		}
	}

	#[inline]
	pub fn from_raw(width: u32, height: u32, data: &mut [C]) -> Result<View<P, C>, ()> {
		if data.len() < width as usize * height as usize * P::channels() {
			return Err(());
		}

		Ok(Self::new(data, width as usize * P::channels(),
			Region::from(0, 0, width, height),
			Region::from(0, 0, width, height)))
	}

	#[inline]
	pub fn with_stride(width: u32, height: u32, stride: usize, data: &mut [C]) -> Result<View<P, C>, ()> {
		if data.len() < stride as usize * height as usize || stride < width as usize * P::channels() {
			return Err(());
		}

		Ok(Self::new(data, stride,
			Region::from(0, 0, width, height),
			Region::from(0, 0, width, height)))
	}

	/// Get the stride.
	#[inline]
	pub fn stride(&self) -> usize {
		self.stride
	}

	/// Get the region.
	#[inline]
	pub fn region(&self) -> Region {
		self.region
	}

	/// Get the width.
	#[inline]
	pub fn width(&self) -> u32 {
		self.region.width
	}

	/// Get the height.
	#[inline]
	pub fn height(&self) -> u32 {
		self.region.height
	}

	/// Get the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it will panic.
	#[inline]
	pub fn get(&self, x: u32, y: u32) -> P {
		Read::new(self.data, self.stride, self.owner, self.region).get(x, y)
	}

	/// Set the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it will panic.
	#[inline]
	pub fn set(&mut self, x: u32, y: u32, pixel: &P) {
		Write::new(self.data, self.stride, self.owner, self.region).set(x, y, pixel)
	}

	/// Get a read-only view of the given region.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`, otherwise it will panic.
	#[inline]
	pub fn readable(&self, region: region::Builder) -> Read<P, C> {
		let region = region.complete(Region::from(0, 0, self.region.width, self.region.height));

		if region.x + region.width > self.region.width || region.y + region.height > self.region.height {
			panic!("out of bounds");
		}

		Read::new(&self.data, self.stride, self.owner, Region { x: region.x + self.region.x, y: region.y + self.region.y, .. region })
	}

	/// Get a write-only view of the given region.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`, otherwise it will panic.
	#[inline]
	pub fn writable(&mut self, region: region::Builder) -> Write<P, C> {
		let region = region.complete(Region::from(0, 0, self.region.width, self.region.height));

		if region.x + region.width > self.region.width || region.y + region.height > self.region.height {
			panic!("out of bounds");
		}

		Write::new(&mut self.data, self.stride, self.owner, Region { x: region.x + self.region.x, y: region.y + self.region.y, .. region })
	}

	/// Get a mutable view of the given region.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`, otherwise it will panic.
	#[inline]
	pub fn view(&mut self, region: region::Builder) -> View<P, C> {
		let region = region.complete(Region::from(0, 0, self.region.width, self.region.height));

		if region.x + region.width > self.region.width || region.y + region.height > self.region.height {
			panic!("out of bounds");
		}

		View::new(&mut self.data, self.stride, self.owner, Region { x: region.x + self.region.x, y: region.y + self.region.y, .. region })
	}

	/// Fill the view with the given pixel.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::Region;
	/// use picto::color::Rgb;
	///
	/// let mut image = read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();
	/// let mut view  = image.view(Region::new().x(10).y(10).width(20).height(30));
	///
	/// // Make a 20x20 pixel region black at offset 10,10.
	/// view.fill(&Rgb::new(0.0, 0.0, 0.0));
	/// ```
	#[inline]
	pub fn fill(&mut self, pixel: &P) {
		self.writable(Default::default()).fill(pixel)
	}

	/// Get a mutable `Iterator` over the pixels.
	pub fn pixels(&self) -> Pixels<P, C> {
		Pixels::new(self.data, self.stride, self.owner, self.region)
	}

	/// Get a mutable `Iterator` over the pixels.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::Region;
	/// use picto::color::{Mix, Rgb};
	///
	/// let mut image = read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();
	/// let mut view  = image.view(Region::new().x(50).y(20));
	///
	/// for (_, _, mut px) in view.pixels_mut() {
	///     // Get the pixel value.
	///     let p = px.get();
	///
	///     // Mix the color with red.
	///     let p = p.mix(&Rgb::new(1.0, 0.0, 0.0), 0.5);
	///
	///     // Set the pixel value.
	///     px.set(&p);
	/// }
	/// ```
	pub fn pixels_mut(&mut self) -> PixelsMut<P, C> {
		PixelsMut::new(self.data, self.stride, self.owner, self.region)
	}

	/// Create a `Buffer` from the `View`.
	pub fn into_owned(&self) -> Buffer<P, C, Vec<C>> {
		let mut buffer = Buffer::new(self.region.width, self.region.height);

		for (x, y, px) in self.pixels() {
			buffer.set(x, y, &px.get());
		}

		buffer
	}

	/// Convert the `View` to a `Buffer` with different channel and pixel type.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::Region;
	/// use picto::color::{Rgb, Rgba};
	///
	/// let mut image = read::from_path::<Rgba, u8, _>("tests/rainbow.png").unwrap();
	/// let     view  = image.view(Region::new().x(10).y(10).width(20).height(20));
	///
	/// // Convert the 20x20 region from Rgba to Rgb.
	/// view.convert::<Rgb, u8>();
	/// ```
	#[inline]
	pub fn convert<PO, CO>(&self) -> Buffer<PO, CO, Vec<CO>>
		where P: Into<PO>,
		      PO: pixel::Write<CO>,
		      CO: pixel::Channel,
	{
		Read::<P, C>::new(self.data, self.stride, self.owner, self.region).convert()
	}

	/// Convert the `View` to a `Buffer` with a closure handling the conversion.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::Region;
	/// use picto::color::{Rgb, Rgba, Srgb};
	///
	/// let mut image = read::from_path::<Rgba, u8, _>("tests/rainbow.png").unwrap();
	/// let     view  = image.view(Region::new().x(10).y(10).width(20).height(20));
	///
	/// // Convert the 20x20 region from Rgba to sRGB.
	/// view.convert_with::<Rgb, f32, _>(|p| Srgb::new(p.red, p.green, p.blue).into());
	/// ```
	#[inline]
	pub fn convert_with<PO, CO, F>(&self, func: F) -> Buffer<PO, CO, Vec<CO>>
		where F:  FnMut(P) -> PO,
		      PO: pixel::Write<CO>,
		      CO: pixel::Channel,
	{
		Read::<P, C>::new(self.data, self.stride, self.owner, self.region).convert_with(func)
	}
}

impl<'a, P, C> From<&'a mut View<'a, P, C>> for View<'a, P, C>
	where P: pixel::Read<C> + pixel::Write<C>,
	      C: pixel::Channel,
{
	#[inline]
	fn from(value: &'a mut View<'a, P, C>) -> View<'a, P, C> {
		View::new(value.data, value.stride, value.owner, value.region)
	}
}

impl<'a, P, C> From<View<'a, P, C>> for Read<'a, P, C>
	where P: pixel::Read<C> + pixel::Write<C>,
	      C: pixel::Channel,
{
	#[inline]
	fn from(value: View<'a, P, C>) -> Read<'a, P, C> {
		Read::new(value.data, value.stride, value.owner, value.region)
	}
}

impl<'a, P, C> From<&'a View<'a, P, C>> for Read<'a, P, C>
	where P: pixel::Read<C> + pixel::Write<C>,
	      C: pixel::Channel,
{
	#[inline]
	fn from(value: &'a View<'a, P, C>) -> Read<'a, P, C> {
		Read::new(value.data, value.stride, value.owner, value.region)
	}
}

impl<'a, P, C> From<View<'a, P, C>> for Write<'a, P, C>
	where P: pixel::Read<C> + pixel::Write<C>,
	      C: pixel::Channel,
{
	#[inline]
	fn from(value: View<'a, P, C>) -> Write<'a, P, C> {
		Write::new(value.data, value.stride, value.owner, value.region)
	}
}

impl<'a, P, C> From<&'a mut View<'a, P, C>> for Write<'a, P, C>
	where P: pixel::Read<C> + pixel::Write<C>,
	      C: pixel::Channel,
{
	#[inline]
	fn from(value: &'a mut View<'a, P, C>) -> Write<'a, P, C> {
		Write::new(value.data, value.stride, value.owner, value.region)
	}
}

#[cfg(test)]
mod test {
	use crate::buffer::Buffer;
	use crate::color::*;
	use crate::region::Region;

	#[test]
	fn pixels_mut() {
		let mut buffer = Buffer::<Rgb, u8, _>::from_raw(2, 2, vec![0, 255, 0, 255, 0, 255, 255, 255, 255, 0, 0, 0]).unwrap();
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
		let mut image = Buffer::<Rgb, u8, Vec<_>>::new(50, 50);
		let     image = image.view(Region::new().x(10).y(10).width(4).height(4));

		assert_eq!(vec![
			(10, 10), (11, 10), (12, 10), (13, 10),
			(10, 11), (11, 11), (12, 11), (13, 11),
			(10, 12), (11, 12), (12, 12), (13, 12),
			(10, 13), (11, 13), (12, 13), (13, 13),
		], image.region().relative().collect::<Vec<_>>());

		let image = image.readable(Region::new().x(1).y(1).width(2).height(2));

		assert_eq!(vec![
			(11, 11), (12, 11),
			(11, 12), (12, 12),
		], image.region().relative().collect::<Vec<_>>());

		let image = image.readable(Region::new().width(2).height(1));

		assert_eq!(vec![
			(11, 11), (12, 11),
		], image.region().relative().collect::<Vec<_>>());
	}

	#[test]
	fn writable() {
		let mut image = Buffer::<Rgb, u8, Vec<_>>::new(50, 50);
		let mut image = image.view(Region::new().x(10).y(10).width(4).height(4));

		assert_eq!(vec![
			(10, 10), (11, 10), (12, 10), (13, 10),
			(10, 11), (11, 11), (12, 11), (13, 11),
			(10, 12), (11, 12), (12, 12), (13, 12),
			(10, 13), (11, 13), (12, 13), (13, 13),
		], image.region().relative().collect::<Vec<_>>());

		let mut image = image.writable(Region::new().x(1).y(1).width(2).height(2));

		assert_eq!(vec![
			(11, 11), (12, 11),
			(11, 12), (12, 12),
		], image.region().relative().collect::<Vec<_>>());

		let image = image.writable(Region::new().width(2).height(1));

		assert_eq!(vec![
			(11, 11), (12, 11),
		], image.region().relative().collect::<Vec<_>>());
	}

	#[test]
	fn view() {
		let mut image = Buffer::<Rgb, u8, Vec<_>>::new(50, 50);
		let mut image = image.view(Region::new().x(10).y(10).width(4).height(4));

		assert_eq!(vec![
			(10, 10), (11, 10), (12, 10), (13, 10),
			(10, 11), (11, 11), (12, 11), (13, 11),
			(10, 12), (11, 12), (12, 12), (13, 12),
			(10, 13), (11, 13), (12, 13), (13, 13),
		], image.region().relative().collect::<Vec<_>>());

		let mut image = image.view(Region::new().x(1).y(1).width(2).height(2));

		assert_eq!(vec![
			(11, 11), (12, 11),
			(11, 12), (12, 12),
		], image.region().relative().collect::<Vec<_>>());

		let image = image.view(Region::new().width(2).height(1));

		assert_eq!(vec![
			(11, 11), (12, 11),
		], image.region().relative().collect::<Vec<_>>());
	}
}
