pub mod altitude;
pub mod command_control;
pub mod streamer;
pub mod ignition;

use actix::prelude::*;
use std::collections::HashMap;
use std::thread;
use std::time;

use altitude::AltitudeMonitor;
use command_control::CommandAndControl;
use ignition::IgnitionActor;
use streamer::{Streamer, StreamValues};
use crate::strelka::streams::StreamUpdate;


impl Message for StreamUpdate {
    type Result = ();
}

pub trait StreamActor {
    fn request_streams(&self) -> Vec<String>;
    fn receive(&mut self, msg: StreamUpdate);
}

impl Actor for Box<dyn StreamActor> {
    type Context = Context<Self>;
}

impl Handler<StreamUpdate> for Box<dyn StreamActor> {
    type Result = ();

    fn handle(&mut self, msg: StreamUpdate, _: &mut Context<Self>) -> Self::Result {
        (*self).receive(msg);
    }
}

pub struct ActorController {
    actors: Vec<actix::Addr<Box<dyn StreamActor>>>,
    stream_map: HashMap<String, Vec<actix::Addr<Box<dyn StreamActor>>>>,
    command_actor: actix::Addr<CommandAndControl>,
    stream_actor: actix::Addr<Streamer>,
}

impl ActorController {
    pub fn new(krpc_client: krpc_mars::RPCClient, stream_client: krpc_mars::StreamClient) -> Self {
        ActorController{ 
            actors: vec!(),
            stream_map: HashMap::new(),
            command_actor: CommandAndControl::new(krpc_client.clone()).start(),
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
    }

}


