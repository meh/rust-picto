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
pub enum Color {
	Gray(u8, bool),
	Rgb(u8, bool),
	Palette(u8),
}

impl Color {
	#[inline]
	pub fn depth(&self) -> usize {
		match *self {
			Color::Gray(n, _) => self.channels() * n as usize,
			Color::Rgb(n, _)  => self.channels() * n as usize,
			Color::Palette(n) => self.channels() * n as usize,
		}
	}

	#[inline]
	pub fn channels(&self) -> usize {
		match *self {
			Color::Gray(_, false) => 1,
			Color::Gray(_, true)  => 2,
			Color::Rgb(_, false)  => 3,
			Color::Rgb(_, true)   => 4,
			Color::Palette(_)     => 3,
		}
	}
}
