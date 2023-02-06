extern crate lovely_env_logger;
#[macro_use]
extern crate log;

use std::env;

mod one {
    pub fn deep() {
        trace!("one level deep!");
        trace!("one level deep!");
    }
}

fn main() {
    env::set_var("RUST_APP_LOG", "trace");

    lovely_env_logger::init_custom_env(lovely_env_logger::Config::default(), "RUST_APP_LOG");

    info!("such information");
    info!("such information again");
    warn!("o_O");
    warn!("O_o");
    error!("boom");
    error!("boom");
    debug!("some nice message to help debugging");
    debug!("or is it too late?");
    self::one::deep();

    env::remove_var("RUST_APP_LOG");
}
