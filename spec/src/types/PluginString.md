# PluginString

Allows a UTF-8 string to be passed between the host and a plugin. 

**The array of `uint8_t` MUST be valid UTF-8.**

```C
struct PluginString {
    uint8_t* ptr;
    uint32_t length;
    struct Layout layout;
}
```