use crate::krpc::space_center;
use crate::strelka::actors::control::{ControlActor, ControlAction};
use crate::strelka::actors::timer::Timer;
use crate::strelka::actors::StreamActor;

use actix::{System, Actor};

// pub enum LaunchPhase {
//     PreIgnition,
//     Ignition,
//     InitialClimb,
//     GravityTurn,
//     BoostToOrbit,
//     Circularize
// }

pub struct LaunchController {
    client: krpc_mars::RPCClient,
    stream_client: krpc_mars::StreamClient,
    abort: bool,
}

impl LaunchController {

    pub fn new() -> Result<LaunchController, failure::Error> {
        let client = krpc_mars::RPCClient::connect("Example", "127.0.0.1:50000")?;
        let stream_client = krpc_mars::StreamClient::connect(&client, "127.0.0.1:50001")?;

        Ok(LaunchController{ 
            client,
            stream_client,
            abort: false,
        })
    }

    pub fn start_launch(&mut self) {
        let sys = System::new("test");
        let ctl_actor = ControlActor::new(self.client.clone()).start();

        Timer::new(ctl_actor.clone()).start();

        sys.run(); // TODO: Return this result
    }

}


/* 

    pub fn new(krpc_client: krpc_mars::RPCClient, stream_client: krpc_mars::StreamClient) -> Self {
        ActorController{ 
            actors: vec!(),
            stream_map: HashMap::new(),
            command_actor: CommandActor::new(krpc_client.clone()).start(),
            stream_actor: Streamer::new(krpc_client, stream_client).start(),
        }
    }

    pub fn register_actor(&mut self, actor: Box<dyn StreamActor>) {
        let linked_streams = actor.request_streams();
        let address = actor.start();
        self.actors.push(address.clone());

        for stream_name in linked_streams {
            let addrs = self.stream_map.entry(stream_name).or_insert(vec!());
            addrs.push(address.clone());
        }
    }

    pub async fn broadcast(&self, update: StreamUpdate) {
        match self.stream_map.get(&update.to_string()) {
            Some(actors) => {
                for a in actors {
                    (*a).send(update).await;
                }
            },
            None => {}
        }
    }

    // TODO: All of this stuff should probably live in its own Actor instead of the main thread
    pub async fn tick(&self) {
        let result = self.stream_actor.send(StreamValues{}).await;
        if let Ok(updated_values) = result {
            for update in updated_values {
                self.broadcast(update).await;
            }
        }

        thread::sleep(time::Duration::from_secs(1));
    }

    pub async fn start(&mut self) {
        self.register_actor(Box::new(AltitudeMonitor{}));
        self.register_actor(Box::new(IgnitionActor::new(self.command_actor.clone())));

        println!("{:?}", self.stream_map.keys());
    }
    
    pub async fn stop_actors(&self) {
        System::current().stop();
    }*/