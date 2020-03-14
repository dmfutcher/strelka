use actix::prelude::{Actor, Addr, Context};

use crate::strelka::actors::{StreamActor, StreamResponse};
use crate::strelka::actors::command::{Command, CommandActor};
use crate::strelka::streams::StreamUpdate;

pub struct PitchOverActor {
    cmd: Addr<CommandActor>,

    started: bool,
    desired_pitch: f64,
}

impl Actor for PitchOverActor {
    type Context = Context<Self>;
}

impl PitchOverActor {

    pub fn new(cmd: Addr<CommandActor>) -> Self {
        PitchOverActor{ cmd, started: false, desired_pitch: 45.0 }
    }

}

impl StreamActor for PitchOverActor {

    fn request_streams(&self) -> Vec<String> {
        vec!("Pitch".to_owned(), "Altitude".to_owned()) // Tick at 60hz
    }

    fn receive(&mut self, update: StreamUpdate) -> StreamResponse {
        match update {
            StreamUpdate::Altitude(altitude) => {
                if altitude > 250.0 && !self.started {
                    self.started = true;
                    println!("Pitchover started");
                }
            },
            StreamUpdate::Pitch(current_pitch) => {
                if self.started {
                    let withinLow = 0.9 * self.desired_pitch;
                    let withinHigh = 1.1 * self.desired_pitch;
                    if current_pitch >= withinLow && current_pitch <= withinHigh {
                        println!("Pitchover finished");
                        self.cmd.do_send(Command::Pitch(0.0));
                        return StreamResponse::Stop;
                    }
        
                    // TODO: Implement gradual pitch control level increasing over time to reduce RUD changes
                    if current_pitch < self.desired_pitch {
                        self.cmd.do_send(Command::Pitch(0.5));
                    } else if current_pitch > self.desired_pitch {
                        self.cmd.do_send(Command::Pitch(-0.5));
                    }
                }
            },
            _ => {}
        }

        StreamResponse::Ok
    }

}
