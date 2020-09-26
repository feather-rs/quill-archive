# Memory and Data Ownership

The WebAssembly execution environment is sandboxed from the host. This can have many benefits, but can cause issues with sharing data between the host and a plugin.

# Ownership

If you are familiar with Rust's ownership model, then the Quill ownership model should be familiar to you.

### Data is owned by the Host when
- The host calls a plugin-exported function returns a pointer to a structure with backing allocations.
- The host calls a plugin-exported function returns a pointer to a structure that is allocated on the heap.

### Data is owned by the Plugin when  
- The plugin calls a host-exported function with a pointer to plugin memory.
    - This is the case in any situation, including where the data passed has backing allocations.
- The host passes data to the plugin through a pointer to a struct as a result of calling a host-exported function.

# Sharing data
Sharing data between the host and plugin is done by calling plugin and host exported functions respectively. 

## Sharing data from the host to a plugin
**Plugins cannot read host memory.**  
When a plugin calls a host-exported function that will pass data to the plugin as a result of the function call, the plugin passes a pointer to a destination struct.  
The host then writes the returned data into the destination struct.  


As defined above, the plugin has full ownership over the memory within the destination struct.

## Sharing data from the plugin to the host

### When returning a non-primitive from a plugin-exported function
When returning a non-primitive from a plugin-exported function, the return value must be allocated on the heap. The pointer to this allocation is then passed to the host as the result of the function call. 

The host is responsible for freeing any allocations created by the returned data. This includes the allocation for the returned structure itself, but also and allocations required for contained structures.

### When passing a non-primitive from a host-exported function
When the plugin passes a non-primitive from a host-exported function, it is the plugin's job to handle deallocating the pointer passed to the host. It is invalid to keep references or pointers to data passed in this manner.