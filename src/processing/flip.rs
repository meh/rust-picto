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
use view::View;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Orientation {
	Vertical,
	Horizontal,
}

pub trait Flip<C: pixel::Channel, P: pixel::Read<C> + pixel::Write<C>> {
	fn flip(self, mode: Orientation);
}

impl<'a, C, P, T> Flip<C, P> for T
	where C: pixel::Channel,
	      P: pixel::Read<C> + pixel::Write<C>,
	      T: Into<View<'a, C, P>>
{
	fn flip(self, mode: Orientation) {
		it(self, mode)
	}
}

pub fn it<'a, C, P, T>(value: T, mode: Orientation)
	where C: pixel::Channel,
	      P: pixel::Write<C> + pixel::Read<C>,
	      T: Into<View<'a, C, P>>
{
	let mut view   = value.into();
	let     width  = view.width();
	let     height = view.height();

	match mode {
		Orientation::Vertical => {
			if height <= 1 {
				return;
			}

			for y in 0 .. height {
				let reverse = height - y - 1;

				if y >= reverse {
					break;
				}

				for x in 0 .. width {
					let top    = view.get(x, y);
					let bottom = view.get(x, reverse);

					view.set(x, y, &bottom);
					view.set(x, reverse, &top);
				}
			}
		}

		Orientation::Horizontal => {
			if width <= 1 {
				return;
			}

			for x in 0 .. width {
				let reverse = width - x - 1;

				if x >= reverse {
					break;
				}

				for y in 0 .. height {
					let left  = view.get(x, y);
					let right = view.get(reverse, y);

					view.set(x, y, &right);
					view.set(reverse, y, &left);
				}
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use buffer::Buffer;
	use color::Rgb;

	#[test]
	fn vertical_none() {
		let mut image = Buffer::<u8, Rgb, _>::new(2, 1);
		image.set(0, 0, &Rgb::new(1.0, 1.0, 1.0));
		image.set(1, 0, &Rgb::new(1.0, 1.0, 1.0));

		image.flip(super::Orientation::Vertical);

		assert_eq!(Rgb::new(1.0, 1.0, 1.0), image.get(0, 0));
		assert_eq!(Rgb::new(1.0, 1.0, 1.0), image.get(1, 0));
	}

	#[test]
	fn vertical_even() {
		let mut image = Buffer::<u8, Rgb, _>::new(2, 2);
		image.set(0, 0, &Rgb::new(1.0, 1.0, 1.0));
		image.set(1, 0, &Rgb::new(1.0, 1.0, 1.0));
		image.set(0, 1, &Rgb::new(0.0, 0.0, 0.0));
		image.set(1, 1, &Rgb::new(0.0, 0.0, 0.0));

		image.flip(super::Orientation::Vertical);

		assert_eq!(Rgb::new(0.0, 0.0, 0.0), image.get(0, 0));
		assert_eq!(Rgb::new(0.0, 0.0, 0.0), image.get(1, 0));
		assert_eq!(Rgb::new(1.0, 1.0, 1.0), image.get(0, 1));
		assert_eq!(Rgb::new(1.0, 1.0, 1.0), image.get(1, 1));
	}

	#[test]
	fn vertical_odd() {
		let mut image = Buffer::<u8, Rgb, _>::new(2, 3);
		image.set(0, 0, &Rgb::new(1.0, 1.0, 1.0));
		image.set(1, 0, &Rgb::new(1.0, 1.0, 1.0));
		image.set(0, 1, &Rgb::new(0.0, 0.0, 0.0));
		image.set(1, 1, &Rgb::new(0.0, 0.0, 0.0));
		image.set(0, 2, &Rgb::new(0.0, 1.0, 0.0));
		image.set(1, 2, &Rgb::new(0.0, 1.0, 0.0));

		image.flip(super::Orientation::Vertical);

		assert_eq!(Rgb::new(0.0, 1.0, 0.0), image.get(0, 0));
		assert_eq!(Rgb::new(0.0, 1.0, 0.0), image.get(1, 0));
		assert_eq!(Rgb::new(0.0, 0.0, 0.0), image.get(0, 1));
		assert_eq!(Rgb::new(0.0, 0.0, 0.0), image.get(1, 1));
		assert_eq!(Rgb::new(1.0, 1.0, 1.0), image.get(0, 2));
		assert_eq!(Rgb::new(1.0, 1.0, 1.0), image.get(1, 2));
	}

	#[test]
	fn horizontal_none() {
		let mut image = Buffer::<u8, Rgb, _>::new(1, 2);
		image.set(0, 0, &Rgb::new(1.0, 1.0, 1.0));
		image.set(0, 1, &Rgb::new(1.0, 1.0, 1.0));

		image.flip(super::Orientation::Horizontal);

		assert_eq!(Rgb::new(1.0, 1.0, 1.0), image.get(0, 0));
		assert_eq!(Rgb::new(1.0, 1.0, 1.0), image.get(0, 1));
	}

	#[test]
	fn horizontal_even() {
		let mut image = Buffer::<u8, Rgb, _>::new(2, 2);
		image.set(0, 0, &Rgb::new(1.0, 1.0, 1.0));
		image.set(1, 0, &Rgb::new(0.0, 0.0, 0.0));
		image.set(0, 1, &Rgb::new(1.0, 1.0, 1.0));
		image.set(1, 1, &Rgb::new(0.0, 0.0, 0.0));

		image.flip(super::Orientation::Horizontal);

		assert_eq!(Rgb::new(0.0, 0.0, 0.0), image.get(0, 0));
		assert_eq!(Rgb::new(1.0, 1.0, 1.0), image.get(1, 0));
		assert_eq!(Rgb::new(0.0, 0.0, 0.0), image.get(0, 1));
		assert_eq!(Rgb::new(1.0, 1.0, 1.0), image.get(1, 1));
	}

	#[test]
	fn horizontal_odd() {
		let mut image = Buffer::<u8, Rgb, _>::new(3, 2);
		image.set(0, 0, &Rgb::new(1.0, 1.0, 1.0));
		image.set(1, 0, &Rgb::new(0.0, 0.0, 0.0));
		image.set(2, 0, &Rgb::new(0.0, 1.0, 0.0));
		image.set(0, 1, &Rgb::new(1.0, 1.0, 1.0));
		image.set(1, 1, &Rgb::new(0.0, 0.0, 0.0));
		image.set(2, 1, &Rgb::new(0.0, 1.0, 0.0));

		image.flip(super::Orientation::Horizontal);

		assert_eq!(Rgb::new(0.0, 1.0, 0.0), image.get(0, 0));
		assert_eq!(Rgb::new(0.0, 0.0, 0.0), image.get(1, 0));
		assert_eq!(Rgb::new(1.0, 1.0, 1.0), image.get(2, 0));
		assert_eq!(Rgb::new(0.0, 1.0, 0.0), image.get(0, 1));
		assert_eq!(Rgb::new(0.0, 0.0, 0.0), image.get(1, 1));
		assert_eq!(Rgb::new(1.0, 1.0, 1.0), image.get(2, 1));
	}
}
