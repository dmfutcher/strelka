pub mod altitude;
pub mod command;
pub mod streamer;
pub mod ignition;
pub mod control;
pub mod timer;

use actix::prelude::*;
// use std::collections::HashMap;
// use std::thread;
// use std::time;

// use altitude::AltitudeMonitor;
// use command::CommandActor;
// use ignition::IgnitionActor;
// use streamer::{Streamer, StreamValues};
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

// pub struct ActorController {
//     actors: Vec<actix::Addr<Box<dyn StreamActor>>>,
//     stream_map: HashMap<String, Vec<actix::Addr<Box<dyn StreamActor>>>>,
//     command_actor: actix::Addr<CommandActor>,
//     stream_actor: actix::Addr<Streamer>,
// }
