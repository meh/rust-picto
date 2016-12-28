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
use region::{self, Region};
use iter::pixel::Iter as Pixels;

/// A read-only view into a `Buffer`.
///
/// The `view::Read` is a readable borrowed region within a `Buffer` and it's
/// parametrized over two types, the `Pixel` and `Channel`.
///
/// The same details on those types from `Buffer` hold true for `View`, except
/// it doesn't own any `Data`.
///
/// There is no functional difference between an immutable `Buffer` and a
/// `view::Read` that encompasses the whole `Buffer` region.
#[derive(PartialEq, Debug)]
pub struct Read<'a, P, C>
	where P: pixel::Read<C>,
	      C: pixel::Channel,
{
	data:   &'a [C],
	stride: usize,

	owner:  Region,
	region: Region,

	pixel:   PhantomData<P>,
	channel: PhantomData<C>,
}

impl<'a, P, C> Read<'a, P, C>
	where P: pixel::Read<C>,
	      C: pixel::Channel,
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &[C], stride: usize, owner: Region, region: Region) -> Read<P, C> {
		Read {
			data:   data,
			stride: stride,

			owner:  owner,
			region: region,

			pixel:   PhantomData,
			channel: PhantomData,
		}
	}

	#[inline]
	pub fn from_raw(width: u32, height: u32, data: &[C]) -> Result<Read<P, C>, ()> {
		if data.len() < width as usize * height as usize * P::channels() {
			return Err(());
		}

		Ok(Self::new(data, width as usize * P::channels(),
			Region::from(0, 0, width, height),
			Region::from(0, 0, width, height)))
	}

	#[inline]
	pub fn with_stride(width: u32, height: u32, stride: usize, data: &[C]) -> Result<Read<P, C>, ()> {
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

	/// Get the dimensions as a tuple containing width and height.
	#[inline]
	pub fn dimensions(&self) -> (u32, u32) {
		(self.region.width, self.region.height)
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
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it
	/// will panic.
	#[inline]
	pub fn get(&self, x: u32, y: u32) -> P {
		if x >= self.region.width || y >= self.region.height {
			panic!("out of bounds");
		}

		let channels = P::channels();
		let index    = ((self.region.y + y) as usize * self.stride)
			+ ((self.region.x + x) as usize * channels);

		P::read(&self.data[index .. index + channels])
	}

	/// Get a read-only view of the given region, refining further from the
	/// current.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`,
	/// otherwise it will panic.
	#[inline]
	pub fn readable(&self, region: region::Builder) -> Read<P, C> {
		let region = region.complete(Region::from(0, 0, self.region.width, self.region.height));

		if region.x + region.width > self.region.width || region.y + region.height > self.region.height {
			panic!("out of bounds");
		}

		Read::new(&self.data, self.stride, self.owner, Region { x: region.x + self.region.x, y: region.y + self.region.y, .. region })
	}

	/// Get an immutable `Iterator` over the pixels.
	pub fn pixels(&self) -> Pixels<P, C> {
		Pixels::new(self.data, self.stride, self.owner, self.region)
	}

	/// Convert the `view::Read` to another `Buffer` with different channel and
	/// pixel type.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::Region;
	/// use picto::color::{Rgb, Rgba};
	///
	/// let image = read::from_path::<Rgba, u8, _>("tests/rainbow.png").unwrap();
	/// let view  = image.readable(Region::new().x(10).y(10).width(20).height(20));
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
		let mut result = Buffer::<PO, CO, Vec<_>>::new(self.region.width, self.region.height);

		for (x, y) in self.region.absolute() {
			result.set(x, y, &self.get(x, y).into());
		}

		result
	}

	/// Convert the `view::Read` to a `Buffer` with a closure handling the
	/// conversion.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::Region;
	/// use picto::color::{Rgb, Rgba, Srgb};
	///
	/// let mut image = read::from_path::<Rgba, u8, _>("tests/rainbow.png").unwrap();
	/// let     view  = image.readable(Region::new().x(10).y(10).width(20).height(20));
	///
	/// // Convert the 20x20 region from Rgba to sRGB.
	/// view.convert_with::<Rgb, f32, _>(|p| Srgb::new(p.red, p.green, p.blue).into());
	/// ```
	#[inline]
	pub fn convert_with<PO, CO, F>(&self, mut func: F) -> Buffer<PO, CO, Vec<CO>>
		where F:  FnMut(P) -> PO,
		      PO: pixel::Write<CO>,
		      CO: pixel::Channel,
	{
		let mut result = Buffer::<PO, CO, Vec<_>>::new(self.region.width, self.region.height);

		for (x, y) in self.region.absolute() {
			result.set(x, y, &func(self.get(x, y)));
		}

		result
	}
}

impl<'a, P, C> From<&'a Read<'a, P, C>> for Read<'a, P, C>
	where P: pixel::Read<C>,
	      C: pixel::Channel,
{
	#[inline]
	fn from(value: &'a Read<'a, P, C>) -> Read<'a, P, C> {
		Read::new(value.data, value.stride, value.owner, value.region)
	}
}

#[cfg(test)]
mod test {
	use buffer::Buffer;
	use color::*;
	use region::Region;

	#[test]
	fn get() {
		let image = Buffer::<Rgba, u8, _>::from_fn(20, 20, |x, y| {
			let w = (x as f32 + y as f32) / 40.0;
			Rgba::new(w, w, w, w)
		});

		let view = image.readable(Region::new().x(10).y(10).width(10).height(10));
		assert_relative_eq!(Rgba::new(0.5, 0.5, 0.5, 0.5),
			view.get(0, 0), epsilon = 0.01);
	}

	#[test]
	fn readable() {
		let image = Buffer::<Rgb, u8, Vec<_>>::new(50, 50);
		let image = image.readable(Region::new().x(10).y(10).width(4).height(4));

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
}
