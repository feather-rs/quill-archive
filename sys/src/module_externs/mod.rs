//! Contains safe wrappers for calls to a host from WASM.
//!
//! Methods in this crate are only safe to call from within wasm
//! as they require exported functions. The functions called by
//! these wrappers are a standard part of the Quill API and will
//! always be available.

mod log;

pub use log::log;
