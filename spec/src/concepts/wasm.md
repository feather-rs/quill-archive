# WASM
The Quill plugin API is based on plugins compiled into WebAssembly. If you are unfamiliar with WebAssembly, visit [https://webassembly.org](https://webassembly.org) to learn more.


WebAssembly was chosen as the compilation target for plugins because
1. It's cross-platform
2. Language agnostic on the host and plugin
3. Restrictive sandboxing on running plugins

