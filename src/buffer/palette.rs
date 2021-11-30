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

use super::Buffer;
use crate::{
	color::{ComponentWise, Limited, Mix, Saturate, Shade},
	pixel,
	util::GetClamped,
};

impl<P, C> Shade for Buffer<P, C, Vec<C>>
where
	P: Shade,
	P: pixel::Read<C> + pixel::Write<C>,
	C: pixel::Channel,
{
	type Scalar = P::Scalar;

	#[inline]
	fn lighten(&self, amount: Self::Scalar) -> Self {
		let mut output = Buffer::<P, C, _>::new(self.width(), self.height());

		for ((_, _, i), (_, _, mut o)) in self.pixels().zip(output.pixels_mut()) {
			o.set(&i.get().lighten(amount));
		}

		output
	}
}

impl<P, C> Mix for Buffer<P, C, Vec<C>>
where
	P: Mix,
	P: pixel::Read<C> + pixel::Write<C>,
	C: pixel::Channel,
{
	type Scalar = P::Scalar;

	#[inline]
	fn mix(&self, other: &Self, factor: Self::Scalar) -> Self {
		let mut output = Buffer::<P, C, _>::new(self.width(), self.height());

		for ((x, y, i), (_, _, mut o)) in self.pixels().zip(output.pixels_mut()) {
			o.set(&i.get().mix(&other.get_clamped(x as i64, y as i64), factor));
		}

		output
	}
}

impl<P, C> Limited for Buffer<P, C, Vec<C>>
where
	P: Limited,
	P: pixel::Read<C> + pixel::Write<C>,
	C: pixel::Channel,
{
	#[inline]
	fn is_valid(&self) -> bool {
		for (_, _, px) in self.pixels() {
			if !px.get().is_valid() {
				return false;
			}
		}

		true
	}

	#[inline]
	fn clamp(&self) -> Self {
		let mut output = Buffer::<P, C, _>::new(self.width(), self.height());

		for ((_, _, i), (_, _, mut o)) in self.pixels().zip(output.pixels_mut()) {
			o.set(&i.get().clamp());
		}

		output
	}

	#[inline]
	fn clamp_self(&mut self) {
		for (_, _, mut px) in self.pixels_mut() {
			let p = px.get().clamp();
			px.set(&p);
		}
	}
}

impl<P, C> ComponentWise for Buffer<P, C, Vec<C>>
where
	P: ComponentWise,
	P: pixel::Read<C> + pixel::Write<C>,
	C: pixel::Channel,
{
	type Scalar = P::Scalar;

	#[inline]
	fn component_wise<F>(&self, other: &Self, mut f: F) -> Self
	where
		F: FnMut(Self::Scalar, Self::Scalar) -> Self::Scalar,
	{
		let mut output = Buffer::<P, C, _>::new(self.width(), self.height());

		for ((x, y, i), (_, _, mut o)) in self.pixels().zip(output.pixels_mut()) {
			o.set(
				&i.get()
					.component_wise(&other.get_clamped(x as i64, y as i64), |a, b| f(a, b)),
			);
		}

		output
	}

	#[inline]
	fn component_wise_self<F>(&self, mut f: F) -> Self
	where
		F: FnMut(Self::Scalar) -> Self::Scalar,
	{
		let mut output = Buffer::<P, C, _>::new(self.width(), self.height());

		for ((_, _, i), (_, _, mut o)) in self.pixels().zip(output.pixels_mut()) {
			o.set(&i.get().component_wise_self(|x| f(x)));
		}

		output
	}
}

impl<P, C> Saturate for Buffer<P, C, Vec<C>>
where
	P: Saturate,
	P: pixel::Read<C> + pixel::Write<C>,
	C: pixel::Channel,
{
	type Scalar = P::Scalar;

	#[inline]
	fn saturate(&self, amount: Self::Scalar) -> Self {
		let mut output = Buffer::<P, C, _>::new(self.width(), self.height());

		for ((_, _, i), (_, _, mut o)) in self.pixels().zip(output.pixels_mut()) {
			o.set(&i.get().saturate(amount));
		}

		output
	}
}
