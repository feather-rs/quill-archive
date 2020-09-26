//! Contains wrappers for calls to a WASM module from the host.
//!
//! The purpose for this crate it to define and implement easy
//! methods for calling module functions, and ensuring they are
//! called correctly.

mod free;

pub use free::{wasm_free, wasm_free_unchecked};
