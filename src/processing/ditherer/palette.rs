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

use std::marker::PhantomData;

pub use exoquant::Color;

use crate::{
	buffer::{cast, Buffer},
	color::Rgba,
	pixel, view,
};

/// A palette table.
pub trait Table {
	fn table() -> &'static [Color];
}

/// A palette to color mapper.
pub trait Mapper {
	fn indices(table: &[Color], input: &Buffer<Rgba, u8, Vec<u8>>) -> Vec<usize>;
}

/// A palette table and mapper pair.
pub struct Palette<T: Table, M: Mapper = mapper::FloydSteinberg>(PhantomData<T>, PhantomData<M>);

impl<T, M, PI, CI, PO, CO> super::Ditherer<PI, CI, PO, CO> for Palette<T, M>
where
	T: Table,
	M: Mapper,
	PI: Into<Rgba>,
	PI: pixel::Read<CI>,
	CI: pixel::Channel,
	PO: From<Rgba> + Into<Rgba> + From<PI>,
	PO: pixel::Write<CO> + pixel::Read<CO>,
	CO: pixel::Channel,
{
	#[inline]
	fn dither(input: &view::Read<PI, CI>, _colors: u32) -> Buffer<PO, CO, Vec<CO>> {
		let mut buffer = input.convert::<Rgba, u8>();
		let indices = M::indices(T::table(), &buffer);

		for (output, index) in buffer.chunks_mut(4).zip(indices.into_iter()) {
			let color = T::table()[index];

			output[0] = color.r;
			output[1] = color.g;
			output[2] = color.b;
			output[3] = color.a;
		}

		cast::Into::<PO, CO>::into(buffer)
	}
}

pub mod mapper {
	use std::slice;

	use exoquant::{ditherer, Color, Remapper, SimpleColorSpace};

	use crate::{buffer::Buffer, color::Rgba};

	pub struct None;
	pub struct Ordered;
	pub struct FloydSteinberg;

	impl super::Mapper for None {
		fn indices(table: &[Color], input: &Buffer<Rgba, u8, Vec<u8>>) -> Vec<usize> {
			Remapper::new(table, &SimpleColorSpace::default(), &ditherer::None).remap_usize(
				unsafe { slice::from_raw_parts(input.as_ptr() as *const _, input.len() / 4) },
				input.width() as usize,
			)
		}
	}

	impl super::Mapper for Ordered {
		fn indices(table: &[Color], input: &Buffer<Rgba, u8, Vec<u8>>) -> Vec<usize> {
			Remapper::new(table, &SimpleColorSpace::default(), &ditherer::Ordered).remap_usize(
				unsafe { slice::from_raw_parts(input.as_ptr() as *const _, input.len() / 4) },
				input.width() as usize,
			)
		}
	}

	impl super::Mapper for FloydSteinberg {
		fn indices(table: &[Color], input: &Buffer<Rgba, u8, Vec<u8>>) -> Vec<usize> {
			Remapper::new(table, &SimpleColorSpace::default(), &ditherer::FloydSteinberg::new()).remap_usize(
				unsafe { slice::from_raw_parts(input.as_ptr() as *const _, input.len() / 4) },
				input.width() as usize,
			)
		}
	}

	pub mod floyd_steinberg {
		use std::slice;

		use exoquant::{ditherer, Color, Remapper, SimpleColorSpace};

		use crate::{buffer::Buffer, color::Rgba};

		pub struct Vanilla;
		pub struct Checkered;

		impl super::super::Mapper for Vanilla {
			fn indices(table: &[Color], input: &Buffer<Rgba, u8, Vec<u8>>) -> Vec<usize> {
				Remapper::new(
					table,
					&SimpleColorSpace::default(),
					&ditherer::FloydSteinberg::vanilla(),
				)
				.remap_usize(
					unsafe { slice::from_raw_parts(input.as_ptr() as *const _, input.len() / 4) },
					input.width() as usize,
				)
			}
		}

		impl super::super::Mapper for Checkered {
			fn indices(table: &[Color], input: &Buffer<Rgba, u8, Vec<u8>>) -> Vec<usize> {
				Remapper::new(
					table,
					&SimpleColorSpace::default(),
					&ditherer::FloydSteinberg::checkered(),
				)
				.remap_usize(
					unsafe { slice::from_raw_parts(input.as_ptr() as *const _, input.len() / 4) },
					input.width() as usize,
				)
			}
		}
	}
}

pub mod table {
	use exoquant::Color;

	pub struct MonoDark;
	pub struct MonoLight;

	impl super::Table for MonoDark {
		fn table() -> &'static [Color] {
			static TABLE: &'static [Color] = &[
				Color {
					r: 0x00,
					g: 0x00,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xff,
					b: 0xff,
					a: 0xff,
				},
			];

			TABLE
		}
	}

	impl super::Table for MonoLight {
		fn table() -> &'static [Color] {
			static TABLE: &'static [Color] = &[
				Color {
					r: 0xff,
					g: 0xff,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x00,
					b: 0x00,
					a: 0xff,
				},
			];

			TABLE
		}
	}

	pub struct Gray1;
	pub struct Gray2;
	pub struct Gray4;
	pub struct Gray8;

	impl super::Table for Gray1 {
		fn table() -> &'static [Color] {
			static TABLE: &'static [Color] = &[
				Color {
					r: 0x00,
					g: 0x00,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xff,
					b: 0xff,
					a: 0xff,
				},
			];

			TABLE
		}
	}

	impl super::Table for Gray2 {
		fn table() -> &'static [Color] {
			static TABLE: &'static [Color] = &[
				Color {
					r: 0x00,
					g: 0x00,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x55,
					g: 0x55,
					b: 0x55,
					a: 0xff,
				},
				Color {
					r: 0xaa,
					g: 0xaa,
					b: 0xaa,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xff,
					b: 0xff,
					a: 0xff,
				},
			];

			TABLE
		}
	}

	impl super::Table for Gray4 {
		fn table() -> &'static [Color] {
			static TABLE: &'static [Color] = &[
				Color {
					r: 0x00,
					g: 0x00,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x11,
					g: 0x11,
					b: 0x11,
					a: 0xff,
				},
				Color {
					r: 0x22,
					g: 0x22,
					b: 0x22,
					a: 0xff,
				},
				Color {
					r: 0x33,
					g: 0x33,
					b: 0x33,
					a: 0xff,
				},
				Color {
					r: 0x44,
					g: 0x44,
					b: 0x44,
					a: 0xff,
				},
				Color {
					r: 0x55,
					g: 0x55,
					b: 0x55,
					a: 0xff,
				},
				Color {
					r: 0x66,
					g: 0x66,
					b: 0x66,
					a: 0xff,
				},
				Color {
					r: 0x77,
					g: 0x77,
					b: 0x77,
					a: 0xff,
				},
				Color {
					r: 0x88,
					g: 0x88,
					b: 0x88,
					a: 0xff,
				},
				Color {
					r: 0x99,
					g: 0x99,
					b: 0x99,
					a: 0xff,
				},
				Color {
					r: 0xaa,
					g: 0xaa,
					b: 0xaa,
					a: 0xff,
				},
				Color {
					r: 0xbb,
					g: 0xbb,
					b: 0xbb,
					a: 0xff,
				},
				Color {
					r: 0xcc,
					g: 0xcc,
					b: 0xcc,
					a: 0xff,
				},
				Color {
					r: 0xdd,
					g: 0xdd,
					b: 0xdd,
					a: 0xff,
				},
				Color {
					r: 0xee,
					g: 0xee,
					b: 0xee,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xff,
					b: 0xff,
					a: 0xff,
				},
			];

			TABLE
		}
	}

	impl super::Table for Gray8 {
		fn table() -> &'static [Color] {
			static TABLE: &'static [Color] = &[
				Color {
					r: 0x00,
					g: 0x00,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x01,
					g: 0x01,
					b: 0x01,
					a: 0xff,
				},
				Color {
					r: 0x02,
					g: 0x02,
					b: 0x02,
					a: 0xff,
				},
				Color {
					r: 0x03,
					g: 0x03,
					b: 0x03,
					a: 0xff,
				},
				Color {
					r: 0x04,
					g: 0x04,
					b: 0x04,
					a: 0xff,
				},
				Color {
					r: 0x05,
					g: 0x05,
					b: 0x05,
					a: 0xff,
				},
				Color {
					r: 0x06,
					g: 0x06,
					b: 0x06,
					a: 0xff,
				},
				Color {
					r: 0x07,
					g: 0x07,
					b: 0x07,
					a: 0xff,
				},
				Color {
					r: 0x08,
					g: 0x08,
					b: 0x08,
					a: 0xff,
				},
				Color {
					r: 0x09,
					g: 0x09,
					b: 0x09,
					a: 0xff,
				},
				Color {
					r: 0x0a,
					g: 0x0a,
					b: 0x0a,
					a: 0xff,
				},
				Color {
					r: 0x0b,
					g: 0x0b,
					b: 0x0b,
					a: 0xff,
				},
				Color {
					r: 0x0c,
					g: 0x0c,
					b: 0x0c,
					a: 0xff,
				},
				Color {
					r: 0x0d,
					g: 0x0d,
					b: 0x0d,
					a: 0xff,
				},
				Color {
					r: 0x0e,
					g: 0x0e,
					b: 0x0e,
					a: 0xff,
				},
				Color {
					r: 0x0f,
					g: 0x0f,
					b: 0x0f,
					a: 0xff,
				},
				Color {
					r: 0x10,
					g: 0x10,
					b: 0x10,
					a: 0xff,
				},
				Color {
					r: 0x11,
					g: 0x11,
					b: 0x11,
					a: 0xff,
				},
				Color {
					r: 0x12,
					g: 0x12,
					b: 0x12,
					a: 0xff,
				},
				Color {
					r: 0x13,
					g: 0x13,
					b: 0x13,
					a: 0xff,
				},
				Color {
					r: 0x14,
					g: 0x14,
					b: 0x14,
					a: 0xff,
				},
				Color {
					r: 0x15,
					g: 0x15,
					b: 0x15,
					a: 0xff,
				},
				Color {
					r: 0x16,
					g: 0x16,
					b: 0x16,
					a: 0xff,
				},
				Color {
					r: 0x17,
					g: 0x17,
					b: 0x17,
					a: 0xff,
				},
				Color {
					r: 0x18,
					g: 0x18,
					b: 0x18,
					a: 0xff,
				},
				Color {
					r: 0x19,
					g: 0x19,
					b: 0x19,
					a: 0xff,
				},
				Color {
					r: 0x1a,
					g: 0x1a,
					b: 0x1a,
					a: 0xff,
				},
				Color {
					r: 0x1b,
					g: 0x1b,
					b: 0x1b,
					a: 0xff,
				},
				Color {
					r: 0x1c,
					g: 0x1c,
					b: 0x1c,
					a: 0xff,
				},
				Color {
					r: 0x1d,
					g: 0x1d,
					b: 0x1d,
					a: 0xff,
				},
				Color {
					r: 0x1e,
					g: 0x1e,
					b: 0x1e,
					a: 0xff,
				},
				Color {
					r: 0x1f,
					g: 0x1f,
					b: 0x1f,
					a: 0xff,
				},
				Color {
					r: 0x20,
					g: 0x20,
					b: 0x20,
					a: 0xff,
				},
				Color {
					r: 0x21,
					g: 0x21,
					b: 0x21,
					a: 0xff,
				},
				Color {
					r: 0x22,
					g: 0x22,
					b: 0x22,
					a: 0xff,
				},
				Color {
					r: 0x23,
					g: 0x23,
					b: 0x23,
					a: 0xff,
				},
				Color {
					r: 0x24,
					g: 0x24,
					b: 0x24,
					a: 0xff,
				},
				Color {
					r: 0x25,
					g: 0x25,
					b: 0x25,
					a: 0xff,
				},
				Color {
					r: 0x26,
					g: 0x26,
					b: 0x26,
					a: 0xff,
				},
				Color {
					r: 0x27,
					g: 0x27,
					b: 0x27,
					a: 0xff,
				},
				Color {
					r: 0x28,
					g: 0x28,
					b: 0x28,
					a: 0xff,
				},
				Color {
					r: 0x29,
					g: 0x29,
					b: 0x29,
					a: 0xff,
				},
				Color {
					r: 0x2a,
					g: 0x2a,
					b: 0x2a,
					a: 0xff,
				},
				Color {
					r: 0x2b,
					g: 0x2b,
					b: 0x2b,
					a: 0xff,
				},
				Color {
					r: 0x2c,
					g: 0x2c,
					b: 0x2c,
					a: 0xff,
				},
				Color {
					r: 0x2d,
					g: 0x2d,
					b: 0x2d,
					a: 0xff,
				},
				Color {
					r: 0x2e,
					g: 0x2e,
					b: 0x2e,
					a: 0xff,
				},
				Color {
					r: 0x2f,
					g: 0x2f,
					b: 0x2f,
					a: 0xff,
				},
				Color {
					r: 0x30,
					g: 0x30,
					b: 0x30,
					a: 0xff,
				},
				Color {
					r: 0x31,
					g: 0x31,
					b: 0x31,
					a: 0xff,
				},
				Color {
					r: 0x32,
					g: 0x32,
					b: 0x32,
					a: 0xff,
				},
				Color {
					r: 0x33,
					g: 0x33,
					b: 0x33,
					a: 0xff,
				},
				Color {
					r: 0x34,
					g: 0x34,
					b: 0x34,
					a: 0xff,
				},
				Color {
					r: 0x35,
					g: 0x35,
					b: 0x35,
					a: 0xff,
				},
				Color {
					r: 0x36,
					g: 0x36,
					b: 0x36,
					a: 0xff,
				},
				Color {
					r: 0x37,
					g: 0x37,
					b: 0x37,
					a: 0xff,
				},
				Color {
					r: 0x38,
					g: 0x38,
					b: 0x38,
					a: 0xff,
				},
				Color {
					r: 0x39,
					g: 0x39,
					b: 0x39,
					a: 0xff,
				},
				Color {
					r: 0x3a,
					g: 0x3a,
					b: 0x3a,
					a: 0xff,
				},
				Color {
					r: 0x3b,
					g: 0x3b,
					b: 0x3b,
					a: 0xff,
				},
				Color {
					r: 0x3c,
					g: 0x3c,
					b: 0x3c,
					a: 0xff,
				},
				Color {
					r: 0x3d,
					g: 0x3d,
					b: 0x3d,
					a: 0xff,
				},
				Color {
					r: 0x3e,
					g: 0x3e,
					b: 0x3e,
					a: 0xff,
				},
				Color {
					r: 0x3f,
					g: 0x3f,
					b: 0x3f,
					a: 0xff,
				},
				Color {
					r: 0x40,
					g: 0x40,
					b: 0x40,
					a: 0xff,
				},
				Color {
					r: 0x41,
					g: 0x41,
					b: 0x41,
					a: 0xff,
				},
				Color {
					r: 0x42,
					g: 0x42,
					b: 0x42,
					a: 0xff,
				},
				Color {
					r: 0x43,
					g: 0x43,
					b: 0x43,
					a: 0xff,
				},
				Color {
					r: 0x44,
					g: 0x44,
					b: 0x44,
					a: 0xff,
				},
				Color {
					r: 0x45,
					g: 0x45,
					b: 0x45,
					a: 0xff,
				},
				Color {
					r: 0x46,
					g: 0x46,
					b: 0x46,
					a: 0xff,
				},
				Color {
					r: 0x47,
					g: 0x47,
					b: 0x47,
					a: 0xff,
				},
				Color {
					r: 0x48,
					g: 0x48,
					b: 0x48,
					a: 0xff,
				},
				Color {
					r: 0x49,
					g: 0x49,
					b: 0x49,
					a: 0xff,
				},
				Color {
					r: 0x4a,
					g: 0x4a,
					b: 0x4a,
					a: 0xff,
				},
				Color {
					r: 0x4b,
					g: 0x4b,
					b: 0x4b,
					a: 0xff,
				},
				Color {
					r: 0x4c,
					g: 0x4c,
					b: 0x4c,
					a: 0xff,
				},
				Color {
					r: 0x4d,
					g: 0x4d,
					b: 0x4d,
					a: 0xff,
				},
				Color {
					r: 0x4e,
					g: 0x4e,
					b: 0x4e,
					a: 0xff,
				},
				Color {
					r: 0x4f,
					g: 0x4f,
					b: 0x4f,
					a: 0xff,
				},
				Color {
					r: 0x50,
					g: 0x50,
					b: 0x50,
					a: 0xff,
				},
				Color {
					r: 0x51,
					g: 0x51,
					b: 0x51,
					a: 0xff,
				},
				Color {
					r: 0x52,
					g: 0x52,
					b: 0x52,
					a: 0xff,
				},
				Color {
					r: 0x53,
					g: 0x53,
					b: 0x53,
					a: 0xff,
				},
				Color {
					r: 0x54,
					g: 0x54,
					b: 0x54,
					a: 0xff,
				},
				Color {
					r: 0x55,
					g: 0x55,
					b: 0x55,
					a: 0xff,
				},
				Color {
					r: 0x56,
					g: 0x56,
					b: 0x56,
					a: 0xff,
				},
				Color {
					r: 0x57,
					g: 0x57,
					b: 0x57,
					a: 0xff,
				},
				Color {
					r: 0x58,
					g: 0x58,
					b: 0x58,
					a: 0xff,
				},
				Color {
					r: 0x59,
					g: 0x59,
					b: 0x59,
					a: 0xff,
				},
				Color {
					r: 0x5a,
					g: 0x5a,
					b: 0x5a,
					a: 0xff,
				},
				Color {
					r: 0x5b,
					g: 0x5b,
					b: 0x5b,
					a: 0xff,
				},
				Color {
					r: 0x5c,
					g: 0x5c,
					b: 0x5c,
					a: 0xff,
				},
				Color {
					r: 0x5d,
					g: 0x5d,
					b: 0x5d,
					a: 0xff,
				},
				Color {
					r: 0x5e,
					g: 0x5e,
					b: 0x5e,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x5f,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x60,
					g: 0x60,
					b: 0x60,
					a: 0xff,
				},
				Color {
					r: 0x61,
					g: 0x61,
					b: 0x61,
					a: 0xff,
				},
				Color {
					r: 0x62,
					g: 0x62,
					b: 0x62,
					a: 0xff,
				},
				Color {
					r: 0x63,
					g: 0x63,
					b: 0x63,
					a: 0xff,
				},
				Color {
					r: 0x64,
					g: 0x64,
					b: 0x64,
					a: 0xff,
				},
				Color {
					r: 0x65,
					g: 0x65,
					b: 0x65,
					a: 0xff,
				},
				Color {
					r: 0x66,
					g: 0x66,
					b: 0x66,
					a: 0xff,
				},
				Color {
					r: 0x67,
					g: 0x67,
					b: 0x67,
					a: 0xff,
				},
				Color {
					r: 0x68,
					g: 0x68,
					b: 0x68,
					a: 0xff,
				},
				Color {
					r: 0x69,
					g: 0x69,
					b: 0x69,
					a: 0xff,
				},
				Color {
					r: 0x6a,
					g: 0x6a,
					b: 0x6a,
					a: 0xff,
				},
				Color {
					r: 0x6b,
					g: 0x6b,
					b: 0x6b,
					a: 0xff,
				},
				Color {
					r: 0x6c,
					g: 0x6c,
					b: 0x6c,
					a: 0xff,
				},
				Color {
					r: 0x6d,
					g: 0x6d,
					b: 0x6d,
					a: 0xff,
				},
				Color {
					r: 0x6e,
					g: 0x6e,
					b: 0x6e,
					a: 0xff,
				},
				Color {
					r: 0x6f,
					g: 0x6f,
					b: 0x6f,
					a: 0xff,
				},
				Color {
					r: 0x70,
					g: 0x70,
					b: 0x70,
					a: 0xff,
				},
				Color {
					r: 0x71,
					g: 0x71,
					b: 0x71,
					a: 0xff,
				},
				Color {
					r: 0x72,
					g: 0x72,
					b: 0x72,
					a: 0xff,
				},
				Color {
					r: 0x73,
					g: 0x73,
					b: 0x73,
					a: 0xff,
				},
				Color {
					r: 0x74,
					g: 0x74,
					b: 0x74,
					a: 0xff,
				},
				Color {
					r: 0x75,
					g: 0x75,
					b: 0x75,
					a: 0xff,
				},
				Color {
					r: 0x76,
					g: 0x76,
					b: 0x76,
					a: 0xff,
				},
				Color {
					r: 0x77,
					g: 0x77,
					b: 0x77,
					a: 0xff,
				},
				Color {
					r: 0x78,
					g: 0x78,
					b: 0x78,
					a: 0xff,
				},
				Color {
					r: 0x79,
					g: 0x79,
					b: 0x79,
					a: 0xff,
				},
				Color {
					r: 0x7a,
					g: 0x7a,
					b: 0x7a,
					a: 0xff,
				},
				Color {
					r: 0x7b,
					g: 0x7b,
					b: 0x7b,
					a: 0xff,
				},
				Color {
					r: 0x7c,
					g: 0x7c,
					b: 0x7c,
					a: 0xff,
				},
				Color {
					r: 0x7d,
					g: 0x7d,
					b: 0x7d,
					a: 0xff,
				},
				Color {
					r: 0x7e,
					g: 0x7e,
					b: 0x7e,
					a: 0xff,
				},
				Color {
					r: 0x7f,
					g: 0x7f,
					b: 0x7f,
					a: 0xff,
				},
				Color {
					r: 0x80,
					g: 0x80,
					b: 0x80,
					a: 0xff,
				},
				Color {
					r: 0x81,
					g: 0x81,
					b: 0x81,
					a: 0xff,
				},
				Color {
					r: 0x82,
					g: 0x82,
					b: 0x82,
					a: 0xff,
				},
				Color {
					r: 0x83,
					g: 0x83,
					b: 0x83,
					a: 0xff,
				},
				Color {
					r: 0x84,
					g: 0x84,
					b: 0x84,
					a: 0xff,
				},
				Color {
					r: 0x85,
					g: 0x85,
					b: 0x85,
					a: 0xff,
				},
				Color {
					r: 0x86,
					g: 0x86,
					b: 0x86,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x87,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x88,
					g: 0x88,
					b: 0x88,
					a: 0xff,
				},
				Color {
					r: 0x89,
					g: 0x89,
					b: 0x89,
					a: 0xff,
				},
				Color {
					r: 0x8a,
					g: 0x8a,
					b: 0x8a,
					a: 0xff,
				},
				Color {
					r: 0x8b,
					g: 0x8b,
					b: 0x8b,
					a: 0xff,
				},
				Color {
					r: 0x8c,
					g: 0x8c,
					b: 0x8c,
					a: 0xff,
				},
				Color {
					r: 0x8d,
					g: 0x8d,
					b: 0x8d,
					a: 0xff,
				},
				Color {
					r: 0x8e,
					g: 0x8e,
					b: 0x8e,
					a: 0xff,
				},
				Color {
					r: 0x8f,
					g: 0x8f,
					b: 0x8f,
					a: 0xff,
				},
				Color {
					r: 0x90,
					g: 0x90,
					b: 0x90,
					a: 0xff,
				},
				Color {
					r: 0x91,
					g: 0x91,
					b: 0x91,
					a: 0xff,
				},
				Color {
					r: 0x92,
					g: 0x92,
					b: 0x92,
					a: 0xff,
				},
				Color {
					r: 0x93,
					g: 0x93,
					b: 0x93,
					a: 0xff,
				},
				Color {
					r: 0x94,
					g: 0x94,
					b: 0x94,
					a: 0xff,
				},
				Color {
					r: 0x95,
					g: 0x95,
					b: 0x95,
					a: 0xff,
				},
				Color {
					r: 0x96,
					g: 0x96,
					b: 0x96,
					a: 0xff,
				},
				Color {
					r: 0x97,
					g: 0x97,
					b: 0x97,
					a: 0xff,
				},
				Color {
					r: 0x98,
					g: 0x98,
					b: 0x98,
					a: 0xff,
				},
				Color {
					r: 0x99,
					g: 0x99,
					b: 0x99,
					a: 0xff,
				},
				Color {
					r: 0x9a,
					g: 0x9a,
					b: 0x9a,
					a: 0xff,
				},
				Color {
					r: 0x9b,
					g: 0x9b,
					b: 0x9b,
					a: 0xff,
				},
				Color {
					r: 0x9c,
					g: 0x9c,
					b: 0x9c,
					a: 0xff,
				},
				Color {
					r: 0x9d,
					g: 0x9d,
					b: 0x9d,
					a: 0xff,
				},
				Color {
					r: 0x9e,
					g: 0x9e,
					b: 0x9e,
					a: 0xff,
				},
				Color {
					r: 0x9f,
					g: 0x9f,
					b: 0x9f,
					a: 0xff,
				},
				Color {
					r: 0xa0,
					g: 0xa0,
					b: 0xa0,
					a: 0xff,
				},
				Color {
					r: 0xa1,
					g: 0xa1,
					b: 0xa1,
					a: 0xff,
				},
				Color {
					r: 0xa2,
					g: 0xa2,
					b: 0xa2,
					a: 0xff,
				},
				Color {
					r: 0xa3,
					g: 0xa3,
					b: 0xa3,
					a: 0xff,
				},
				Color {
					r: 0xa4,
					g: 0xa4,
					b: 0xa4,
					a: 0xff,
				},
				Color {
					r: 0xa5,
					g: 0xa5,
					b: 0xa5,
					a: 0xff,
				},
				Color {
					r: 0xa6,
					g: 0xa6,
					b: 0xa6,
					a: 0xff,
				},
				Color {
					r: 0xa7,
					g: 0xa7,
					b: 0xa7,
					a: 0xff,
				},
				Color {
					r: 0xa8,
					g: 0xa8,
					b: 0xa8,
					a: 0xff,
				},
				Color {
					r: 0xa9,
					g: 0xa9,
					b: 0xa9,
					a: 0xff,
				},
				Color {
					r: 0xaa,
					g: 0xaa,
					b: 0xaa,
					a: 0xff,
				},
				Color {
					r: 0xab,
					g: 0xab,
					b: 0xab,
					a: 0xff,
				},
				Color {
					r: 0xac,
					g: 0xac,
					b: 0xac,
					a: 0xff,
				},
				Color {
					r: 0xad,
					g: 0xad,
					b: 0xad,
					a: 0xff,
				},
				Color {
					r: 0xae,
					g: 0xae,
					b: 0xae,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xaf,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xb0,
					g: 0xb0,
					b: 0xb0,
					a: 0xff,
				},
				Color {
					r: 0xb1,
					g: 0xb1,
					b: 0xb1,
					a: 0xff,
				},
				Color {
					r: 0xb2,
					g: 0xb2,
					b: 0xb2,
					a: 0xff,
				},
				Color {
					r: 0xb3,
					g: 0xb3,
					b: 0xb3,
					a: 0xff,
				},
				Color {
					r: 0xb4,
					g: 0xb4,
					b: 0xb4,
					a: 0xff,
				},
				Color {
					r: 0xb5,
					g: 0xb5,
					b: 0xb5,
					a: 0xff,
				},
				Color {
					r: 0xb6,
					g: 0xb6,
					b: 0xb6,
					a: 0xff,
				},
				Color {
					r: 0xb7,
					g: 0xb7,
					b: 0xb7,
					a: 0xff,
				},
				Color {
					r: 0xb8,
					g: 0xb8,
					b: 0xb8,
					a: 0xff,
				},
				Color {
					r: 0xb9,
					g: 0xb9,
					b: 0xb9,
					a: 0xff,
				},
				Color {
					r: 0xba,
					g: 0xba,
					b: 0xba,
					a: 0xff,
				},
				Color {
					r: 0xbb,
					g: 0xbb,
					b: 0xbb,
					a: 0xff,
				},
				Color {
					r: 0xbc,
					g: 0xbc,
					b: 0xbc,
					a: 0xff,
				},
				Color {
					r: 0xbd,
					g: 0xbd,
					b: 0xbd,
					a: 0xff,
				},
				Color {
					r: 0xbe,
					g: 0xbe,
					b: 0xbe,
					a: 0xff,
				},
				Color {
					r: 0xbf,
					g: 0xbf,
					b: 0xbf,
					a: 0xff,
				},
				Color {
					r: 0xc0,
					g: 0xc0,
					b: 0xc0,
					a: 0xff,
				},
				Color {
					r: 0xc1,
					g: 0xc1,
					b: 0xc1,
					a: 0xff,
				},
				Color {
					r: 0xc2,
					g: 0xc2,
					b: 0xc2,
					a: 0xff,
				},
				Color {
					r: 0xc3,
					g: 0xc3,
					b: 0xc3,
					a: 0xff,
				},
				Color {
					r: 0xc4,
					g: 0xc4,
					b: 0xc4,
					a: 0xff,
				},
				Color {
					r: 0xc5,
					g: 0xc5,
					b: 0xc5,
					a: 0xff,
				},
				Color {
					r: 0xc6,
					g: 0xc6,
					b: 0xc6,
					a: 0xff,
				},
				Color {
					r: 0xc7,
					g: 0xc7,
					b: 0xc7,
					a: 0xff,
				},
				Color {
					r: 0xc8,
					g: 0xc8,
					b: 0xc8,
					a: 0xff,
				},
				Color {
					r: 0xc9,
					g: 0xc9,
					b: 0xc9,
					a: 0xff,
				},
				Color {
					r: 0xca,
					g: 0xca,
					b: 0xca,
					a: 0xff,
				},
				Color {
					r: 0xcb,
					g: 0xcb,
					b: 0xcb,
					a: 0xff,
				},
				Color {
					r: 0xcc,
					g: 0xcc,
					b: 0xcc,
					a: 0xff,
				},
				Color {
					r: 0xcd,
					g: 0xcd,
					b: 0xcd,
					a: 0xff,
				},
				Color {
					r: 0xce,
					g: 0xce,
					b: 0xce,
					a: 0xff,
				},
				Color {
					r: 0xcf,
					g: 0xcf,
					b: 0xcf,
					a: 0xff,
				},
				Color {
					r: 0xd0,
					g: 0xd0,
					b: 0xd0,
					a: 0xff,
				},
				Color {
					r: 0xd1,
					g: 0xd1,
					b: 0xd1,
					a: 0xff,
				},
				Color {
					r: 0xd2,
					g: 0xd2,
					b: 0xd2,
					a: 0xff,
				},
				Color {
					r: 0xd3,
					g: 0xd3,
					b: 0xd3,
					a: 0xff,
				},
				Color {
					r: 0xd4,
					g: 0xd4,
					b: 0xd4,
					a: 0xff,
				},
				Color {
					r: 0xd5,
					g: 0xd5,
					b: 0xd5,
					a: 0xff,
				},
				Color {
					r: 0xd6,
					g: 0xd6,
					b: 0xd6,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xd7,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xd8,
					g: 0xd8,
					b: 0xd8,
					a: 0xff,
				},
				Color {
					r: 0xd9,
					g: 0xd9,
					b: 0xd9,
					a: 0xff,
				},
				Color {
					r: 0xda,
					g: 0xda,
					b: 0xda,
					a: 0xff,
				},
				Color {
					r: 0xdb,
					g: 0xdb,
					b: 0xdb,
					a: 0xff,
				},
				Color {
					r: 0xdc,
					g: 0xdc,
					b: 0xdc,
					a: 0xff,
				},
				Color {
					r: 0xdd,
					g: 0xdd,
					b: 0xdd,
					a: 0xff,
				},
				Color {
					r: 0xde,
					g: 0xde,
					b: 0xde,
					a: 0xff,
				},
				Color {
					r: 0xdf,
					g: 0xdf,
					b: 0xdf,
					a: 0xff,
				},
				Color {
					r: 0xe0,
					g: 0xe0,
					b: 0xe0,
					a: 0xff,
				},
				Color {
					r: 0xe1,
					g: 0xe1,
					b: 0xe1,
					a: 0xff,
				},
				Color {
					r: 0xe2,
					g: 0xe2,
					b: 0xe2,
					a: 0xff,
				},
				Color {
					r: 0xe3,
					g: 0xe3,
					b: 0xe3,
					a: 0xff,
				},
				Color {
					r: 0xe4,
					g: 0xe4,
					b: 0xe4,
					a: 0xff,
				},
				Color {
					r: 0xe5,
					g: 0xe5,
					b: 0xe5,
					a: 0xff,
				},
				Color {
					r: 0xe6,
					g: 0xe6,
					b: 0xe6,
					a: 0xff,
				},
				Color {
					r: 0xe7,
					g: 0xe7,
					b: 0xe7,
					a: 0xff,
				},
				Color {
					r: 0xe8,
					g: 0xe8,
					b: 0xe8,
					a: 0xff,
				},
				Color {
					r: 0xe9,
					g: 0xe9,
					b: 0xe9,
					a: 0xff,
				},
				Color {
					r: 0xea,
					g: 0xea,
					b: 0xea,
					a: 0xff,
				},
				Color {
					r: 0xeb,
					g: 0xeb,
					b: 0xeb,
					a: 0xff,
				},
				Color {
					r: 0xec,
					g: 0xec,
					b: 0xec,
					a: 0xff,
				},
				Color {
					r: 0xed,
					g: 0xed,
					b: 0xed,
					a: 0xff,
				},
				Color {
					r: 0xee,
					g: 0xee,
					b: 0xee,
					a: 0xff,
				},
				Color {
					r: 0xef,
					g: 0xef,
					b: 0xef,
					a: 0xff,
				},
				Color {
					r: 0xf0,
					g: 0xf0,
					b: 0xf0,
					a: 0xff,
				},
				Color {
					r: 0xf1,
					g: 0xf1,
					b: 0xf1,
					a: 0xff,
				},
				Color {
					r: 0xf2,
					g: 0xf2,
					b: 0xf2,
					a: 0xff,
				},
				Color {
					r: 0xf3,
					g: 0xf3,
					b: 0xf3,
					a: 0xff,
				},
				Color {
					r: 0xf4,
					g: 0xf4,
					b: 0xf4,
					a: 0xff,
				},
				Color {
					r: 0xf5,
					g: 0xf5,
					b: 0xf5,
					a: 0xff,
				},
				Color {
					r: 0xf6,
					g: 0xf6,
					b: 0xf6,
					a: 0xff,
				},
				Color {
					r: 0xf7,
					g: 0xf7,
					b: 0xf7,
					a: 0xff,
				},
				Color {
					r: 0xf8,
					g: 0xf8,
					b: 0xf8,
					a: 0xff,
				},
				Color {
					r: 0xf9,
					g: 0xf9,
					b: 0xf9,
					a: 0xff,
				},
				Color {
					r: 0xfa,
					g: 0xfa,
					b: 0xfa,
					a: 0xff,
				},
				Color {
					r: 0xfb,
					g: 0xfb,
					b: 0xfb,
					a: 0xff,
				},
				Color {
					r: 0xfc,
					g: 0xfc,
					b: 0xfc,
					a: 0xff,
				},
				Color {
					r: 0xfd,
					g: 0xfd,
					b: 0xfd,
					a: 0xff,
				},
				Color {
					r: 0xfe,
					g: 0xfe,
					b: 0xfe,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xff,
					b: 0xff,
					a: 0xff,
				},
			];

			TABLE
		}
	}

	pub struct XTerm;
	pub struct VT340;

	impl super::Table for XTerm {
		fn table() -> &'static [Color] {
			static TABLE: &'static [Color] = &[
				Color {
					r: 0x00,
					g: 0x00,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x80,
					g: 0x00,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x80,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x80,
					g: 0x80,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x00,
					b: 0x80,
					a: 0xff,
				},
				Color {
					r: 0x80,
					g: 0x00,
					b: 0x80,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x80,
					b: 0x80,
					a: 0xff,
				},
				Color {
					r: 0xc0,
					g: 0xc0,
					b: 0xc0,
					a: 0xff,
				},
				Color {
					r: 0x80,
					g: 0x80,
					b: 0x80,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x00,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xff,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xff,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x00,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x00,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xff,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xff,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x00,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x00,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x00,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x00,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x00,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x00,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x5f,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x5f,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x5f,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x5f,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x5f,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x5f,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x87,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x87,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x87,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x87,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x87,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0x87,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xaf,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xaf,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xaf,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xaf,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xaf,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xaf,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xd7,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xd7,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xd7,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xd7,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xd7,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xd7,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xff,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xff,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xff,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xff,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xff,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x00,
					g: 0xff,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x00,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x00,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x00,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x00,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x00,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x00,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x5f,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x5f,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x5f,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x5f,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x5f,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x5f,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x87,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x87,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x87,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x87,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x87,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0x87,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xaf,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xaf,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xaf,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xaf,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xaf,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xaf,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xd7,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xd7,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xd7,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xd7,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xd7,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xd7,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xff,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xff,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xff,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xff,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xff,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x5f,
					g: 0xff,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x00,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x00,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x00,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x00,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x00,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x00,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x5f,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x5f,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x5f,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x5f,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x5f,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x5f,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x87,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x87,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x87,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x87,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x87,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0x87,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xaf,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xaf,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xaf,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xaf,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xaf,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xaf,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xd7,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xd7,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xd7,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xd7,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xd7,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xd7,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xff,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xff,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xff,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xff,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xff,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0x87,
					g: 0xff,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x00,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x00,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x00,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x00,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x00,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x00,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x5f,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x5f,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x5f,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x5f,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x5f,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x5f,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x87,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x87,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x87,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x87,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x87,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0x87,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xaf,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xaf,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xaf,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xaf,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xaf,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xaf,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xd7,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xd7,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xd7,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xd7,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xd7,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xd7,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xff,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xff,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xff,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xff,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xff,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xaf,
					g: 0xff,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x00,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x00,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x00,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x00,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x00,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x00,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x5f,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x5f,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x5f,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x5f,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x5f,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x5f,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x87,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x87,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x87,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x87,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x87,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0x87,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xaf,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xaf,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xaf,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xaf,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xaf,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xaf,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xd7,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xd7,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xd7,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xd7,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xd7,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xd7,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xff,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xff,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xff,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xff,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xff,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xd7,
					g: 0xff,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x00,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x00,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x00,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x00,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x00,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x00,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x5f,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x5f,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x5f,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x5f,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x5f,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x5f,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x87,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x87,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x87,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x87,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x87,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0x87,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xaf,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xaf,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xaf,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xaf,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xaf,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xaf,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xd7,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xd7,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xd7,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xd7,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xd7,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xd7,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xff,
					b: 0x00,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xff,
					b: 0x5f,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xff,
					b: 0x87,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xff,
					b: 0xaf,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xff,
					b: 0xd7,
					a: 0xff,
				},
				Color {
					r: 0xff,
					g: 0xff,
					b: 0xff,
					a: 0xff,
				},
				Color {
					r: 0x08,
					g: 0x08,
					b: 0x08,
					a: 0xff,
				},
				Color {
					r: 0x12,
					g: 0x12,
					b: 0x12,
					a: 0xff,
				},
				Color {
					r: 0x1c,
					g: 0x1c,
					b: 0x1c,
					a: 0xff,
				},
				Color {
					r: 0x26,
					g: 0x26,
					b: 0x26,
					a: 0xff,
				},
				Color {
					r: 0x30,
					g: 0x30,
					b: 0x30,
					a: 0xff,
				},
				Color {
					r: 0x3a,
					g: 0x3a,
					b: 0x3a,
					a: 0xff,
				},
				Color {
					r: 0x44,
					g: 0x44,
					b: 0x44,
					a: 0xff,
				},
				Color {
					r: 0x4e,
					g: 0x4e,
					b: 0x4e,
					a: 0xff,
				},
				Color {
					r: 0x58,
					g: 0x58,
					b: 0x58,
					a: 0xff,
				},
				Color {
					r: 0x62,
					g: 0x62,
					b: 0x62,
					a: 0xff,
				},
				Color {
					r: 0x6c,
					g: 0x6c,
					b: 0x6c,
					a: 0xff,
				},
				Color {
					r: 0x76,
					g: 0x76,
					b: 0x76,
					a: 0xff,
				},
				Color {
					r: 0x80,
					g: 0x80,
					b: 0x80,
					a: 0xff,
				},
				Color {
					r: 0x8a,
					g: 0x8a,
					b: 0x8a,
					a: 0xff,
				},
				Color {
					r: 0x94,
					g: 0x94,
					b: 0x94,
					a: 0xff,
				},
				Color {
					r: 0x9e,
					g: 0x9e,
					b: 0x9e,
					a: 0xff,
				},
				Color {
					r: 0xa8,
					g: 0xa8,
					b: 0xa8,
					a: 0xff,
				},
				Color {
					r: 0xb2,
					g: 0xb2,
					b: 0xb2,
					a: 0xff,
				},
				Color {
					r: 0xbc,
					g: 0xbc,
					b: 0xbc,
					a: 0xff,
				},
				Color {
					r: 0xc6,
					g: 0xc6,
					b: 0xc6,
					a: 0xff,
				},
				Color {
					r: 0xd0,
					g: 0xd0,
					b: 0xd0,
					a: 0xff,
				},
				Color {
					r: 0xda,
					g: 0xda,
					b: 0xda,
					a: 0xff,
				},
				Color {
					r: 0xe4,
					g: 0xe4,
					b: 0xe4,
					a: 0xff,
				},
				Color {
					r: 0xee,
					g: 0xee,
					b: 0xee,
					a: 0xff,
				},
			];

			TABLE
		}
	}

	impl super::Table for VT340 {
		fn table() -> &'static [Color] {
			static TABLE: &'static [Color] = &[
				Color {
					r: (20 * 255 / 100) as u8,
					g: (20 * 255 / 100) as u8,
					b: (80 * 255 / 100) as u8,
					a: 0xff,
				},
				Color {
					r: (80 * 255 / 100) as u8,
					g: (13 * 255 / 100) as u8,
					b: (13 * 255 / 100) as u8,
					a: 0xff,
				},
				Color {
					r: (20 * 255 / 100) as u8,
					g: (80 * 255 / 100) as u8,
					b: (20 * 255 / 100) as u8,
					a: 0xff,
				},
				Color {
					r: (80 * 255 / 100) as u8,
					g: (20 * 255 / 100) as u8,
					b: (80 * 255 / 100) as u8,
					a: 0xff,
				},
				Color {
					r: (20 * 255 / 100) as u8,
					g: (80 * 255 / 100) as u8,
					b: (80 * 255 / 100) as u8,
					a: 0xff,
				},
				Color {
					r: (80 * 255 / 100) as u8,
					g: (80 * 255 / 100) as u8,
					b: (20 * 255 / 100) as u8,
					a: 0xff,
				},
				Color {
					r: (53 * 255 / 100) as u8,
					g: (53 * 255 / 100) as u8,
					b: (53 * 255 / 100) as u8,
					a: 0xff,
				},
				Color {
					r: (26 * 255 / 100) as u8,
					g: (26 * 255 / 100) as u8,
					b: (26 * 255 / 100) as u8,
					a: 0xff,
				},
				Color {
					r: (33 * 255 / 100) as u8,
					g: (33 * 255 / 100) as u8,
					b: (60 * 255 / 100) as u8,
					a: 0xff,
				},
				Color {
					r: (60 * 255 / 100) as u8,
					g: (26 * 255 / 100) as u8,
					b: (26 * 255 / 100) as u8,
					a: 0xff,
				},
				Color {
					r: (33 * 255 / 100) as u8,
					g: (60 * 255 / 100) as u8,
					b: (33 * 255 / 100) as u8,
					a: 0xff,
				},
				Color {
					r: (60 * 255 / 100) as u8,
					g: (33 * 255 / 100) as u8,
					b: (60 * 255 / 100) as u8,
					a: 0xff,
				},
				Color {
					r: (33 * 255 / 100) as u8,
					g: (60 * 255 / 100) as u8,
					b: (60 * 255 / 100) as u8,
					a: 0xff,
				},
				Color {
					r: (60 * 255 / 100) as u8,
					g: (60 * 255 / 100) as u8,
					b: (33 * 255 / 100) as u8,
					a: 0xff,
				},
				Color {
					r: (80 * 255 / 100) as u8,
					g: (80 * 255 / 100) as u8,
					b: (80 * 255 / 100) as u8,
					a: 0xff,
				},
				Color {
					r: (0 * 255 / 100) as u8,
					g: (0 * 255 / 100) as u8,
					b: (0 * 255 / 100) as u8,
					a: 0xff,
				},
			];

			TABLE
		}
	}
}
