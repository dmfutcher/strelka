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

    burn_calculated: bool,
    phase: BurnPhase,
}

impl Actor for CircularisationBurnActor {
    type Context = Context<Self>;
}

impl CircularisationBurnActor {

    pub fn new(cmd: Addr<CommandActor>) -> Self {
        CircularisationBurnActor{ 
            cmd, 
            burn_calculated: false,
            phase: BurnPhase::PreBurn
        }
    }

}

impl StreamActor for CircularisationBurnActor {

    fn name(&self) -> &'static str { "Circularisation Burn" }

    fn request_streams(&self) -> Vec<&'static str> {
        vec!("TimeToApoapsis", "Apoapsis")
    }

    fn receive(&mut self, update: StreamUpdate) -> StreamResponse {
        match self.phase{
            BurnPhase::PreBurn => {
                if !self.burn_calculated {
                    if let StreamUpdate::Apoapsis(apo_alt) = update {
                        info!("Calculating circularisation burn");
                        self.cmd.do_send(Command::CreateOrbitNode(apo_alt));
                        self.burn_calculated = true;
                        info!("Waiting until circularisation manouevre");
                    }
                }
            },
            BurnPhase::Burning => {

            },
            BurnPhase::PostBurn => {

            },
        }

        StreamResponse::Ok
    }

}
