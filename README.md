# lovely-env-logger

[![Crates.io](https://img.shields.io/crates/v/lovely_env_logger.svg)](https://crates.io/crates/lovely_env_logger)
[![Docs](https://docs.rs/lovely_env_logger/badge.svg)](https://docs.rs/lovely_env_logger)
[![MIT/APACHE-2.0](https://img.shields.io/crates/l/lovely_env_logger.svg)](https://crates.io/crates/lovely_env_logger)

A simple logger built on top of [env_logger](https://docs.rs/env_logger).
It is configured via an environment variable and writes to standard
error with nice colored output for log levels.
Originally a fork from [pretty_env_logger](https://github.com/seanmonstar/pretty-env-logger).

## Screenshots
![example default output](log_default.png)
![example output with relative timestamps, short
levels](log_relative_timestamps.png)
![example output with system timestamps, file names and line numbers](log_file_line_system_time.png)

## Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
log = "0.4"
lovely_env_logger = "0.5"
```

Add some usage to your application:

```rust
extern crate lovely_env_logger;
#[macro_use] extern crate log;

fn main() {
    lovely_env_logger::init();
    info!("such information");
    warn!("o_O");
    error!("much error");
}
```

Then run your app with the environmental variable set:

```
RUST_LOG=trace cargo run
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

