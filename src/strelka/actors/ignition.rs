use actix::prelude::{Actor, Addr, Context};

use crate::strelka::actors::StreamActor;
use crate::strelka::actors::command::{Command, CommandActor};
use crate::strelka::streams::StreamUpdate;

pub struct IgnitionActor {
    countdown_val: i8,
    // TODO: Needs refactored once prototype is proved
    cnc: Addr<CommandActor>,
}

impl IgnitionActor {
    pub fn new(cnc: Addr<CommandActor>) -> Self {
        IgnitionActor{ countdown_val: 10, cnc }
    }

    fn ignite(&self) {
        println!("Ignition!");
        self.cnc.do_send(Command::Stage);
    }
}

impl Actor for IgnitionActor {
    type Context = Context<Self>;
}

impl StreamActor for IgnitionActor {

    fn request_streams(&self) -> Vec<String> {
        // Subscribe to UniversalTime so receive() will be called at 60hz, even though we don't use the value ...
        // TODO: Timer based actors?
        vec!("ut".to_owned())
    }

    fn receive(&mut self, _: StreamUpdate) {
        if self.countdown_val > 0 {
            println!("{}", self.countdown_val);
            self.countdown_val -= 1;
        } else if self.countdown_val == 0 {
            self.ignite();
            self.countdown_val -= 1;
            // TODO: How do we stop StreamActors?
        }
    }
}

