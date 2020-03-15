pub mod altitude;
pub mod burn_to_apo;
pub mod command;
pub mod controller;
pub mod gravity_turn;
pub mod ignition;
pub mod streamer;

use actix::prelude::*;
use crate::strelka::streams::StreamUpdate;

impl Message for StreamUpdate {
    type Result = ();
}

pub enum StreamResponse {
    Ok,
    Stop,
}

pub trait StreamActor {
    fn name(&self) -> &'static str;
    fn request_streams(&self) -> Vec<&'static str>;
    fn receive(&mut self, msg: StreamUpdate) -> StreamResponse;
}

impl Actor for Box<dyn StreamActor> {
    type Context = Context<Self>;
}

impl Handler<StreamUpdate> for Box<dyn StreamActor> {
    type Result = ();

    fn handle(&mut self, msg: StreamUpdate, ctx: &mut Context<Self>) -> Self::Result {
        match (*self).receive(msg) {
            StreamResponse::Stop => {
                info!("{} program complete", self.name());
                ctx.stop();
            },
            _ => {}
        };
    }
}
