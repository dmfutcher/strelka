extern crate krpc_mars;
extern crate failure;
extern crate actix;
extern crate actix_rt;
extern crate strum;
extern crate strum_macros;

#[allow(dead_code)]
mod krpc;
mod strelka;

use strelka::launch_controller;

fn main() {

    match launch_controller::LaunchController::new() {
        Ok(mut ctl) => ctl.start_launch(),
        Err(e) => println!("Failed: {:?}", e) // TODO: Improve error handling here
    };

}