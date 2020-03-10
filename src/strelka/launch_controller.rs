use crate::krpc::space_center;
use crate::strelka::actor;

pub enum LaunchPhase {
    PreIgnition,
    Ignition,
    InitialClimb,
    GravityTurn,
    BoostToOrbit,
    Circularize
}

pub struct LaunchController {
    client: krpc_mars::RPCClient,
    vessel: Option<space_center::Vessel>,
    control: Option<space_center::Control>,
}

impl LaunchController {

    pub fn new() -> Result<LaunchController, failure::Error> {
        let client = krpc_mars::RPCClient::connect("Example", "127.0.0.1:50000")?;

        actor::run_actors();

        Ok(LaunchController{ 
            client: client,
            vessel: None,
            control: None,
        })

    }

    fn get_active_vessel(&self) -> Result<space_center::Vessel, failure::Error> {
        let vessel = self.client.mk_call(&space_center::get_active_vessel())?;
        Ok(vessel)
    }

}


// let vessel = client.mk_call(&space_center::get_active_vessel())?;
// println!("Active vessel: {:?}", vessel);

// let control = client.mk_call(&vessel.get_control())?;

// let result = client.mk_call(&control.activate_next_stage())?;
// println!("{:?}", result);