#![allow(unused_imports)]

///
/// `log_info!` macro is used to log informations.
///
/// Logs contains label, file, and message.
///
/// The usage is similar to `println!` macro.
///
/// ### Example
/// ```
/// let address = "127.0.0.1";
/// let port = 8000;
///
/// // start web server
///
/// log_info!("Server started on {}:{}", address, port);
/// ```
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        let args = std::fmt::format(format_args!($($arg)*));
        let msg = format!("{} [{}] {}", $crate::get_msg_label($crate::LogType::Info), file!(),args);

        #[cfg(feature = "dump")]
        $crate::dump_log($crate::LogType::Info, args);

        println!("{msg}");
    };
}

pub use log_info;

///
/// `log_warn!` macro is used to log warning.
///
/// Logs contains label, file and message.
///
/// The usage is similar to `println!` macro.
///
/// ### Example
/// ```
/// match handle_request() {
///     Ok() => todo!(),
///     Err(why) => {
///         log_warn!("Could not respond to request! {}", why);
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        let args = std::fmt::format(format_args!($($arg)*));

        let msg = format!("{} [{}] {}", $crate::get_msg_label($crate::LogType::Warn), file!(), args);

        #[cfg(feature = "dump")]
        $crate::dump_log($crate::LogType::Warn, args);

        println!("{msg}");
    };
}

pub use log_warn;

///
/// `log_error!` macro is used to log errors.
///
/// Logs contains label, file, and message.
///
/// The usage is similar to `println!` macro.
///
/// ### Example
/// ```
/// match start_web_server() {
///     Ok() => todo!(),
///     Err(error) => {
///         log_error("Error while starting web server! {}", error);
///     }
/// }
/// ```
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        let args = std::fmt::format(format_args!($($arg)*));

        let msg = format!("{} [{}] {}", $crate::get_msg_label($crate::LogType::Error), file!(), args);

        #[cfg(feature = "dump")]
        $crate::dump_log($crate::LogType::Error, args);

        println!("{msg}");
    };
}

pub use log_error;
