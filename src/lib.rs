#![feature(lang_items, box_patterns)]
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate num;
extern crate nix;

extern crate bitflags;

pub mod kbindings;

#[cfg(feature = "api")]
pub mod api;
