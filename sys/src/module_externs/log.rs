use crate::raw::{LogLevel, PluginRef, PluginString};

extern "C" {
    fn unsafe_log(level: u8, string: *const PluginRef<PluginString>);
}

/// Logs a message using the specified `LogLevel`
pub fn log<S>(level: LogLevel, message: S)
where
    S: AsRef<str>,
{
    let as_str = message.as_ref();

    unsafe { unsafe_log(level as u8, &PluginString::from_borrow(as_str)) };
}
