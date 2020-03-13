use actix::prelude::*;

use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use  std::boxed::Box;
use std::any::Any;

use crate::strelka::actors::{StreamActor, StreamUpdate};
use crate::strelka::actors::command::CommandActor;
use crate::strelka::actors::streamer::{Streamer, StreamValues};
use crate::strelka::actors::altitude::AltitudeActor;
use crate::strelka::actors::ignition::IgnitionActor;

pub struct ActorController {
    actors: Vec<actix::Addr<Box<dyn StreamActor>>>,
    stream_map: HashMap<String, Vec<actix::Addr<Box<dyn StreamActor>>>>,
    cmd_actor: actix::Addr<CommandActor>,
    stream_actor: actix::Addr<Streamer>,
}

// Ideally we'd put all the logic in here into its own Actor inside the system; however there are
// complications with calling async fns in actix message handlers, so do it here for now.
impl ActorController {

    pub fn new(krpc_client: krpc_mars::RPCClient, stream_client: krpc_mars::StreamClient) -> Self {
        ActorController{ 
            actors: vec!(),
            stream_map: HashMap::new(),
            cmd_actor: CommandActor::new(krpc_client.clone()).start(),
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

    pub async fn broadcast_stream_update(&self, update: StreamUpdate) {
        match self.stream_map.get(&update.to_string()) {
            Some(actors) => {
                for a in actors {
                    (*a).send(update).await;
                }
            },
            None => {}
        }
    }

    pub async fn tick(&self) {
        let result = self.stream_actor.send(StreamValues{}).await;
        println!("{:?}", result);
        if let Ok(updated_values) = result {
            for update in updated_values {
                self.broadcast_stream_update(*update).await;
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    pub async fn start(&mut self) {
        self.register_actor(Box::new(AltitudeActor{}));
        self.register_actor(Box::new(IgnitionActor::new(self.cmd_actor.clone())));
    }
    
    pub async fn stop_actors(&self) {
        System::current().stop();
    }

}
