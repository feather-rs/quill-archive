use anyhow::Result;

use wasmer::{Array, Instance, Memory, ValueType, WasmPtr};

use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::Deref;

use crate::host_externs::wasm_free_unchecked;

/// Represents a `Layout`, but this one is safe to send between WASM and the Host.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AllocationLayout {
    pub size: u32,
    pub align: u32,
}

/// Represents a type that has been allocated on the heap in a WASM module.
///
/// # Safety
/// Calling `free` on `PluginBox<T>` is required to prevent memory leaks.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PluginBox<T: ValueType> {
    pub boxed: T,
    layout: AllocationLayout,
}

impl<T: ValueType> Deref for PluginBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.boxed
    }
}

unsafe impl<T> ValueType for PluginBox<T> where T: ValueType {}

/// Represents a transient reference to memory in a WASM module.
///
/// # Safety
/// It is UB to use a `PluginRef<T>` after control has been returned to WASM.
/// this is because memory could have shifted due to WASM memory management,
/// or the memory could have been deallocated.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct PluginRef<T: ValueType>(pub T);

impl<T: ValueType> Deref for PluginRef<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

unsafe impl<T> ValueType for PluginRef<T> where T: ValueType {}

/// A trait that indicates that a structure has allocations to WASM memory.
/// These allocations **MUST** be freed, not doing so would cause memory leaks.
pub trait WasmFree: ValueType {
    /// Frees all allocations in WASM memory for `Self`.
    ///
    /// It is intended that you also call `free()` on any members that implement it
    /// when writing an implementation of the trait. This is to make memory management
    /// easier, and it is part of the quill-internals style.
    fn free(self, instance: &Instance, memory: &Memory) -> Result<()>;
}

/// A type that allows Strings to be sent over FFI.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PluginString {
    pub ptr: u32,
    pub len: u32,
    string_layout: AllocationLayout,
}

impl PluginString {
    /// Try to convert a `PluginString` to a `String`.
    ///
    /// This copies the data out of the `PluginString`.
    pub fn to_string(&self, memory: &Memory) -> Option<String> {
        let ptr: WasmPtr<u8, Array> = WasmPtr::new(self.ptr);

        Some(ptr.get_utf8_string(memory, self.len)?.to_owned())
    }
}

unsafe impl ValueType for PluginString {}

impl WasmFree for PluginString {
    fn free(self, instance: &Instance, _: &Memory) -> Result<()> {
        unsafe {
            wasm_free_unchecked(
                instance,
                WasmPtr::new(self.ptr),
                self.string_layout.size as u32,
                self.string_layout.align as u32,
            )
        }
    }
}

/// A type that allows slices to be sent safely over FFI.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PluginSlice<T: ValueType> {
    pub len: u32,
    pub elements: u32, // *const [T]
    slice_layout: AllocationLayout,
    _marker: PhantomData<T>,
}

unsafe impl<T> ValueType for PluginSlice<T> where T: ValueType {}

impl<T> WasmFree for PluginSlice<T>
where
    T: ValueType,
{
    fn free(self, instance: &Instance, _: &Memory) -> Result<()> {
        unsafe {
            wasm_free_unchecked(
                instance,
                WasmPtr::new(self.elements),
                self.slice_layout.size as u32,
                self.slice_layout.align as u32,
            )
        }
    }
}

/// A type that allows slices to be sent over FFI. This type is specific to cases
/// where the `T` has freeing logic, as `PluginSlice` cannot handle that.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PluginSliceAlloc<T: ValueType + WasmFree> {
    pub len: u32,
    pub elements: u32, // *const [T]
    slice_layout: AllocationLayout,
    _marker: PhantomData<T>,
}

unsafe impl<T> ValueType for PluginSliceAlloc<T> where T: ValueType + WasmFree {}

impl<T> WasmFree for PluginSliceAlloc<T>
where
    T: ValueType + WasmFree,
{
    fn free(self, instance: &Instance, memory: &Memory) -> Result<()> {
        // Start by runnning WasmFree on the slice elements
        let slice_ptr: WasmPtr<T, Array> = WasmPtr::new(self.elements);

        let slice = slice_ptr.deref(memory, 0, self.len).unwrap();

        for element in slice.iter() {
            element.get().free(instance, memory)?;
        }

        unsafe {
            wasm_free_unchecked(
                instance,
                WasmPtr::new(self.elements),
                self.slice_layout.size as u32,
                self.slice_layout.align as u32,
            )
        }
    }
}

/// A type that allows system definitions to be sent over FFI.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PluginSystem {
    pub stage: crate::raw::SystemStage,
    pub name: PluginString,
}

unsafe impl ValueType for PluginSystem {}

impl WasmFree for PluginSystem {
    fn free(self, instance: &Instance, memory: &Memory) -> Result<()> {
        self.name.free(instance, memory)
    }
}

/// A type that defines a plugin's properties.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PluginRegister {
    pub name: PluginString,
    pub version: PluginString,
    pub systems: PluginSliceAlloc<PluginSystem>,
}

unsafe impl ValueType for PluginRegister {}

impl WasmFree for PluginRegister {
    fn free(self, instance: &Instance, memory: &Memory) -> Result<()> {
        self.name.free(instance, memory)?;
        self.version.free(instance, memory)?;
        self.systems.free(instance, memory)
    }
}

impl PluginBox<PluginRegister> {
    pub fn free_ptr_to(
        self,
        ptr: WasmPtr<PluginBox<PluginRegister>>,
        instance: &Instance,
    ) -> Result<()> {
        // Get layout required to store the type
        let layout = self.layout;
        unsafe {
            wasm_free_unchecked(
                instance,
                WasmPtr::new(ptr.offset()),
                layout.size as u32,
                layout.align as u32,
            )
        }
    }
}

/// Defines a level that a message should be logged at.
#[repr(u8)]
pub enum LogLevel {
    Debug = 1,
    Info = 2,
}

impl TryFrom<u8> for LogLevel {
    type Error = ();

    fn try_from(from: u8) -> Result<LogLevel, ()> {
        match from {
            1 => Ok(LogLevel::Debug),
            2 => Ok(LogLevel::Info),
            _ => Err(()),
        }
    }
}
