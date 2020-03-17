extern crate krpc_mars;
#[macro_use] extern crate failure;
extern crate actix;
extern crate actix_rt;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[allow(dead_code)]
mod krpc;
mod strelka;

use strelka::launch_controller;

#[actix_rt::main]
async fn main() {
    pretty_env_logger::init();

    match launch_controller::LaunchController::new() {
        Ok(mut ctl) => ctl.start_launch().await,
        Err(_) => error!("Failed to start launch")
    };
}
