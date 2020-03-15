use actix::prelude::{Actor, Addr, Context};

use crate::strelka::actors::{StreamActor, StreamResponse};
use crate::strelka::actors::command::{Command, CommandActor};
use crate::strelka::streams::StreamUpdate;

enum BurnPhase {
    PreBurn,
    Burning,
    PostBurn,
}

pub struct CircularisationBurnActor {
    cmd: Addr<CommandActor>,

    phase: BurnPhase,
}

impl Actor for CircularisationBurnActor {
    type Context = Context<Self>;
}

impl CircularisationBurnActor {

    pub fn new(cmd: Addr<CommandActor>) -> Self {
        CircularisationBurnActor{ cmd, phase: BurnPhase::PreBurn }
    }

}

impl StreamActor for CircularisationBurnActor {

    fn name(&self) -> &'static str { "Circularisation Burn" }

    fn request_streams(&self) -> Vec<&'static str> {
        vec!("Apoapsis")
    }

    fn receive(&mut self, update: StreamUpdate) -> StreamResponse {
        match self.phase{
            BurnPhase::PreBurn => {
                info!("Calculating circularisation burn");
            },
            BurnPhase::Burning => {

            },
            BurnPhase::PostBurn => {

            },
        }

        StreamResponse::Ok
    }

}
