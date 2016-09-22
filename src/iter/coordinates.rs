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

use area::Area;

#[derive(Eq, PartialEq, Debug)]
pub struct Coordinates {
	x: u32,
	y: u32,

	inner: Area,
}

impl Coordinates {
	pub fn new(area: Area) -> Self {
		Coordinates {
			x: 0,
			y: 0,

			inner: area,
		}
	}

	#[inline]
	pub fn x(&self) -> u32 {
		self.x
	}

	#[inline]
	pub fn y(&self) -> u32 {
		self.y
	}

	#[inline]
	pub fn width(&self) -> u32 {
		self.inner.width
	}

	#[inline]
	pub fn height(&self) -> u32 {
		self.inner.height
	}

	#[inline]
	pub fn area(&self) -> Area {
		self.inner
	}
}

impl Iterator for Coordinates {
	type Item = (u32, u32);

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if self.x >= self.inner.width {
			self.x  = 0;
			self.y += 1;
		}

		if self.y >= self.inner.height {
			return None;
		}

		self.x += 1;

		Some((self.x - 1 + self.inner.x , self.y + self.inner.y))
	}
}
