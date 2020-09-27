# PluginSystem

Defines a system that must be run with the host's event loop.

```C
struct PluginSystem {
    uint8_t stage;
    struct PluginString* stage;
};
```