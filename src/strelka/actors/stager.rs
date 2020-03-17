use actix::prelude::{Actor, Addr, Context};

use crate::strelka::actors::{StreamActor, StreamResponse};
use crate::strelka::actors::command::{Command, CommandActor};
use crate::strelka::streams::StreamUpdate;

pub struct StageActor {
    cmd: Addr<CommandActor>,
}

impl Actor for StageActor {
    type Context = Context<Self>;
}

impl StageActor {

    pub fn new(cmd: Addr<CommandActor>) -> Self {
        StageActor{ 
            cmd, 
        }
    }

}

impl StreamActor for StageActor {

    fn name(&self) -> &'static str { "Staging monitor" }

    fn request_streams(&self) -> Vec<&'static str> {
        vec!("EnginesDepleted")
    }

    fn receive(&mut self, update: StreamUpdate) -> StreamResponse {
        if let StreamUpdate::EnginesDepleted(count) = update {
            if count > 0 {
                self.cmd.do_send(Command::Stage);
            }
        }

        StreamResponse::Ok
    }

}
