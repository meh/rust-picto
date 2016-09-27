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

/// Miscellaneous processing utilities.
#[macro_use]
pub mod util;

/// Module grouping all the processing functionality for easier usage.
///
/// # Example
///
/// ```
/// use picto::Buffer;
/// use picto::color::Rgb;
/// use picto::processing::prelude::*;
///
/// Buffer::<u8, Rgb, _>::from_pixel(1, 1, &Rgb::new(1.0, 0.0, 0.0))
/// 	.scale_by::<scaler::Nearest, u8, Rgb>(20.0);
/// ```
pub mod prelude;

/// Image sampling algorithms.
pub mod sampler;
pub use self::sampler::Sampler;

/// Image scaling algorithms.
pub mod scaler;
pub use self::scaler::Scaler;

/// Image flipping.
pub mod flip;
pub use self::flip::Flip;

/// Image scaling.
pub mod scale;
pub use self::scale::Scale;

/// Image sampling.
pub mod sample;
