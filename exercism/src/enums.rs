
// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let msg = match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
    };
    println!("{}", msg);
    msg
}
pub fn info(message: &str) -> String {
    return format!("[INFO]: {}", message);
}
pub fn warn(message: &str) -> String {
    return format!("[WARNING]: {}", message);
}
pub fn error(message: &str) -> String {
    return format!("[ERROR]: {}", message);
}

fn main() {
    log(LogLevel::Info, "Info, world!");
    log(LogLevel::Warning, "Warning, world!");
    log(LogLevel::Error, "Error, world!");
}
