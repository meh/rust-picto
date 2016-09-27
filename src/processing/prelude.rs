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

pub use processing::sampler;
pub use processing::scaler;

pub use processing::Flip;
pub use processing::Scale;

/// Flipping orientation.
pub mod flip {
	pub use processing::flip::Orientation::Vertical as Vertically;
	pub use processing::flip::Orientation::Horizontal as Horizontally;
}
