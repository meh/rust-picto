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

use crate::iter::Coordinates;

/// A region within a buffer or view.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Region {
	pub x: u32,
	pub y: u32,

	pub width: u32,
	pub height: u32,
}

impl Region {
	/// Create a `Builder` to create an `Region`.
	pub fn new() -> Builder {
		Default::default()
	}

	/// Create an `Region` from the given parameters.
	#[inline]
	pub fn from(x: u32, y: u32, width: u32, height: u32) -> Self {
		Region { x, y, width, height }
	}

	/// Get an iterator over relative coordinates, based on the coordinates
	/// within the parent view.
	#[inline]
	pub fn relative(&self) -> Coordinates {
		Coordinates::new(*self)
	}

	/// Get an iterator over absolute coordinates.
	#[inline]
	pub fn absolute(&self) -> Coordinates {
		Coordinates::new(Region {
			x: 0,
			y: 0,

			width: self.width,
			height: self.height,
		})
	}
}

#[derive(Eq, PartialEq, Copy, Clone, Default, Debug)]
pub struct Builder {
	pub x: Option<u32>,
	pub y: Option<u32>,

	pub width: Option<u32>,
	pub height: Option<u32>,
}

impl Builder {
	/// Complete any missing parts of the `Builder` and return the resulting
	/// `Region`.
	#[inline]
	pub fn complete(&self, region: Region) -> Region {
		let (x, width) = if let Some(x) = self.x {
			if let Some(width) = self.width {
				(x, width)
			}
			else {
				(x, region.width - x)
			}
		}
		else {
			(region.x, self.width.unwrap_or(region.width))
		};

		let (y, height) = if let Some(y) = self.y {
			if let Some(height) = self.height {
				(y, height)
			}
			else {
				(y, region.height - y)
			}
		}
		else {
			(region.y, self.height.unwrap_or(region.height))
		};

		Region { x, y, width, height }
	}

	/// Create an `Region` based on the `Builder` state.
	#[inline]
	pub fn with<F: FnMut(&Builder) -> Region>(&self, mut func: F) -> Region {
		func(self)
	}

	/// Builds an `Region` panicking if any fields are missing.
	#[inline]
	pub fn unwrap(self) -> Region {
		Region {
			x: self.x.unwrap(),
			y: self.y.unwrap(),

			width: self.width.unwrap(),
			height: self.height.unwrap(),
		}
	}

	/// Set the X value.
	#[inline]
	pub fn x(mut self, value: u32) -> Self {
		self.x = Some(value);
		self
	}

	/// Set the Y value.
	#[inline]
	pub fn y(mut self, value: u32) -> Self {
		self.y = Some(value);
		self
	}

	/// Set the width.
	#[inline]
	pub fn width(mut self, value: u32) -> Self {
		self.width = Some(value);
		self
	}

	/// Set the height.
	#[inline]
	pub fn height(mut self, value: u32) -> Self {
		self.height = Some(value);
		self
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn complete() {
		assert_eq!(
			Region::from(5, 0, 5, 10),
			Region::new().x(5).complete(Region::from(0, 0, 10, 10))
		);

		assert_eq!(
			Region::from(0, 0, 5, 10),
			Region::new().width(5).complete(Region::from(0, 0, 10, 10))
		);
	}

	#[test]
	fn relative() {
		assert_eq!(
			vec![(2, 4), (3, 4), (2, 5), (3, 5), (2, 6), (3, 6), (2, 7), (3, 7)],
			Region::new()
				.x(2)
				.y(4)
				.width(2)
				.height(4)
				.unwrap()
				.relative()
				.collect::<Vec<(u32, u32)>>()
		);
	}

	#[test]
	fn absolute() {
		assert_eq!(
			vec![(0, 0), (1, 0), (0, 1), (1, 1), (0, 2), (1, 2), (0, 3), (1, 3)],
			Region::new()
				.x(2)
				.y(4)
				.width(2)
				.height(4)
				.unwrap()
				.absolute()
				.collect::<Vec<(u32, u32)>>()
		);
	}
}
