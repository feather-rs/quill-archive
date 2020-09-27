# SystemStage

Represents the stage at which a system should run. When used to call functions in Quill, this enum MUST be represented by a `uint8_t`.

```C
enum LogLevel {
    Pre = 1,
    Tick = 2,
    HandleEvents = 3,
    SendPackets = 4,
    CleanUp = 5
}
```