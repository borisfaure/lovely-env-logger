use std::{thread, time};
extern crate lovely_env_logger;
#[macro_use]
extern crate log;

mod nested {
    pub fn deep() {
        trace!("one level deep!");
    }
}
mod longer_nested {
    pub fn deep() {
        trace!("one level deep, but longer!");
    }
}

fn main() {
    lovely_env_logger::init_default();

    if !log_enabled!(log::Level::Trace) {
        eprintln!("To see the full demo, try setting `RUST_LOG=log=trace`.");
        return;
    }

    debug!("deboogging");
    thread::sleep(time::Duration::from_millis(100));
    info!("such information");
    thread::sleep(time::Duration::from_millis(357));
    warn!("o_O");
    thread::sleep(time::Duration::from_millis(400));
    error!("boom");
    thread::sleep(time::Duration::from_millis(100));
    self::nested::deep();
    thread::sleep(time::Duration::from_millis(900));
    debug!("deboogging");
    thread::sleep(time::Duration::from_millis(100));
    info!("such information");
    thread::sleep(time::Duration::from_millis(100));
    warn!("o_O");
    thread::sleep(time::Duration::from_millis(100));
    error!("boom");
    thread::sleep(time::Duration::from_millis(100));
    self::longer_nested::deep();
    thread::sleep(time::Duration::from_millis(100));
    debug!("deboogging");
    thread::sleep(time::Duration::from_millis(100));
    info!("such information");
    thread::sleep(time::Duration::from_millis(100));
    warn!("o_O");
    thread::sleep(time::Duration::from_millis(2300));
    error!("boom");
}
