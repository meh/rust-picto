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

/// An orientation, for flipping, sampling, etc.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Orientation {
	Vertical,
	Horizontal,
}

impl Orientation {
	/// Reverse the orientation.
	#[inline]
	pub fn rev(&self) -> Self {
		match *self {
			Orientation::Vertical => Orientation::Horizontal,

			Orientation::Horizontal => Orientation::Vertical,
		}
	}
}
