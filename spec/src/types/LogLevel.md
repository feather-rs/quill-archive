# LogLevel

Represents a log level for a `log` invocation. When used to call functions in Quill, this enum MUST be represented by a `uint8_t`.

```C
enum LogLevel {
    Debug = 1,
    Info = 2
}
```