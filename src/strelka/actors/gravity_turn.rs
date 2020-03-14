use actix::prelude::{Actor, Addr, Context};

use crate::strelka::actors::{StreamActor, StreamResponse};
use crate::strelka::actors::command::{Command, CommandActor};
use crate::strelka::streams::StreamUpdate;

pub struct GravityTurnActor {
    cmd: Addr<CommandActor>,

    started: bool,
    desired_pitch: f64,
}

impl Actor for GravityTurnActor {
    type Context = Context<Self>;
}

impl GravityTurnActor {

    pub fn new(cmd: Addr<CommandActor>) -> Self {
        GravityTurnActor{ cmd, started: false, desired_pitch: 40.0 }
    }

}

impl StreamActor for GravityTurnActor {

    fn name(&self) -> &'static str { "Gravity turn" }

    fn request_streams(&self) -> Vec<&'static str> {
        vec!("Pitch", "Altitude")
    }

    fn receive(&mut self, update: StreamUpdate) -> StreamResponse {
        match update {
            StreamUpdate::Altitude(altitude) => {
                if altitude > 250.0 && !self.started {
                    self.started = true;
                    info!("Gravity turn started");
                }
            },
            StreamUpdate::Pitch(current_pitch) => {
                if self.started {
                    let within_low = 0.9 * self.desired_pitch;
                    let within_high = 1.1 * self.desired_pitch;
                    if current_pitch >= within_low && current_pitch <= within_high {
                        info!("Gravity turn finished");
                        self.cmd.do_send(Command::SetPitch(0.0));
                        return StreamResponse::Stop;
                    }
        
                    // TODO: Implement gradual pitch control level increasing over time to reduce RUD changes
                    if current_pitch < self.desired_pitch {
                        self.cmd.do_send(Command::SetPitch(1.0));
                    } else if current_pitch > self.desired_pitch {
                        self.cmd.do_send(Command::SetPitch(-1.0));
                    }
                }
            },
            _ => {}
        }

        StreamResponse::Ok
    }

}