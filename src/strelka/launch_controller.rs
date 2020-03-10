use crate::krpc::space_center;
use crate::strelka::actors::ActorController;

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
    actor_ctl: ActorController,
    vessel: Option<space_center::Vessel>,
    control: Option<space_center::Control>,
}

impl LaunchController {

    pub fn new() -> Result<LaunchController, failure::Error> {
        let client = krpc_mars::RPCClient::connect("Example", "127.0.0.1:50000")?;
        let actor_ctl = ActorController::new();

        Ok(LaunchController{ 
            client,
            actor_ctl: actor_ctl,
            vessel: None,
            control: None,
        })
    }

    fn get_active_vessel(&self) -> Result<space_center::Vessel, failure::Error> {
        let vessel = self.client.mk_call(&space_center::get_active_vessel())?;
        Ok(vessel)
    }

    pub async fn start_launch(&mut self) {
        self.actor_ctl.start().await;
    }

}


// let vessel = client.mk_call(&space_center::get_active_vessel())?;
// println!("Active vessel: {:?}", vessel);

// let control = client.mk_call(&vessel.get_control())?;

// let result = client.mk_call(&control.activate_next_stage())?;
// println!("{:?}", result);