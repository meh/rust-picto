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

pub struct Nearest;

impl<CI, PI, CO, PO, T: Float> super::Scaler<CI, PI, CO, PO, T> for Nearest
	where CI: pixel::Channel,
	      PI: Pixel<CI> + pixel::Read<CI>,
	      CO: pixel::Channel,
	      PO: Pixel<CO> + pixel::Write<CO>,
	      PO: From<PI>
{
	fn scale(input: view::Ref<CI, PI>, mut output: view::Mut<CO, PO>) {
		let x_ratio = num::cast::<_, T>(input.width()).unwrap() / num::cast(output.width()).unwrap();
		let y_ratio = num::cast::<_, T>(input.height()).unwrap() / num::cast(output.height()).unwrap();

		for (x, y) in output.area().absolute() {
			output.set(x, y, &PO::from(input.get(
				num::cast::<T, _>((num::cast::<_, T>(x).unwrap() * x_ratio).floor()).unwrap(),
				num::cast::<T, _>((num::cast::<_, T>(y).unwrap() * y_ratio).floor()).unwrap())));
		}
	}
}
