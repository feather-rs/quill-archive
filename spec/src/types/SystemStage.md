# SystemStage

Represents the stage at which a system should run. When used to call functions in Quill, this enum MUST be represented by a `uint8_t`.

```C
enum SystemStage {
    FEATHER_STAGE_PRE = 1,
    FEATHER_STAGE_TICK = 2,
    FEATHER_STAGE_HANDLE_EVENTS = 3,
    FEATHER_STAGE_SEND_PACKETS = 4,
    FEATHER_STAGE_CLEAN_UP = 5
};
```