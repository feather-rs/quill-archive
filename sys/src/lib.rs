#![feature(alloc_layout_extra)]
#![deny(improper_ctypes)]

pub mod raw;

#[cfg(not(feature = "host"))]
pub mod module_externs;

#[cfg(feature = "host")]
pub mod host_externs;
