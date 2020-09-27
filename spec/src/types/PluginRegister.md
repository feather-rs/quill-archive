# PluginRegister

Defines a plugin's meta as well as systems the plugin can run.

**The `PluginSliceAlloc` MUST contain `PluginSystem`s**

```C
struct PluginRegister {
    struct PluginString* name;
    struct PluginString* version;
    struct PluginSliceAlloc* systems;
};
```