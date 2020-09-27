# Layout

Allows data with allocations to be freed correctly.

```C
struct Layout {
    uint32_t size;
    uint32_t align;
};
```