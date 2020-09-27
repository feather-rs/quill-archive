# LogLevel

Represents a log level for a `log` invocation. When used to call functions in Quill, this enum MUST be represented by a `uint8_t`.

```C
enum LogLevel {
    FEATHER_LOG_DEBUG = 1,
    FEATHER_LOG_INFO = 2
};
```