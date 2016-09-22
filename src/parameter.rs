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

use error::{self, Error};

/// An encoder with parameters.
pub trait HasParameters {
	fn set<T: Parameter<Self>>(&mut self, value: T) -> error::Result<()> {
		value.set(self)
	}

	fn get<T: Parameter<Self>>(&mut self) -> error::Result<T> {
		T::get(self)
	}
}

/// An encoder parameter.
pub trait Parameter<T: ?Sized>: Sized {
	#[allow(unused_variables)]
	fn set(self, to: &mut T) -> error::Result<()> {
		Err(Error::Unsupported("parameter not supported".into()))
	}

	#[allow(unused_variables)]
	fn get(from: &mut T) -> error::Result<Self> {
		Err(Error::Unsupported("parameter not supported".into()))
	}
}
