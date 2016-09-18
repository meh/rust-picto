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
	pub fn complete(&self, x: u32, y: u32, width: u32, height: u32) -> Area {
		Area {
			x: self.x.unwrap_or(x),
			y: self.y.unwrap_or(y),

			width:  self.width.unwrap_or(width),
			height: self.height.unwrap_or(height),
		}
	}

	#[inline]
	pub fn x(&mut self, value: u32) -> &mut Self {
		self.x = Some(value);
		self
	}

	#[inline]
	pub fn y(&mut self, value: u32) -> &mut Self {
		self.y = Some(value);
		self
	}

	#[inline]
	pub fn width(&mut self, value: u32) -> &mut Self {
		self.width = Some(value);
		self
	}

	#[inline]
	pub fn height(&mut self, value: u32) -> &mut Self {
		self.height = Some(value);
		self
	}
}
