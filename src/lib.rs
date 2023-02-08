#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/lovely_env_logger/latest")]

//! A logger configured via environment variables which writes to standard
//! error with nice colored output for log levels.
//!
//! ## Example
//!
//! ```
//! extern crate lovely_env_logger;
//! #[macro_use] extern crate log;
//!
//! fn main() {
//!     lovely_env_logger::init_default();
//!
//!     trace!("a trace example");
//!     debug!("deboogging");
//!     info!("such information");
//!     warn!("o_O");
//!     error!("boom");
//! }
//! ```
//!
//! Run the program with the environment variable `RUST_LOG=trace`.
//!
//! ## Defaults
//!
//! The defaults can be setup by calling `init_default()` or
//! `try_init_default()` at the start of the program.
//!
//! ## Enable logging
//!
//! This crate uses [env_logger][] internally, so the same ways of enabling
//! logs through an environment variable are supported.
//!
//! ## Configuration through environment variables
//! Some options can be set through environment variables that have priority
//! over configuration set through the `Config` structure.
//!
//! The following options are set through environment variables named by
//! adding a suffix added to `RUST_LOG` or the environment variable used to
//! filter the traces.
//!
//! ### `RUST_LOG_SHORT_LEVELS`
//! Display levels on 3 characters to `1`. Display them as 5 characters
//! otherwise.
//!
//! ### `RUST_LOG_WITH_FILE_NAME`
//! Display the file calling the log macro when set to `1`. Disable it otherwise.
//!
//! ### `RUST_LOG_WITH_LINE_NUMBER`
//! Display the line number calling the log macro when set to `1`. Disable it otherwise.
//!
//! ### `RUST_LOG_WITH_PADDING`
//! Display the log lines with padding after module name, when set to `1`. Disable it otherwise.
//!
//! ### `RUST_LOG_WITH_SYSTEM_TIMESTAMPS`
//! Enable timestamps when set to `1`. Disable it otherwise.
//! Requires to be compiled with the `humantime` feature.
//!
//! ### `RUST_LOG_WITH_RELATIVE_TIMESTAMPS`
//! When set to `1`, display timestamps using the difference compared to the
//! previous log, or the date of log if the difference is too large.
//! Requires to be compiled with the `reltime` feature.
//!
//! [env_logger]: https://docs.rs/env_logger

#[doc(hidden)]
pub extern crate env_logger;

extern crate log;

#[cfg(feature = "reltime")]
use chrono::{DateTime, Local, Timelike};
#[cfg(feature = "reltime")]
use std::sync::{Arc, Mutex};

use std::default::Default;
use std::env;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

use env_logger::{
    fmt::{Color, Style, StyledValue},
    Builder,
};
use log::Level;

/// Default environment variable to filter logs
const RUST_LOG_ENV: &str = "RUST_LOG";

/// Configuration for the lovely env logger
pub struct Config {
    #[cfg(feature = "humantime")]
    /// Whether to display a timestamp
    pub with_system_timestamp: bool,
    #[cfg(feature = "reltime")]
    /// Whether to display a timestamp as reltime
    pub reltime: bool,

    /// Display levels as 5 or 3 letters
    pub short_levels: bool,
    /// Display the file calling the log macro
    pub with_file_name: bool,
    /// Display the line number calling the log macro
    pub with_line_number: bool,
    /// Pad the log line after module names
    pub with_padding: bool,
}

impl Default for Config {
    /// Creates a new Config for the lovely env logger
    #[inline]
    fn default() -> Self {
        Self {
            #[cfg(feature = "humantime")]
            with_system_timestamp: false,
            #[cfg(feature = "reltime")]
            reltime: false,
            short_levels: false,
            with_file_name: false,
            with_line_number: false,
            with_padding: false,
        }
    }
}
impl Config {
    /// Creates a new Config for the lovely env logger, with timestamps
    /// enabled
    #[inline]
    #[cfg(feature = "humantime")]
    pub fn new_timed() -> Self {
        Self {
            with_system_timestamp: true,
            ..Self::default()
        }
    }
    /// Creates a new Config for the lovely env logger, with relative
    /// timestamps enabled
    #[inline]
    #[cfg(feature = "reltime")]
    pub fn new_reltime() -> Self {
        Self {
            reltime: true,
            ..Self::default()
        }
    }

    /// Creates a new Config for the lovely env logger,
    /// with values from the defined environment_variable_prefix, or from the
    /// fallback configuration
    #[inline]
    fn from_environment_variables(environment_variable_prefix: &str, fallback_cfg: Self) -> Self {
        Self {
            #[cfg(feature = "humantime")]
            with_system_timestamp: match env::var_os(
                environment_variable_prefix.to_owned() + "_WITH_SYSTEM_TIMESTAMPS",
            ) {
                Some(v) => v == "1",
                None => fallback_cfg.with_system_timestamp,
            },
            #[cfg(feature = "reltime")]
            reltime: match env::var_os(
                environment_variable_prefix.to_owned() + "_WITH_RELATIVE_TIMESTAMPS",
            ) {
                Some(v) => v == "1",
                None => fallback_cfg.reltime,
            },
            short_levels: match env::var_os(
                environment_variable_prefix.to_owned() + "_SHORT_LEVELS",
            ) {
                Some(v) => v == "1",
                None => fallback_cfg.short_levels,
            },
            with_file_name: match env::var_os(
                environment_variable_prefix.to_owned() + "_WITH_FILE_NAME",
            ) {
                Some(v) => v == "1",
                None => fallback_cfg.with_file_name,
            },
            with_line_number: match env::var_os(
                environment_variable_prefix.to_owned() + "_WITH_LINE_NUMBER",
            ) {
                Some(v) => v == "1",
                None => fallback_cfg.with_line_number,
            },
            with_padding: match env::var_os(
                environment_variable_prefix.to_owned() + "_WITH_PADDING",
            ) {
                Some(v) => v == "1",
                None => fallback_cfg.with_padding,
            },
        }
    }
}

/// Initializes the global logger with a lovely env logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
pub fn init(config: Config) {
    try_init(config).unwrap();
}

/// Initializes the global logger with a lovely env logger, with default
/// settings.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
pub fn init_default() {
    try_init(Config::default()).unwrap();
}

/// Initializes the global logger with a lovely env logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init(config: Config) -> Result<(), log::SetLoggerError> {
    try_init_custom_env(config, RUST_LOG_ENV)
}

/// Initializes the global logger with a lovely env logger, with default
/// settings.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_default() -> Result<(), log::SetLoggerError> {
    try_init_custom_env(Config::default(), RUST_LOG_ENV)
}

/// Initialized the global logger with a lovely env logger, with a custom variable name.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
pub fn init_custom_env(config: Config, environment_variable_name: &str) {
    try_init_custom_env(config, environment_variable_name).unwrap();
}

/// Initialized the global logger with a lovely env logger, with a custom variable name.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_custom_env(
    config: Config,
    environment_variable_name: &str,
) -> Result<(), log::SetLoggerError> {
    let cfg = Config::from_environment_variables(environment_variable_name, config);
    let mut builder = formatted_builder(cfg);
    if let Ok(s) = ::std::env::var(environment_variable_name) {
        builder.parse_filters(&s);
    }
    builder.try_init()
}

/// Returns a `env_logger::Builder` for further customization.
///
/// This method will return a colored and formatted `env_logger::Builder`
/// for further customization. Refer to env_logger::Build crate documentation
/// for further details and usage.
pub fn formatted_builder(config: Config) -> Builder {
    let mut builder = Builder::new();
    #[cfg(feature = "reltime")]
    let last_time = Arc::new(Mutex::new(Local::now()));

    builder.format(move |f, record| {
        use std::io::Write;

        let (target, location) = compute_target_and_location(record, &config);

        let mut style = f.style();
        let level = colored_level(&mut style, record.level(), config.short_levels);

        let mut style = f.style();
        let target = style.set_bold(true).value(target);
        #[cfg(feature = "reltime")]
        {
            if config.reltime {
                let reltime = compute_reltime(&last_time);
                let mut style = f.style();
                let is_delta = reltime.is_delta();
                let reltime = style.set_bold(!is_delta).value(&reltime);

                return writeln!(
                    f,
                    "{} {} {}{} {}",
                    reltime,
                    level,
                    target,
                    location,
                    record.args(),
                );
            }
        }
        #[cfg(feature = "humantime")]
        {
            if config.with_system_timestamp {
                let time = f.timestamp_millis();
                return writeln!(
                    f,
                    "{} {} {}{} {}",
                    time,
                    level,
                    target,
                    location,
                    record.args(),
                );
            }
        }
        if config.with_padding {
            writeln!(f, "{} {}{} > {}", level, target, location, record.args(),)
        } else {
            writeln!(f, "{} {}{} {}", level, target, location, record.args(),)
        }
    });

    builder
}

struct Padded<T> {
    value: T,
    width: usize,
}

impl<T: fmt::Display> fmt::Display for Padded<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: <width$}", self.value, width = self.width)
    }
}

enum OptionalPadded<T> {
    None,
    Some { value: T, width: usize },
}

impl<T: fmt::Display> fmt::Display for OptionalPadded<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let OptionalPadded::Some { value, width } = self {
            write!(f, "{value: <width$}")
        } else {
            fmt::Result::Ok(())
        }
    }
}

static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

fn compute_target_and_location<'a>(
    record: &log::Record<'a>,
    config: &Config,
) -> (Padded<&'a str>, OptionalPadded<String>) {
    let target = record.target();
    let opt_file = if config.with_file_name {
        record.file()
    } else {
        None
    };
    let opt_line = if config.with_line_number {
        record.line()
    } else {
        None
    };
    let target_len = target.len();
    let (added_opt, added_len) = match (opt_file, opt_line) {
        (None, None) => (None, 0),
        (Some(file), None) => (Some(format!(":{file}")), file.len() + 1),
        (None, Some(line)) => {
            let line_str: String = line.to_string();
            (Some(format!(":{line_str}")), line_str.len() + 1)
        }
        (Some(file), Some(line)) => {
            let line_str: String = line.to_string();
            (
                Some(format!(":{file}:{line_str}")),
                file.len() + line_str.len() + 2,
            )
        }
    };
    let full_width = if config.with_padding {
        max_target_width(target_len + added_len)
    } else {
        target_len + added_len
    };
    if let Some(added) = added_opt {
        let target_padded = Padded {
            value: target,
            width: target_len,
        };
        let location_padded = OptionalPadded::Some {
            value: added,
            width: full_width - target_len,
        };
        (target_padded, location_padded)
    } else {
        let target_padded = Padded {
            value: target,
            width: full_width,
        };
        (target_padded, OptionalPadded::None)
    }
}
fn max_target_width(target_len: usize) -> usize {
    let max_width = MAX_MODULE_WIDTH.load(Ordering::Relaxed);
    if max_width < target_len {
        MAX_MODULE_WIDTH.store(target_len, Ordering::Relaxed);
        target_len
    } else {
        max_width
    }
}

fn colored_level(style: &mut Style, level: Level, short_levels: bool) -> StyledValue<&'static str> {
    let (color, msg) = match (level, short_levels) {
        (Level::Trace, false) => (Color::Magenta, "TRACE"),
        (Level::Trace, true) => (Color::Magenta, "TRC"),
        (Level::Debug, false) => (Color::Blue, "DEBUG"),
        (Level::Debug, true) => (Color::Blue, "DBG"),
        (Level::Info, false) => (Color::Green, "INFO "),
        (Level::Info, true) => (Color::Green, "INF"),
        (Level::Warn, false) => (Color::Yellow, "WARN "),
        (Level::Warn, true) => (Color::Yellow, "WRN"),
        (Level::Error, false) => (Color::Red, "ERROR"),
        (Level::Error, true) => (Color::Red, "ERR"),
    };
    style.set_color(color).value(msg)
}

#[cfg(feature = "reltime")]
enum RelTime {
    Diff(u32),
    DateTime(DateTime<Local>),
}

#[cfg(feature = "reltime")]
impl RelTime {
    #[inline]
    fn is_delta(&self) -> bool {
        matches!(self, Self::Diff(_))
    }
}
#[cfg(feature = "reltime")]
impl fmt::Display for RelTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Diff(diff) => {
                write!(f, "[  +0.{diff:0>9}]")
            }
            Self::DateTime(dt) => {
                write!(f, "[{}]", dt.format("%b%e %T"))
            }
        }
    }
}

#[cfg(feature = "reltime")]
fn compute_reltime(last_time: &Arc<Mutex<DateTime<Local>>>) -> RelTime {
    let now = Local::now();
    let mut old = last_time.lock().unwrap();
    let old_date = old.date_naive();
    let old_time = old.time();
    let now_date = now.date_naive();
    let now_time = now.time();
    let reltime = if old_date == now_date
        && old_time.hour() == now_time.hour()
        && old_time.minute() == now_time.minute()
        && old_time.second() == now_time.second()
    {
        let diff: u32 = now_time.nanosecond() - old_time.nanosecond();
        RelTime::Diff(diff)
    } else {
        RelTime::DateTime(now)
    };
    *old = now;
    reltime
}
