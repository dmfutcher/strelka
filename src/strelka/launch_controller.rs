use crate::strelka::actors::controller::ActorController;

// TODO: Currently un-used ... might use it later
// pub enum LaunchPhase {
//     PreIgnition,
//     Ignition,
//     InitialClimb,
//     GravityTurn,
//     BoostToOrbit,
//     Circularize
// }

pub struct LaunchController {
    abort: bool,
    actor_ctl: ActorController,
}

impl LaunchController {

    pub fn new() -> Result<LaunchController, failure::Error> {
        let client = krpc_mars::RPCClient::connect("Example", "127.0.0.1:50000")?;
        let stream_client = krpc_mars::StreamClient::connect(&client, "127.0.0.1:50001")?;
        let actor_ctl = ActorController::new(client.clone(), stream_client.clone());

        Ok(LaunchController{ 
            abort: false,
            actor_ctl: actor_ctl,
        })
    }

    pub async fn start_launch(&mut self) {
        self.actor_ctl.start().await;
        self.launch_loop().await;
    }

    async fn launch_loop(&mut self) {
        while !self.abort {
            self.actor_ctl.tick().await;
        }
    }

}
