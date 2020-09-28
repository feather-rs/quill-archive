use std::alloc::Layout as AllocationLayout;
use std::convert::TryFrom;
use std::ops::Deref;

/// Represents a `Layout`, but this one is safe to send between WASM and the Host.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Layout {
    pub size: usize,
    pub align: usize,
}

impl From<AllocationLayout> for Layout {
    fn from(from: AllocationLayout) -> Self {
        Self {
            size: from.size(),
            align: from.align(),
        }
    }
}

impl Into<AllocationLayout> for Layout {
    fn into(self) -> AllocationLayout {
        AllocationLayout::from_size_align(self.size, self.align).unwrap()
    }
}

/// Represents a type that has been allocated on the heap in a WASM module.
///
/// # Safety
/// Calling `free` on `PluginBox<T>` is required to prevent memory leaks.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PluginBox<T: Clone + Copy> {
    pub boxed: T,
    pub layout: Layout,
}

/// Represents a transient reference to memory in a WASM module.
///
/// # Safety
/// It is UB to use a `PluginRef<T>` after control has been returned to WASM.
/// this is because memory could have shifted due to WASM memory management,
/// or the memory could have been deallocated.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct PluginRef<T: Copy>(pub T);

impl<T: Clone + Copy> Deref for PluginBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.boxed
    }
}

impl<T: Clone + Copy> Deref for PluginRef<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

/// A type that allows Strings to be sent over FFI.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PluginString {
    ptr: *const u8,
    len: usize,
    slice_layout: Layout,
}

impl PluginString {
    /// Creates a `PluginRef<PluginString>` from a string slice.
    ///
    /// # Safety
    /// The string slice must live longer than the returned `PluginRef<PluginString>`
    pub unsafe fn from_borrow(string: &str) -> PluginRef<Self> {
        PluginRef(Self {
            ptr: string.as_ptr(),
            len: string.len(),
            slice_layout: AllocationLayout::new::<u8>()
                .repeat(string.len())
                .unwrap()
                .0
                .into(),
        })
    }
}

impl From<String> for PluginString {
    fn from(string: String) -> Self {
        let as_str_boxed = string.into_boxed_str();

        PluginString {
            len: as_str_boxed.len(),
            slice_layout: AllocationLayout::new::<u8>()
                .repeat(as_str_boxed.len())
                .unwrap()
                .0
                .into(),
            ptr: Box::into_raw(as_str_boxed) as *const u8,
        }
    }
}

impl From<&str> for PluginString {
    fn from(str: &str) -> Self {
        let as_str_boxed: Box<[u8]> = Box::from(str.as_bytes());

        PluginString {
            len: as_str_boxed.len(),
            slice_layout: AllocationLayout::new::<u8>()
                .repeat(str.len())
                .unwrap()
                .0
                .into(),
            ptr: Box::into_raw(as_str_boxed) as *const u8,
        }
    }
}

/// A type that allows slices to be sent over FFI.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PluginSlice<T: Copy + Clone> {
    len: usize,
    elements: *const [T],
    slice_layout: Layout,
}

impl<T> From<&[T]> for PluginSlice<T>
where
    T: Clone + Copy,
{
    fn from(from: &[T]) -> PluginSlice<T> {
        let as_box: Box<[T]> = from.into();
        PluginSlice {
            len: as_box.len(),
            elements: Box::into_raw(as_box),
            slice_layout: AllocationLayout::new::<T>()
                .repeat(from.len())
                .unwrap()
                .0
                .into(),
        }
    }
}

/// A type that allows slices to be sent over FFI. This type is specific to cases
/// where the `T` has freeing logic, as `PluginSlice` cannot handle that.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PluginSliceAlloc<T: Copy + Clone> {
    len: usize,
    elements: *const T,
    slice_layout: Layout,
}

impl<T> From<&[T]> for PluginSliceAlloc<T>
where
    T: Clone + Copy,
{
    fn from(from: &[T]) -> PluginSliceAlloc<T> {
        let as_box: Box<[T]> = from.into();
        PluginSliceAlloc {
            len: as_box.len(),
            slice_layout: AllocationLayout::new::<T>()
                .repeat(from.len())
                .unwrap()
                .0
                .into(),
            elements: Box::into_raw(as_box) as *const T,
        }
    }
}

/// A type that allows system definitions to be sent over FFI.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PluginSystem {
    pub stage: super::SystemStage,
    pub name: PluginString,
}

/// A type that defines a plugin's properties.
///
/// # Examples
/// Despite the type definition, it is relatively easy to
/// construct.
/// ```
/// # use quill_internals::raw::{PluginRegister, PluginSystem, SystemStage};
/// let plugin_register = PluginRegister {
///     name: "Plugin Name".into(), // PluginBox<PluginString> can be created from a &str
///     version: "1.0.0".into(), // See above
///     systems: (&[PluginSystem {
///            stage: SystemStage::Tick,
///            name: "plugin_system".into()
///              }] as &[_]).into()
///     };
/// ```
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PluginRegister {
    pub name: PluginString,
    pub version: PluginString,
    pub systems: PluginSliceAlloc<PluginSystem>,
}

impl Into<*const PluginBox<PluginRegister>> for PluginRegister {
    fn into(self) -> *const PluginBox<PluginRegister> {
        let boxed_self = Box::new(PluginBox {
            boxed: self,
            layout: AllocationLayout::new::<PluginBox<PluginRegister>>().into(),
        });

        Box::into_raw(boxed_self)
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
