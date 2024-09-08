#[cfg(feature = "time")]
pub fn get_formatted_time() -> String {
    let time = chrono::offset::Local::now();
    let fmt = std::option_env!("EZ_LOG_TIME_FORMAT").unwrap_or("%d/%m/%Y %H:%M:%S%.3f");
    format!("{}", time.format(fmt))
}
