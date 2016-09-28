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
