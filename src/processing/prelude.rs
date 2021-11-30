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

pub use crate::processing::sampler;
pub use crate::processing::scaler;
pub use crate::processing::ditherer;

pub use crate::processing::Flip;
pub use crate::processing::Rotate;
pub use crate::processing::Scale;
pub use crate::processing::Sample;
pub use crate::processing::Blur;
pub use crate::processing::Sharpen;
pub use crate::processing::Dither;

/// Flipping orientation.
pub mod flip {
	pub use crate::orientation::Orientation::Vertical as Vertically;
	pub use crate::orientation::Orientation::Horizontal as Horizontally;
}

/// Sampling orientation.
pub mod sample {
	pub use crate::orientation::Orientation::Vertical as Vertically;
	pub use crate::orientation::Orientation::Horizontal as Horizontally;
}
