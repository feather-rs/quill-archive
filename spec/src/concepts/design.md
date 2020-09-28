# Design

Quill is designed to be a powerful API that allows plugins to interact with a Minecraft server implementation.  

## ECS
ECS is the foundation of Quill's API. A host that provides Quill API support will use an ECS to define its state. Using methods exposed by the host, a plugin can access and modify the host's state with the same, or nearly the same, level of control that the server itself can. This design allows plugins to be incredibly powerful, comparable to the host itself.