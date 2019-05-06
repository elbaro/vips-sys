#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(vips_8_74)]
include!(concat!(env!("OUT_DIR"),"/binding_8_74.rs"));

#[cfg(not(vips_8_74))]
include!(concat!(env!("OUT_DIR"),"/binding.rs"));
