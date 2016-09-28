picto
=====
[![Crates.io](https://img.shields.io/crates/v/picto.svg)](https://crates.io/crates/picto) [![Crates.io](https://img.shields.io/crates/d/picto.svg)](https://crates.io/crates/picto) ![WTFPL](http://img.shields.io/badge/license-WTFPL-blue.svg) [![Build Status](https://api.travis-ci.org/kbknapp/clap-rs.svg?branch=master)](https://travis-ci.org/meh/rust-picto)

An image handling library.

Usage
-----
Add the following to the `Cargo.toml` in your project:

```toml
[dependencies]
picto = "0.2"
```

Supported Formats
-----------------
| Format | Decoding | Encoding |
|--------|----------|----------|
| PNG    | ✔        | ✔        |
| JPEG   | ✔        | ✘        |
| GIF    | ✔        | ✔        |
| BMP    | ✔        | ✔        |
| TGA    | ✔        | ✔        |
| XYZ    | ✔        | ✘        |

Documentation
-------------
Documentation is available [here](https://docs.rs/picto).

Example
-------
The following example turns an image to gray scale (maintaining the alpha),
then upscales it, and blurs it.

```rust
extern crate picto;
use picto::{read, write};
use picto::color::{Rgba, Lumaa};
use picto::processing::prelude::*;

use std::env;

fn main() {
  write::to_path(env::args().nth(2).unwrap(),
    &read::from_path::<u8, Rgba, _>(env::args().nth(1).unwrap()).unwrap()
      .convert::<u8, Lumaa>()
      .scale_by::<scaler::Cubic, u8, Rgba>(2.0)
      .blur::<u8, Rgba>(4.0)).unwrap();
}
```
