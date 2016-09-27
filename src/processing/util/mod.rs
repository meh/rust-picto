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

/// Helper macro for `num::cast()`.
macro_rules! num {
	($value:expr => $to:ident) => (
		$crate::num::cast::<_, $to>($value).unwrap()
	);

	($value:expr) => (
		$crate::num::cast($value).unwrap()
	);
}

/// Helper macro for `num::zero()`.
macro_rules! zero {
	() => (
		$crate::num::zero()
	);

	($ty:ty) => (
		$crate::num::zero::<$ty>()
	)
}

/// Helper macro for `num::one()`.
macro_rules! one {
	() => (
		$crate::num::one()
	);

	($ty:ty) => (
		$crate::num::one::<$ty>()
	)
}

mod clamping;
pub use self::clamping::{clamp, Clamped, Get as GetClamped, Set as SetClamped};
