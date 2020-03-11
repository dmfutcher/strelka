use std::time::Duration;
use actix::prelude::*;

use crate::strelka::actors::control::{ControlActor, ControlAction};

pub struct Timer {
    dur: Duration,
    ctl_actor: Addr<ControlActor>,
}

impl Timer {

    pub fn new(ctl_actor: Addr<ControlActor>) -> Self {
        Timer{
            dur: Duration::from_secs(1),
            ctl_actor,
        }
    }

    fn tick(&self) {
        self.ctl_actor.do_send(ControlAction::Tick);
    }

}

impl Actor for Timer {
    type Context = Context<Self>;

    // stop system after `self.dur` seconds
    fn started(&mut self, ctx: &mut Context<Self>) {
        ctx.run_interval(self.dur, |act, ctx| {
            println!("Timer tick");
            act.tick();
            // self.ctl_actor.send(ControlAction::Tick);
        });
    }
}
