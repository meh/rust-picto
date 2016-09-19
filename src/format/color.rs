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

use std::mem;

use num::Float;
use buffer;
use color;

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

impl<T: Float + Copy + 'static> buffer::Valid<u8, color::Rgb<T>> for Color {
	fn valid(&self) -> bool {
		if let Color::Rgb(bits, false) = *self {
			bits == mem::size_of::<u8>() as u8
		}
		else {
			false
		}
	}
}

impl<T: Float + Copy + 'static> buffer::Valid<u8, color::Rgba<T>> for Color {
	fn valid(&self) -> bool {
		if let Color::Rgb(bits, true) = *self {
			bits == mem::size_of::<u8>() as u8
		}
		else {
			false
		}
	}
}

impl<T: Float + Copy + 'static> buffer::Valid<u8, color::Luma<T>> for Color {
	fn valid(&self) -> bool {
		if let Color::Gray(bits, false) = *self {
			bits == mem::size_of::<u8>() as u8
		}
		else {
			false
		}
	}
}

impl<T: Float + Copy + 'static> buffer::Valid<u8, color::Lumaa<T>> for Color {
	fn valid(&self) -> bool {
		if let Color::Gray(bits, true) = *self {
			bits == mem::size_of::<u8>() as u8
		}
		else {
			false
		}
	}
}
