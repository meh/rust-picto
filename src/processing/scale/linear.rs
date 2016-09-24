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

use num::{self, Float};
use pixel::{self, Pixel};
use view;
use color::Rgba;

pub struct Linear;

impl<CI, PI, CO, PO, T: Float> super::Scaler<CI, PI, CO, PO, T> for Linear
	where CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      PI: Into<Rgba>,
	      CO: pixel::Channel,
	      PO: Pixel<CO> + pixel::Write<CO>,
	      PO: From<Rgba>
{
	fn scale(input: view::Ref<CI, PI>, mut output: view::Mut<CO, PO>) {
		let x_ratio = num::cast::<_, T>(input.width() - 1).unwrap() / num::cast(output.width()).unwrap();
		let y_ratio = num::cast::<_, T>(input.height() - 1).unwrap() / num::cast(output.height()).unwrap();

		for (x, y) in output.area().absolute() {
			let px = num::cast::<_, u32>(x_ratio * num::cast(x).unwrap()).unwrap();
			let py = num::cast::<_, u32>(y_ratio * num::cast(y).unwrap()).unwrap();

			let x_diff = (x_ratio * num::cast(x).unwrap()) - num::cast(px).unwrap();
			let y_diff = (y_ratio * num::cast(y).unwrap()) - num::cast(py).unwrap();

			let a: Rgba = input.get(px,     py).into();
			let b: Rgba = input.get(px + 1, py).into();
			let c: Rgba = input.get(px,     py + 1).into();
			let d: Rgba = input.get(px + 1, py + 1).into();

			output.set(x, y, &Rgba::new(
				num::cast::<_, f32>(
					(num::cast::<_, T>(a.red).unwrap() * (num::one::<T>() - x_diff) * (num::one::<T>() - y_diff)) +
					(num::cast::<_, T>(b.red).unwrap() * (x_diff)                   * (num::one::<T>() - y_diff)) +
					(num::cast::<_, T>(c.red).unwrap() * (y_diff)                   * (num::one::<T>() - x_diff)) +
					(num::cast::<_, T>(d.red).unwrap() * (x_diff                    * y_diff))).unwrap(),

				num::cast::<_, f32>(
					(num::cast::<_, T>(a.green).unwrap() * (num::one::<T>() - x_diff) * (num::one::<T>() - y_diff)) +
					(num::cast::<_, T>(b.green).unwrap() * (x_diff)                  * (num::one::<T>() - y_diff)) +
					(num::cast::<_, T>(c.green).unwrap() * (y_diff)                  * (num::one::<T>() - x_diff)) +
					(num::cast::<_, T>(d.green).unwrap() * (x_diff                   * y_diff))).unwrap(),

				num::cast::<_, f32>(
					(num::cast::<_, T>(a.blue).unwrap() * (num::one::<T>() - x_diff) * (num::one::<T>() - y_diff)) +
					(num::cast::<_, T>(b.blue).unwrap() * (x_diff)                   * (num::one::<T>() - y_diff)) +
					(num::cast::<_, T>(c.blue).unwrap() * (y_diff)                   * (num::one::<T>() - x_diff)) +
					(num::cast::<_, T>(d.blue).unwrap() * (x_diff                    * y_diff))).unwrap(),

				num::cast::<_, f32>(
					(num::cast::<_, T>(a.alpha).unwrap() * (num::one::<T>() - x_diff) * (num::one::<T>() - y_diff)) +
					(num::cast::<_, T>(b.alpha).unwrap() * (x_diff)                   * (num::one::<T>() - y_diff)) +
					(num::cast::<_, T>(c.alpha).unwrap() * (y_diff)                   * (num::one::<T>() - x_diff)) +
					(num::cast::<_, T>(d.alpha).unwrap() * (x_diff                    * y_diff))).unwrap(),
			).into())
		}
	}
}
