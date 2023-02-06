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

    debug!("some nice message to help debugging");
    info!("such information");
    warn!("o_O");
    error!("boom");
    self::nested::deep();
    debug!("some nice message to help debugging");
    info!("such information");
    warn!("o_O");
    error!("boom");
    self::longer_nested::deep();
    debug!("some nice message to help debugging");
    info!("such information");
    warn!("o_O");
    error!("boom");
}
