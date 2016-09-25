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

#[macro_use]
pub mod util;
pub mod prelude;

pub mod sampler;
pub use self::sampler::Sampler;

pub mod scaler;
pub use self::scaler::Scaler;

pub mod flip;
pub use self::flip::Flip;

pub mod scale;
pub use self::scale::Scale;

pub mod sample;
