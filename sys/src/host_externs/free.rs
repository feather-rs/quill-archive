use crate::raw::WasmFree;
use anyhow::Result;
use std::mem::{align_of, size_of};
use wasmer::{Instance, NativeFunc, WasmPtr};

/// Frees a value that has been allocated in WASM that should
/// be freed by the host.
///
/// Automatically determines the size and align to free, if you
/// need manual control over that use `wasm_free_unsafe`.
pub fn wasm_free<T: WasmFree>(instance: &Instance, ptr: WasmPtr<T>) -> Result<()> {
    // Get information about the value we're freeing
    let free_size = size_of::<T>() as u32;
    let free_align = align_of::<T>() as u32;

    // Get the start of the value as a byte
    let as_u8 = WasmPtr::new(ptr.offset());

    // Try to get the free function
    let free_function: NativeFunc<(WasmPtr<u8>, u32, u32)> =
        instance.exports.get_native_function("__quill_free")?;

    free_function.call(as_u8, free_size, free_align)?;

    Ok(())
}

/// Frees memory from WASM with no checks.
///
/// If you are freeing a value that is `WasmFree` it is recommended to use `wasm_free` as it is guaranteed safe.
///
/// # Safety
/// There are no checks to ensure size and align are correct, or that the memory that's being freed is meant
/// to be freed by the host.
pub unsafe fn wasm_free_unchecked(
    instance: &Instance,
    ptr: WasmPtr<u8>,
    free_size: u32,
    free_align: u32,
) -> Result<()> {
    let free_function: NativeFunc<(WasmPtr<u8>, u32, u32)> =
        instance.exports.get_native_function("__quill_free")?;

    free_function.call(ptr, free_size, free_align)?;

    Ok(())
}
