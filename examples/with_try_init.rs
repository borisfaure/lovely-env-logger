extern crate lovely_env_logger;
#[macro_use]
extern crate log;

mod one {
    pub fn deep() {
        trace!("one level deep!");
    }
}

fn main() {
    if let Err(e) = lovely_env_logger::try_init_default() {
        eprintln!("Some custom msg {}", e);
        panic!("error!") // or whatever
    };

    info!("such information");
    warn!("o_O");
    error!("boom");
    debug!("some nice message to help debugging");
    self::one::deep();
}
