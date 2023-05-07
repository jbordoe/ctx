use colored::*;

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub fn log(severity: LogLevel, message: &str) {
    let severity_color = match severity {
        LogLevel::Debug => Color::Purple,
        LogLevel::Info => Color::LightBlue,
        LogLevel::Warn => Color::Yellow,
        LogLevel::Error => Color::Red,
    };
    println!(
        "{}: {}",
        "ctx".color(severity_color).bold(),
        message.color(Color::Grey)
    );
}

