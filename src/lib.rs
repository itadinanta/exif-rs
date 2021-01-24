//
// Copyright (c) 2016 KAMADA Ken'ichi.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//

//! This is a pure-Rust library to parse Exif data.
//!
//! This library parses Exif attributes in a raw Exif data block.
//! It can also read Exif data directly from some image formats
//! including TIFF, JPEG, HEIF, PNG, and WebP.
//!
//! # Examples
//!
//! An example to parse JPEG/TIFF files:
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! for path in &["tests/exif.jpg", "tests/exif.tif"] {
//!     let file = std::fs::File::open(path)?;
//!     let mut bufreader = std::io::BufReader::new(&file);
//!     let exifreader = exif::Reader::new();
//!     let exif = exifreader.read_from_container(&mut bufreader)?;
//!     for f in exif.fields() {
//!         println!("{} {} {}",
//!                  f.tag, f.ifd_num, f.display_value().with_unit(&exif));
//!     }
//! }
//! # Ok(()) }
//! ```
//!
//! # Upgrade Guide
//!
//! See the [upgrade guide](doc/upgrade/index.html) for API incompatibilities.

pub use error::Error;
pub use jpeg::get_exif_attr as get_exif_attr_from_jpeg;
pub use reader::{Exif, Reader};
pub use tag::{Context, Tag};
pub use tiff::{DateTime, Field, In};
pub use tiff::parse_exif_compat03 as parse_exif;
pub use value::Value;
pub use value::{Rational, SRational};

/// The interfaces in this module are experimental and unstable.
pub mod experimental {
    pub use crate::writer::Writer;
}

#[cfg(test)]
#[macro_use]
mod tmacro;

pub mod doc;
mod endian;
mod error;
mod isobmff;
mod jpeg;
mod png;
mod reader;
mod tag;
mod tiff;
mod util;
mod value;
mod webp;
mod writer;
