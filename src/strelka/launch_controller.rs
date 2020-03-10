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
    stream_client: krpc_mars::StreamClient,
    abort: bool,
    actor_ctl: ActorController,
    vessel: Option<space_center::Vessel>,
    control: Option<space_center::Control>,
}

impl LaunchController {

    pub fn new() -> Result<LaunchController, failure::Error> {
        let client = krpc_mars::RPCClient::connect("Example", "127.0.0.1:50000")?;
        let stream_client = krpc_mars::StreamClient::connect(&client, "127.0.0.1:50001")?;
        let actor_ctl = ActorController::new(client.clone(), stream_client.clone());

        Ok(LaunchController{ 
            client,
            stream_client,
            abort: false,
            actor_ctl: actor_ctl,
            vessel: None,
            control: None,
        })
    }

    // TODO: Before getting too deep into talking to krpc, need to work out an error handling scheme other than panic! everywhere
    fn get_active_vessel(&self) -> Result<space_center::Vessel, krpc_mars::error::Error> {
        self.client.mk_call(&space_center::get_active_vessel())
    }

    fn get_vessel_control(&self) -> Result<space_center::Control, krpc_mars::error::Error> {
        if let Some(vessel) = &self.vessel {
            self.client.mk_call(&vessel.get_control())
        } else {
            panic!("No vessel, could not get control");
        }
    }

    pub async fn start_launch(&mut self) {
        match self.get_active_vessel() {
            Ok(vessel) => self.vessel = Some(vessel),
            Err(_) => panic!("Failed to get vessel from kRPC")
        }

        match self.get_vessel_control() {
            Ok(ctl) => self.control = Some(ctl),
            Err(_) => panic!("Failed to get control from vessel")
        }

        self.actor_ctl.start().await;
        self.launch_loop().await;
    }

    async fn launch_loop(&mut self) {
        while !self.abort {
            self.actor_ctl.tick().await;
        }
    }

}
