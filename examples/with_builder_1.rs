extern crate env_logger;
extern crate lovely_env_logger;
#[macro_use]
extern crate log;

use env_logger::Target;

mod one {
    pub fn deep() {
        trace!("one level deep!");
        trace!("one level deep!");
    }
}

fn main() {
    lovely_env_logger::formatted_builder(lovely_env_logger::Config::default())
        //let's just set some random stuff.. for more see
        // https://docs.rs/env_logger/0.10.0/env_logger/struct.Builder.html
        .target(Target::Stdout)
        .parse_filters("with_builder_1=trace")
        .init();

    info!("such information");
    info!("such information again");
    warn!("o_O");
    warn!("O_o");
    error!("boom");
    error!("boom");
    debug!("some nice message to help debugging");
    debug!("or is it too late?");
    self::one::deep();
}
