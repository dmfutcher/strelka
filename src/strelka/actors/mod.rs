pub mod altitude;
pub mod command;
pub mod controller;
pub mod ignition;
pub mod streamer;

use actix::prelude::*;
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
