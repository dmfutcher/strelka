use actix::prelude::{Actor, Addr, Context};

use crate::strelka::actors::{StreamActor, StreamResponse};
use crate::strelka::actors::command::{Command, CommandActor};
use crate::strelka::streams::StreamUpdate;

pub struct IgnitionActor {
    countdown_val: i8,
    // TODO: Needs refactored once prototype is proved
    cmd: Addr<CommandActor>,
}

impl IgnitionActor {
    pub fn new(cmd: Addr<CommandActor>) -> Self {
        IgnitionActor{ countdown_val: 10, cmd }
    }

    fn ignite(&self) {
        println!("Ignition!");
        self.cmd.do_send(Command::Stage);
    }
}

impl Actor for IgnitionActor {
    type Context = Context<Self>;
}

impl StreamActor for IgnitionActor {

    fn request_streams(&self) -> Vec<String> {
        // Subscribe to UniversalTime so receive() will be called at 60hz, even though we don't use the value ...
        // TODO: Timer based actors?
        vec!("UniversalTime".to_owned())
    }

    fn receive(&mut self, _: StreamUpdate) -> StreamResponse {
        if self.countdown_val > 0 {
            println!("{}", self.countdown_val);
            self.countdown_val -= 1;

            self.cmd.do_send(Command::SetRCS(true));
            self.cmd.do_send(Command::SetSAS(true));
            self.cmd.do_send(Command::SetThrottle(100.0));
        } else if self.countdown_val == 0 {
            self.ignite();
            self.countdown_val -= 1;
            return StreamResponse::Stop;
        }

        return StreamResponse::Ok;
    }
}

