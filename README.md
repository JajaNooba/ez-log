# ez_log: Simple logging library for Rust


### Features

Ez_log can be configured using Rust features.

Default features:
 - `color`: Colored terminal output.
 - `time`: Adds time for every log output. Time format can be configured by editing `EZ_LOG_TIME_FORMAT` env variable, using chrono [format syntax].  

Optional features:
 - `dump`: Creates file for every day and writes to it log output. Directory for those files can be configured by editing `EZ_LOG_LOGS_DIR` env variable.

[format syntax]: https://docs.rs/chrono/0.4.38/chrono/format/strftime/index.html

### Overview

Logs are perform by `log_info!`, `log_warn!` and `log_error!` macros.
Usage of those macros is similar to `print_ln!` macro.

### Example 
```rs
let address = "127.0.0.1";
let port = 8000;

match start_web_server() {
    Ok() => {
        log_info!("Server started on {}:{}", address, port);
        // continue execution
    },
    Err(error) => {
        log_error!("Error while starting web server! {}", error);
        // exit program 
    }
}
```

```rs
if let Err(error) = handle_request() {
    log_warn!("Could not respond to request! {}", error);
}
```
