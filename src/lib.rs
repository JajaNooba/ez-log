//! # ez_log: Simple logging library for Rust
//!
//!
//! ### Features
//!
//! Ez_log can be configured using Rust features.
//!
//! Default features:
//! - `color`: Colored terminal output.
//! - `time`: Adds time for every log output. Time format can be configured by editing
//!     `EZ_LOG_TIME_FORMAT` env variable, using chrono [format syntax].
//!
//! Optional features:
//! - `dump`: Creates file for every day and writes to it log output. Directory for those files can
//!     be configured by editing `EZ_LOG_LOGS_DIR` env variable.
//!
//! [format syntax]: https://docs.rs/chrono/0.4.38/chrono/format/strftime/index.html
//!
mod macros;
mod utils;

#[allow(dead_code)]
enum LogType {
    Info,
    Warn,
    Error,
}

#[allow(dead_code)]
fn get_msg_label(log_type: LogType) -> String {
    #[cfg(not(feature = "color"))]
    match log_type {
        LogType::Info => "[INFO]",
        LogType::Warn => "[WARN]",
        LogType::Error => "[Error]",
    }

    #[cfg_attr(feature = "color", cfg(not(feature = "time")))]
    {
        use colorized::*;
        match log_type {
            LogType::Info => format!("[{}]", "INFO".color(Colors::BrightBlueFg)),
            LogType::Warn => format!("[{}]", "WARN".color(Colors::BrightYellowFg)),
            LogType::Error => format!("[{}]", "ERROR".color(Colors::BrightRedFg)),
        }
    }

    #[cfg_attr(feature = "time", cfg(not(feature = "color")))]
    {
        match log_type {
            LogType::Info => format!("{} [INFO]", utils::get_formatted_time()),
            LogType::Warn => format!("{} [WARN]", utils::get_formatted_time()),
            LogType::Error => format!("{} [ERROR]", utils::get_formatted_time()),
        }
    }

    #[cfg_attr(feature = "color", cfg(feature = "time"))]
    {
        use colorized::*;
        match log_type {
            LogType::Info => format!(
                "{} [{}]",
                utils::get_formatted_time().color(Colors::GreenFg),
                "INFO".color(Colors::BrightBlueFg)
            ),
            LogType::Warn => format!(
                "{} [{}]",
                utils::get_formatted_time().color(Colors::GreenFg),
                "WARN".color(Colors::BrightYellowFg),
            ),
            LogType::Error => format!(
                "{} [{}]",
                utils::get_formatted_time().color(Colors::GreenFg),
                "ERROR".color(Colors::BrightRedFg),
            ),
        }
    }
}

#[allow(dead_code)]
#[cfg(feature = "dump")]
fn dump_log(log_type: LogType, log_file: String, msg: String) {
    use std::{fs, io::Write, path::Path};

    let log_dir_str = std::option_env!("EZ_LOG_LOGS_DIR").unwrap_or("./logs");

    let log_dir_path = Path::new(log_dir_str);

    if !log_dir_path.exists() {
        fs::create_dir_all(log_dir_path).unwrap();
    }

    let time = chrono::offset::Local::now();
    let file_name = format!("{}", time.format("%d_%m_%Y"));
    let file_path = format!("{log_dir_str}/{file_name}");

    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .unwrap();

    let content = match log_type {
        LogType::Info => format!(
            "{} [INFO] [{}] {}\n",
            utils::get_formatted_time(),
            log_file,
            msg
        ),
        LogType::Warn => format!(
            "{} [WARN] [{}] {}\n",
            utils::get_formatted_time(),
            log_file,
            msg
        ),
        LogType::Error => format!(
            "{} [ERROR] [{}] {}\n",
            utils::get_formatted_time(),
            log_file,
            msg
        ),
    };

    if let Err(e) = file.write(content.as_bytes()) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn info_test() {
        log_info!("info test {}", 123);
    }

    #[test]
    fn warn_test() {
        log_warn!("warn test {}", 123);
    }

    #[test]
    fn error_test() {
        log_error!("error test {}", 123);
    }
}
