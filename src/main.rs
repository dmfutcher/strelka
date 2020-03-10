extern crate krpc_mars;
extern crate failure;
extern crate actix;
extern crate actix_rt;
extern crate strum;
extern crate strum_macros;

mod krpc;
mod strelka;

use strelka::launch_controller;

fn main() -> Result<(), failure::Error> {
    let launch_controller = launch_controller::LaunchController::new()?;

    Ok(())
}