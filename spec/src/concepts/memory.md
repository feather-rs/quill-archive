# Memory and Data Ownership

The WebAssembly execution environment is sandboxed from the host. This has useful benefits in terms of security and error isolation, but comes at the expense of increased complexity when the host and the plugin interacts.

Due to the sandboxed memory model of WebAssembly, a plugin cannot read arbitrary host memory. This causes complexity when moving data from the host to a plugin because the plugin must pass a pointer to a struct for return values from a call to a host-exported function.  


Additionally, in cases where a plugin-exported function has a non [WebAssembly primitive](https://webassembly.github.io/spec/core/syntax/types.html#syntax-valtype) return value, the return value needs to be allocated on the plugin's heap. WebAssembly's sandboxed memory model prevents a plugin and the host sharing a heap, which would allieviate issues with passing structures as return values. This means that the host must call a plugin's exported `free` function once it is finished with the returned data.

# Ownership

If you are familiar with Rust's ownership model, then the Quill ownership model should be familiar to you.

When data is owned, the owner is guaranteed that **the data will never be changed by any external source**.   
Additionally, a contract is created in which the owner **__MUST__ free any allocations involved in the owned data**.

### Data is owned by the Host when
- The host calls a plugin-exported function that returns a pointer to a structure with backing allocations.
- The host calls a plugin-exported function returns a pointer to a structure that is allocated on the heap.

### Data is owned by the Plugin when  
- The plugin calls a host-exported function with a pointer to plugin memory where the pointer is taken as an argument in a host-exported function.
    - This is the case in any situation, including where the data passed has backing allocations.
- The host passes data to the plugin through a pointer to a struct as a result of calling a host-exported function.

# Sharing data
Sharing data between the host and plugin is done by calling plugin and host exported functions respectively. 

## Sharing data from the host to a plugin
**Plugins cannot read arbitrary host memory.**  
Plugins are limited to reading the linear memory they've been allocated.

When a plugin calls a host-exported function that will pass data to the plugin as a result of the function call, the plugin passes a pointer to a destination struct as the final function argument(s).
The host MUST write the returned data into the destination struct.  


As defined above, the plugin has full ownership over the memory within the destination struct. This includes the allocation where the struct resides as well as any pointers within the struct.

## Sharing data from the plugin to the host

### When returning a non-WebAssembly primitive from a plugin-exported function
When returning a non-WebAssembly primitive from a plugin-exported function, the return value must be allocated on the heap. The pointer to this allocation is then passed to the host as the result of the function call. 

The host is responsible for freeing any allocations created by the returned data. This includes the allocation for the returned structure itself, but also and allocations required for contained structures.

### When passing a non-WebAssembly primitive from a host-exported function
When the plugin passes a non-WebAssembly primitive from a host-exported function, it is the plugin's job to handle deallocating the pointer passed to the host. It is invalid to keep references or pointers to data passed in this manner.