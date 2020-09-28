# PluginSliceAlloc<T>
Allows an unsized sequence of `T` to be passed between the host and a plugin. The data within `PluginSlice` must be able to be represented only by its raw bytes, but also has backing allocations that MUST be freed.

```C
struct PluginSlice {
    uint32_t length;
    T* elements;
    struct Layout layout;
};
```

