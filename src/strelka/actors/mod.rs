pub mod altitude;
pub mod command_control;

use actix::prelude::*;
use std::collections::HashMap;

use altitude::AltitudeMonitor;
use command_control::CommandAndControl;
use crate::strelka::streams::StreamUpdate;


impl Message for StreamUpdate {
    type Result = ();
}

pub trait StreamActor {
    fn request_streams(&self) -> Vec<String>;
    fn receive(&self, msg: StreamUpdate);
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
}

impl ActorController {
    pub fn new() -> Self {
        ActorController{ 
            actors: vec!(),
            stream_map: HashMap::new(),
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

    pub async fn start(&mut self) {
        let candc = CommandAndControl{ };
        let alt = AltitudeMonitor{ };
        self.register_actor(Box::new(candc));
        self.register_actor(Box::new(alt));
    
        self.broadcast(StreamUpdate::Altitude(100.0)).await;
    }
    
    pub async fn stop_actors(&self) {
        System::current().stop();
    }

}


