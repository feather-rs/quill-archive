//! Defines types that allow data flow between a Plugin host and a loaded
//! plugin.
//!
//! The `host` feature affects the compilation of this module.
//! This is because the same types have different functions on
//! the module and on the host. Additionally, `wasmer` requires
//! that data that can be dereferenced from WASM modules implements
//! a marker trait.
//!
//! The main difference between `module` types and `host` types is all pointers
//! are represented as `u32`. This is because WASM has a 32-bit usize, and using
//! the normal pointer type `*const T` would cause UB due to the difference in
//! usize. Additionally, using `u32` prevents the host from accidentally trying
//! to cast a wasm ptr directly to a reference on the host. Doing so would cause
//! UB on cast, and a segfault on dereference.

#[cfg(not(feature = "host"))]
mod module;

#[cfg(not(feature = "host"))]
pub use module::*;

#[cfg(feature = "host")]
mod host;

#[cfg(feature = "host")]
pub use host::*;

use std::convert::TryFrom;

/// C-Compatible representation of a system stage
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum SystemStage {
    /// Called before main gameplay logic runs.
    Pre = 1,
    /// Should handle all gameplay logic.
    Tick = 2,
    /// Should be used to handle events.
    HandleEvents = 3,
    /// Should be used for packet broadcasting on the
    /// server side, and packet sending on the client.
    SendPackets = 4,
    /// Should be used to clean up / reset resources
    /// at the end of the tick.
    CleanUp = 5,
}

impl TryFrom<u8> for SystemStage {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SystemStage::Pre),
            2 => Ok(SystemStage::Tick),
            3 => Ok(SystemStage::HandleEvents),
            4 => Ok(SystemStage::SendPackets),
            5 => Ok(SystemStage::CleanUp),
            _ => Err(()),
        }
    }
}
