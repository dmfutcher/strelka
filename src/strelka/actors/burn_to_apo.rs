use actix::prelude::{Actor, Addr, Context};

use crate::strelka::actors::{StreamActor, StreamResponse};
use crate::strelka::actors::command::{Command, CommandActor};
use crate::strelka::streams::StreamUpdate;

pub struct BurnToApoActor {
    cmd: Addr<CommandActor>,

    target_apo_alt: f64,
}

impl Actor for BurnToApoActor {
    type Context = Context<Self>;
}

impl BurnToApoActor {

    pub fn new(cmd: Addr<CommandActor>) -> Self {
        BurnToApoActor{ cmd, target_apo_alt: 75_000.0 }
    }

}

impl StreamActor for BurnToApoActor {

    fn name(&self) -> &'static str { "Burn to apoapsis" }

    fn request_streams(&self) -> Vec<&'static str> {
        vec!("Apoapsis")
    }

    fn receive(&mut self, update: StreamUpdate) -> StreamResponse {
        match update {
            StreamUpdate::Apoapsis(apo) => {
                if apo >= self.target_apo_alt {
                    info!("Apoapsis at target altitude");
                    self.cmd.do_send(Command::SetThrottle(0.0));
                    info!("MECO confirmed");

                    // TODO: Start working out circularisation burn
                }
            },
            _ => {}
        }

        StreamResponse::Ok
    }

}
