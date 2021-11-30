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

use crate::orientation::Orientation;
use crate::pixel::{self, Pixel};
use crate::view::{self, View};
use crate::region::{self, Region};
use crate::color;
use crate::iter::pixel::{Iter as Pixels, IterMut as PixelsMut};

/// Buffer for an image.
///
/// The `Buffer` is parametrized over three types, the `Pixel`, the `Channel`
/// and the `Data`, and it's the owner of the `Data`.
///
/// The `Pixel` is out-of-the-box handled by the `palette` crate, but it could
/// be any other type behaving like a color.
///
/// The `Channel` is a primitive type from which the `Pixel` can be read from
/// or written to, this is typically `u8`.
///
/// The `Data` is the backing storage which contains a serie of `Channel` in
/// amount equal to `Pixel::channels() * width * height`.
///
///	The most common `Buffer` types are available in the `buffer` module:
///
/// - `buffer::Luma` is an alias for `Buffer<color::Luma, u8, Vec<u8>>`
/// - `buffer::Lumaa` is an alias for `Buffer<color::Lumaa, u8, Vec<u8>>`
/// - `buffer::Rgb` is an alias for `Buffer<color::Rgb, u8, Vec<u8>>`
/// - `buffer::Rgba` is an alias for `Buffer<color::Rgba, u8, Vec<u8>>`
///
/// # Notes
///
/// The `Data` can be any type, but most functionality will only be available
/// when that type implements `Deref<Target = [Channel]>`.
///
/// This in practice means that for example you could use a `Box<[u8]>` as
/// `Data` and almost everything would work like it were a `Vec<u8>`.
#[derive(Clone, PartialEq, Debug)]
pub struct Buffer<P, C, D>
	where P: Pixel<C>,
	      C: pixel::Channel,
{
	region: Region,

	data:   D,
	stride: usize,

	pixel:   PhantomData<P>,
	channel: PhantomData<C>,
}

impl<P, C> Buffer<P, C, Vec<C>>
	where P: Pixel<C>,
	      C: pixel::Channel,
{
	/// Create a new `Buffer` with the requested space allocated and all channels
	/// set to `0`.
	///
	/// # Example
	///
	/// ```
	/// use picto::Buffer;
	/// use picto::color::Rgb;
	///
	/// Buffer::<Rgb, u8, _>::new(1024, 1024);
	/// ```
	#[inline]
	pub fn new(width: u32, height: u32) -> Self {
		Buffer {
			region: Region::from(0, 0, width, height),

			data:   vec![zero!(); width as usize * height as usize * P::channels()],
			stride: width as usize * P::channels(),

			channel: PhantomData,
			pixel:   PhantomData,
		}
	}
}

impl<P, C> Buffer<P, C, Vec<C>>
	where P: pixel::Write<C>,
	      C: pixel::Channel,
{
	/// Create a new `Buffer` with the request space allocated and filled with
	/// the given pixel.
	///
	/// # Example
	///
	/// ```
	/// use picto::Buffer;
	/// use picto::color::Rgb;
	///
	/// Buffer::<Rgb, u8, _>::from_pixel(1024, 1024, &Rgb::new(1.0, 0.0, 0.0));
	/// ```
	#[inline]
	pub fn from_pixel(width: u32, height: u32, pixel: &P) -> Self {
		let mut buffer = Self::new(width, height);
		buffer.fill(pixel);

		buffer
	}

	/// Create a new `Buffer` with the request space allocated and filled with
	/// the pixel returned by the given function.
	///
	/// The function takes the coordinates and returns a pixel.
	///
	/// # Example
	///
	/// ```
	/// use picto::Buffer;
	/// use picto::color::Rgb;
	///
	/// Buffer::<Rgb, u8, _>::from_fn(1024, 1024, |x, y| {
	///     let w = (x as f32 + y as f32) / 2048.0;
	///     Rgb::new(w, w, w)
	/// });
	/// ```
	#[inline]
	pub fn from_fn<T, F>(width: u32, height: u32, mut func: F) -> Self
		where T: Into<P>,
		      F: FnMut(u32, u32) -> T
	{
		let mut buffer = Self::new(width, height);

		for (x, y) in buffer.region().absolute() {
			buffer.set(x, y, &func(x, y).into());
		}

		buffer
	}
}

impl<P, C> Buffer<P, C, Vec<C>>
	where P: pixel::Write<C> + color::Mix + Clone,
	      C: pixel::Channel,
{
	/// Create a `Buffer` from an orientation and a gradient.
	///
	/// # Example
	///
	/// ```
	/// use picto::{Buffer, Orientation};
	/// use picto::color::{Rgb, Gradient};
	///
	/// Buffer::<Rgb, u8, _>::from_gradient(1024, 1024, Orientation::Horizontal, Gradient::new(
	///     vec![Rgb::new(0.0, 0.0, 0.0), Rgb::new(1.0, 1.0, 1.0), Rgb::new(0.0, 0.0, 0.0)]));
	/// ```
	#[inline]
	pub fn from_gradient(width: u32, height: u32, mode: Orientation, gradient: color::Gradient<P>) -> Self {
		let mut buffer = Buffer::new(width, height);

		match mode {
			Orientation::Vertical => {
				for (y, px) in (0 .. height).zip(gradient.take(height as usize)) {
					for x in 0 .. width {
						buffer.set(x, y, &px);
					}
				}
			}

			Orientation::Horizontal => {
				for (x, px) in (0 .. width).zip(gradient.take(width as usize)) {
					for y in 0 .. height {
						buffer.set(x, y, &px);
					}
				}
			}
		}

		buffer
	}
}

impl<P, C, D> Buffer<P, C, D>
	where P: Pixel<C>,
	      C: pixel::Channel,
	      D: Deref<Target = [C]>,
{
	/// Use an existing container as backing storage for an image `Buffer`.
	///
	/// The size of the storage is compared against the supplied dimensions and
	/// `P::channel()`.
	///
	/// # Example
	///
	/// ```
	/// use picto::Buffer;
	/// use picto::color::Rgb;
	///
	/// Buffer::<Rgb, u8, _>::from_raw(2, 2, vec![
	///     255,   0,   0,
	///       0, 255,   0,
	///       0,   0, 255,
	///     255,   0, 255,
	/// ]).unwrap();
	/// ```
	#[inline]
	pub fn from_raw(width: u32, height: u32, data: D) -> Result<Self, ()> {
		if data.len() < width as usize * height as usize * P::channels() {
			return Err(());
		}

		Ok(Buffer {
			region: Region::from(0, 0, width, height),

			data:   data,
			stride: width as usize * P::channels(),

			pixel:   PhantomData,
			channel: PhantomData,
		})
	}
}

impl<P, C, D> Buffer<P, C, D>
	where P: Pixel<C>,
	      C: pixel::Channel,
{
	/// Get the backing storage of the `Buffer`.
	#[inline]
	pub fn into_raw(self) -> D {
		self.data
	}

	/// Get the stride.
	#[inline]
	pub fn stride(&self) -> usize {
		self.stride
	}

	/// Get the `Region` of the `Buffer`.
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
}

impl<P, C, D> Buffer<P, C, D>
	where P: pixel::Read<C>,
	      C: pixel::Channel,
	      D: Deref<Target = [C]>
{
	/// Get the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it will panic.
	#[inline]
	pub fn get(&self, x: u32, y: u32) -> P {
		view::Read::new(&self.data, self.stride, self.region, self.region).get(x, y)
	}

	/// Get a read-only of the given region.
	///
	/// Passing `Default::default()` as `region` will create a view on the whole
	/// `Buffer`.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`, otherwise it will panic.
	#[inline]
	pub fn readable(&self, region: region::Builder) -> view::Read<P, C> {
		let region = region.complete(self.region);

		if region.x + region.width > self.region.width || region.y + region.height > self.region.height {
			panic!("out of bounds");
		}

		view::Read::new(&self.data, self.stride, self.region, region)
	}

	/// Get an immutable `Iterator` over the pixels.
	#[inline]
	pub fn pixels(&self) -> Pixels<P, C> {
		Pixels::new(&self.data, self.stride, self.region, self.region)
	}

	/// Convert the `Buffer` to another `Buffer` with different channel and pixel type.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::Region;
	/// use picto::color::{Rgb, Lumaa};
	///
	/// let image = read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();
	///
	/// // Convert the `Buffer` from Rgb to grayscale with alpha.
	/// image.convert::<Lumaa, u8>();
	/// ```
	#[inline]
	pub fn convert<PO, CO>(&self) -> Buffer<PO, CO, Vec<CO>>
		where P:  Into<PO>,
		      PO: pixel::Write<CO>,
		      CO: pixel::Channel,
	{
		let mut result = Buffer::<PO, CO, Vec<CO>>::new(self.region.width, self.region.height);

		for (input, output) in self.chunks(P::channels()).zip(result.chunks_mut(PO::channels())) {
			P::read(input).into().write(output)
		}

		result
	}

	/// Convert the `Buffer` to another `Buffer` with a closure handling the
	/// conversion.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::Region;
	/// use picto::color::{Rgb, Srgb};
	///
	/// let image = read::from_path::<Rgb, u8, _>("tests/rainbow.png").unwrap();
	///
	/// // Conver the `Buffer` to the sRGB color space.
	/// image.convert_with::<Rgb, f32, _>(|p| Srgb::new(p.red, p.green, p.blue).into());
	/// ```
	#[inline]
	pub fn convert_with<PO, CO, F>(&self, mut func: F) -> Buffer<PO, CO, Vec<CO>>
		where F:  FnMut(P) -> PO,
		      PO: pixel::Write<CO>,
		      CO: pixel::Channel,
	{
		let mut result = Buffer::<PO, CO, Vec<CO>>::new(self.region.width, self.region.height);

		for (input, output) in self.chunks(P::channels()).zip(result.chunks_mut(PO::channels())) {
			func(P::read(input)).write(output)
		}

		result
	}
}

impl<P, C, D> Buffer<P, C, D>
	where P: pixel::Write<C>,
	      C: pixel::Channel,
	      D: DerefMut<Target = [C]>,
{
	/// Set the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it will panic.
	#[inline]
	pub fn set(&mut self, x: u32, y: u32, pixel: &P) {
		view::Write::new(&mut self.data, self.stride, self.region, self.region).set(x, y, pixel)
	}

	/// Get a write-only view of the given region.
	///
	/// Passing `Default::default()` as `region` will create a view on the whole
	/// `Buffer`.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`, otherwise it will panic.
	#[inline]
	pub fn writable(&mut self, region: region::Builder) -> view::Write<P, C> {
		let region = region.complete(self.region);

		if region.x + region.width > self.region.width || region.y + region.height > self.region.height {
			panic!("out of bounds");
		}

		view::Write::new(&mut self.data, self.stride, self.region, region)
	}

	/// Fill the buffer with the given pixel.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::color::Rgb;
	///
	/// let mut image = read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();
	/// image.fill(&Rgb::new(1.0, 1.0, 1.0));
	/// ```
	#[inline]
	pub fn fill(&mut self, pixel: &P) {
		for chunk in self.chunks_mut(P::channels()) {
			pixel.write(chunk);
		}
	}
}

impl<P, C, D> Buffer<P, C, D>
	where P: pixel::Write<C> + pixel::Read<C>,
	      C: pixel::Channel,
	      D: DerefMut<Target = [C]>,
{
	/// Get a view of the given region.
	///
	/// Passing `Default::default()` as `region` will create a view on the whole
	/// `Buffer`.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <= self.height()`,
	/// otherwise it will panic.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::Region;
	/// use picto::color::Rgba;
	///
	/// let mut image = read::from_path::<Rgba, u8, _>("tests/boat.xyz").unwrap();
	/// let mut view  = image.view(Region::new().x(10).y(10).width(20).height(30));
	///
	/// for (_, _, mut px) in view.pixels_mut() {
	///     // Get the current value.
	///     let p = px.get();
	///
	///     // Make it opaque.
	///     px.set(&Rgba { alpha: 0.5, .. p });
	/// }
	/// ```
	#[inline]
	pub fn view(&mut self, region: region::Builder) -> View<P, C> {
		let region = region.complete(self.region);

		if region.x + region.width > self.region.width || region.y + region.height > self.region.height {
			panic!("out of bounds");
		}

		View::new(&mut self.data, self.stride, self.region, region)
	}

	/// Get a mutable `Iterator` over the pixels.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::color::{IntoColor, Hue, RgbHue, Rgb};
	///
	/// let mut image = read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();
	///
	/// for (x, y, mut px) in image.pixels_mut() {
	///     // Get the pixel value.
	///     let p = px.get();
	///
	///     // Convert to HSL and shift the hue.
	///     let p = p.into_hsl().shift_hue(RgbHue::from_radians(90.0));
	///
	///     // Set the pixel value.
	///     px.set(&p.into());
	/// }
	/// ```
	#[inline]
	pub fn pixels_mut(&mut self) -> PixelsMut<P, C> {
		PixelsMut::new(&mut self.data, self.stride, self.region, self.region)
	}
}

impl<'a, P, C, D> From<&'a Buffer<P, C, D>> for view::Read<'a, P, C>
	where P: pixel::Read<C>,
	      C: pixel::Channel,
	      D: Deref<Target = [C]>,
{
	#[inline]
	fn from(value: &'a Buffer<P, C, D>) -> view::Read<'a, P, C> {
		value.readable(Default::default())
	}
}

impl<'a, P, C, D> From<&'a mut Buffer<P, C, D>> for view::Write<'a, P, C>
	where P: pixel::Write<C>,
	      C: pixel::Channel,
	      D: DerefMut<Target = [C]>,
{
	#[inline]
	fn from(mut value: &'a mut Buffer<P, C, D>) -> view::Write<'a, P, C> {
		value.writable(Default::default())
	}
}

impl<'a, P, C, D> From<&'a mut Buffer<P, C, D>> for View<'a, P, C>
	where P: pixel::Write<C> + pixel::Read<C>,
	      C: pixel::Channel,
	      D: DerefMut<Target = [C]>,
{
	#[inline]
	fn from(mut value: &'a mut Buffer<P, C, D>) -> View<'a, P, C> {
		value.view(Default::default())
	}
}

impl<P, C, D> Deref for Buffer<P, C, D>
	where P: Pixel<C>,
	      C: pixel::Channel,
	      D: Deref<Target = [C]>,
{
	type Target = D::Target;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<P, C, D> DerefMut for Buffer<P, C, D>
	where P: Pixel<C>,
	      C: pixel::Channel,
	      D: DerefMut<Target = [C]>,
{
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::color::*;

	#[test]
	fn new() {
		assert_eq!(3, Buffer::<Rgb, u8, Vec<_>>::new(1, 1).into_raw().len());
		assert_eq!(6, Buffer::<Rgb, u8, Vec<_>>::new(1, 2).into_raw().len());
		assert_eq!(6, Buffer::<Rgb, u8, Vec<_>>::new(2, 1).into_raw().len());
		assert_eq!(12, Buffer::<Rgb, u8, Vec<_>>::new(2, 2).into_raw().len());
	}

	#[test]
	fn from_raw() {
		assert!(Buffer::<Rgb, u8, _>::from_raw(1, 1, vec![0, 0, 0]).is_ok());
		assert!(Buffer::<Rgb, u8, _>::from_raw(1, 2, vec![0, 0, 0, 0, 0, 0]).is_ok());
		assert!(Buffer::<Rgb, u8, _>::from_raw(2, 1, vec![0, 0, 0, 0, 0, 0]).is_ok());
		assert!(Buffer::<Rgb, u8, _>::from_raw(2, 2, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).is_ok());

		assert!(Buffer::<Rgb, u8, _>::from_raw(1, 1, vec![0, 0]).is_err());
	}

	#[test]
	fn into_raw() {
		assert_eq!(vec![1, 2, 3],
			Buffer::<Rgb, u8, _>::from_raw(1, 1, vec![1, 2, 3]).unwrap().into_raw());

		assert_eq!(vec![0, 0, 0],
			Buffer::<Rgb, u8, Vec<_>>::new(1, 1).into_raw());
	}

	#[test]
	fn deref() {
		assert!(Buffer::<Rgb, u8, _>::from_raw(1, 1, vec![0, 0, 0]).unwrap().len() == 3);
	}

	#[test]
	fn clone() {
		let a = Buffer::<Rgb, u8, _>::from_raw(1, 1, vec![0, 0, 0]).unwrap();
		let b = a.clone();

		assert_eq!(a.get(0, 0), b.get(0, 0));
	}

	#[test]
	fn eq() {
		let a = Buffer::<Rgb, u8, _>::from_raw(1, 1, vec![0, 0, 0]).unwrap();
		let b = a.clone();

		assert_eq!(a, b);
	}

	#[test]
	fn convert() {
		let a = Buffer::<Rgb, u8, _>::from_raw(1, 1, vec![255, 0, 255]).unwrap();
		let b = a.convert::<Rgba, u8>();

		assert_eq!(Rgba::new(1.0, 0.0, 1.0, 1.0),
			b.get(0, 0));

		assert_eq!(vec![255, 0, 255, 255],
			b.into_raw());
	}
}
