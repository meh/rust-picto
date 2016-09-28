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

use pixel;
use color::{Shade, Mix, Limited, ComponentWise, Saturate};
use processing::util::GetClamped;
use super::Buffer;

impl<C, P> Shade for Buffer<C, P, Vec<C>>
	where C: pixel::Channel,
	      P: pixel::Read<C> + pixel::Write<C>,
	      P: Shade
{
	type Scalar = P::Scalar;

	fn lighten(&self, amount: Self::Scalar) -> Self {
		let mut output = Buffer::<C, P, _>::new(self.width(), self.height());

		for ((_, _, i), (_, _, mut o)) in self.pixels().zip(output.pixels_mut()) {
			o.set(&i.get().lighten(amount));
		}

		output
	}
}

impl<C, P> Mix for Buffer<C, P, Vec<C>>
	where C: pixel::Channel,
	      P: pixel::Read<C> + pixel::Write<C>,
	      P: Mix
{
	type Scalar = P::Scalar;

	fn mix(&self, other: &Self, factor: Self::Scalar) -> Self {
		let mut output = Buffer::<C, P, _>::new(self.width(), self.height());

		for ((x, y, i), (_, _, mut o)) in self.pixels().zip(output.pixels_mut()) {
			o.set(&i.get().mix(&other.get_clamped(x as i64, y as i64), factor));
		}

		output
	}
}

impl<C, P> Limited for Buffer<C, P, Vec<C>>
	where C: pixel::Channel,
	      P: pixel::Read<C> + pixel::Write<C>,
	      P: Limited
{
	fn is_valid(&self) -> bool {
		for (_, _, px) in self.pixels() {
			if !px.get().is_valid() {
				return false;
			}
		}

		true
	}

	fn clamp(&self) -> Self {
		let mut output = Buffer::<C, P, _>::new(self.width(), self.height());

		for ((_, _, i), (_, _, mut o)) in self.pixels().zip(output.pixels_mut()) {
			o.set(&i.get().clamp());
		}

		output
	}

	fn clamp_self(&mut self) {
		for (_, _, mut px) in self.pixels_mut() {
			let p = px.get().clamp();
			px.set(&p);
		}
	}
}

impl<C, P> ComponentWise for Buffer<C, P, Vec<C>>
	where C: pixel::Channel,
	      P: pixel::Read<C> + pixel::Write<C>,
	      P: ComponentWise
{
	type Scalar = P::Scalar;

	fn component_wise<F>(&self, other: &Self, mut f: F) -> Self
		where F: FnMut(Self::Scalar, Self::Scalar) -> Self::Scalar
	{
		let mut output = Buffer::<C, P, _>::new(self.width(), self.height());

		for ((x, y, i), (_, _, mut o)) in self.pixels().zip(output.pixels_mut()) {
			o.set(&i.get().component_wise(&other.get_clamped(x as i64, y as i64), |a, b| f(a, b)));
		}

		output
	}

	fn component_wise_self<F>(&self, mut f: F) -> Self
		where F: FnMut(Self::Scalar) -> Self::Scalar
	{
		let mut output = Buffer::<C, P, _>::new(self.width(), self.height());

		for ((_, _, i), (_, _, mut o)) in self.pixels().zip(output.pixels_mut()) {
			o.set(&i.get().component_wise_self(|x| f(x)));
		}

		output
	}
}

impl<C, P> Saturate for Buffer<C, P, Vec<C>>
	where C: pixel::Channel,
	      P: pixel::Read<C> + pixel::Write<C>,
	      P: Saturate
{
	type Scalar = P::Scalar;

	fn saturate(&self, amount: Self::Scalar) -> Self {
		let mut output = Buffer::<C, P, _>::new(self.width(), self.height());

		for ((_, _, i), (_, _, mut o)) in self.pixels().zip(output.pixels_mut()) {
			o.set(&i.get().saturate(amount));
		}

		output
	}
}
