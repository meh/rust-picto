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

use iter::Coordinates;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Area {
	pub x: u32,
	pub y: u32,

	pub width:  u32,
	pub height: u32,
}

impl Area {
	pub fn new() -> Builder {
		Default::default()
	}

	#[inline]
	pub fn from(x: u32, y: u32, width: u32, height: u32) -> Self {
		Area {
			x: x,
			y: y,

			width:  width,
			height: height,
		}
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
		Coordinates::new(Area {
			x: 0,
			y: 0,

			width:  self.width,
			height: self.height,
		})
	}
}

#[derive(Eq, PartialEq, Copy, Clone, Default, Debug)]
pub struct Builder {
	pub x: Option<u32>,
	pub y: Option<u32>,

	pub width:  Option<u32>,
	pub height: Option<u32>,
}

impl Builder {
	#[inline]
	pub fn complete(&self, area: Area) -> Area {
		Area {
			x: self.x.unwrap_or(area.x),
			y: self.y.unwrap_or(area.y),

			width:  self.width.unwrap_or(area.width),
			height: self.height.unwrap_or(area.height),
		}
	}

	pub fn unwrap(self) -> Area {
		Area {
			x: self.x.unwrap(),
			y: self.y.unwrap(),

			width:  self.width.unwrap(),
			height: self.height.unwrap(),
		}
	}

	#[inline]
	pub fn x(mut self, value: u32) -> Self {
		self.x = Some(value);
		self
	}

	#[inline]
	pub fn y(mut self, value: u32) -> Self {
		self.y = Some(value);
		self
	}

	#[inline]
	pub fn width(mut self, value: u32) -> Self {
		self.width = Some(value);
		self
	}

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
	fn relative() {
		assert_eq!(vec![(2, 4), (3, 4), (2, 5), (3, 5), (2, 6), (3, 6), (2, 7), (3, 7)],
		 Area::new().x(2).y(4).width(2).height(4).unwrap().relative().collect::<Vec<(u32, u32)>>());
	}

	#[test]
	fn absolute() {
		assert_eq!(vec![(0, 0), (1, 0), (0, 1), (1, 1), (0, 2), (1, 2), (0, 3), (1, 3)],
		 Area::new().x(2).y(4).width(2).height(4).unwrap().absolute().collect::<Vec<(u32, u32)>>());
	}

}
