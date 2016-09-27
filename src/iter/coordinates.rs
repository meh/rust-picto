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

/// Iterator over X and Y coordinates within an `Area`.
#[derive(Eq, PartialEq, Debug)]
pub struct Coordinates {
	x: u32,
	y: u32,

	area: Area,
}

impl Coordinates {
	/// Create a new `Iterator` for the given `Area`.
	#[inline]
	pub fn new(area: Area) -> Self {
		Coordinates {
			x: 0,
			y: 0,

			area: area,
		}
	}

	/// The `Area` being iterated over.
	#[inline]
	pub fn area(&self) -> Area {
		self.area
	}
}

impl Iterator for Coordinates {
	type Item = (u32, u32);

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if self.x >= self.area.width {
			self.x  = 0;
			self.y += 1;
		}

		if self.y >= self.area.height {
			return None;
		}

		self.x += 1;

		Some((self.x - 1 + self.area.x, self.y + self.area.y))
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len(), Some(self.len()))
	}
}

impl ExactSizeIterator for Coordinates {
	#[inline]
	fn len(&self) -> usize {
		let length    = self.area.width * self.area.height;
		let remaining = length - (self.y * self.area.width + self.x);

		remaining as usize
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use area::Area;

	#[test]
	fn size_hint() {
		let mut coord = Coordinates::new(Area::from(0, 0, 2, 2));

		assert_eq!(4, coord.size_hint().0);
		coord.next().unwrap();
		assert_eq!(3, coord.size_hint().0);
		coord.next().unwrap();
		assert_eq!(2, coord.size_hint().0);
		coord.next().unwrap();
		assert_eq!(1, coord.size_hint().0);
		coord.next().unwrap();
		assert_eq!(0, coord.size_hint().0);
	}
}
