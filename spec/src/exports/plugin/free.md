# Free

```c
void free(void*, unsigned int, unsigned int)
```

Arguments (in order):
- The pointer to free
- Size of the allocation
- Align of the allocation

**The `size` and `align` parameters MUST be equal to the `size` and `align` values given to `alloc` when the memory block at `pointer` was allocated.**