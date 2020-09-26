# Free

```rs
fn free(ptr: *const u8, size: u8, align: u8)
```

Tells the plugin to free allocated data at `ptr` with `size` and `align`.

**Assume that using an incorrect size and align will cause UB**