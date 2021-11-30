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

use crate::color;

pub type Luma<T = f32> = super::Buffer<color::Luma<T>, u8, Vec<u8>>;
pub type Lumaa<T = f32> = super::Buffer<color::Lumaa<T>, u8, Vec<u8>>;

pub type Rgb<T = f32> = super::Buffer<color::Rgb<T>, u8, Vec<u8>>;
pub type Rgba<T = f32> = super::Buffer<color::Rgba<T>, u8, Vec<u8>>;
