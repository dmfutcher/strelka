pub mod altitude;
pub mod burn_to_apo;
pub mod circularisation_burn;
pub mod command;
pub mod controller;
pub mod gravity_turn;
pub mod ignition;
pub mod stager;
pub mod streamer;
pub mod spawner;

use actix::prelude::*;
use crate::strelka::streams::StreamUpdate;

impl Message for StreamUpdate {
    type Result = ();
}

pub enum StreamResponse {
    Ok,
    Stop,
}

pub trait StreamActor: Send {
    fn name(&self) -> &'static str;
    fn request_streams(&self) -> Vec<&'static str>;
    fn receive(&mut self, msg: StreamUpdate) -> StreamResponse;
    fn on_start(&self) {}
}

impl Actor for Box<dyn StreamActor> {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) {
        self.on_start();
    }
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

// unsafe impl Send for StreamActor {}