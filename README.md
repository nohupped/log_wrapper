# log_wrapper

[![docs](https://docs.rs/log_wrapper/badge.svg)](https://docs.rs/log_wrapper)
[![crates.io](https://img.shields.io/crates/v/log_wrapper.svg)](https://crates.io/crates/log_wrapper)

Just some boilerplate code for a logger wrapped over the [log](https://docs.rs/log/0.4.10/log/) crate that logs to STDOUT. This has a `global static ConsoleLogger` struct that over-rides the `log::Log` trait. Once this is initiated with the `ConsoleLogger::new(loglevel)`, where loglevel is either of `trace, debug, info, warn, error, off`, the standard macros from `log` crate such as `info!`, `warn!`, etc. can be used to print and the output will be filtered as per the defined loglevel.

[Documentation](https://docs.rs/log_wrapper)

Eg:

somemod/src/main.rc

```rust
#[macro_use]
extern crate log;
use log_wrapper::{ConsoleLogger};
use somemod_backend::print_log;
fn main() {
    ConsoleLogger::new("warn".to_string());
    info!("This is info");
    warn!("warning");
    error!("oops");
    println!("Now from module");
    print_log();

}
```

somemod_backend/src/lib.rs

```rust
#[macro_use]
extern crate log;
pub mod nested_mod{
    pub fn print_log() {
        info!("This is info");
        warn!("warning");
        error!("oops");
    }
}
```

Output:

```bash
23/02/2020:02:29:10 WARN module:somemod file:somemod/src/main.rs:8 warning
23/02/2020:02:29:10 ERROR module:somemod file:somemod/src/main.rs:9 oops
Now from module
23/02/2020:02:29:10 WARN module:somemod_backend::nested_mod file:somemod_backend/src/lib.rs:6 warning
23/02/2020:02:29:10 ERROR module:somemod_backend::nested_mod file:somemod_backend/src/lib.rs:7 oops
```

## Why

This is just the boilerplate code taken from the [log](https://docs.rs/log/0.4.10/log/) crate itself that I wanted to use so that I don't have to type it every time.
