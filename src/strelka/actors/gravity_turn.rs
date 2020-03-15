use actix::prelude::{Actor, Addr, Context};

use crate::strelka::actors::{StreamActor, StreamResponse};
use crate::strelka::actors::command::{Command, CommandActor};
use crate::strelka::actors::spawner::{Spawner, SpawnerCommand};
use crate::strelka::actors::burn_to_apo::BurnToApoActor;
use crate::strelka::streams::StreamUpdate;

pub struct GravityTurnActor {
    cmd: Addr<CommandActor>,
    spawn: Addr<Spawner>,

    started: bool,
    finished: bool,
    desired_pitch: f64,
}

impl Actor for GravityTurnActor {
    type Context = Context<Self>;
}

impl GravityTurnActor {

    pub fn new(cmd: Addr<CommandActor>, spawn: Addr<Spawner>) -> Self {
        GravityTurnActor{ 
            cmd,
            spawn,
            started: false, 
            finished: false,
            desired_pitch: 55.0
        }
    }

}

impl StreamActor for GravityTurnActor {

    fn name(&self) -> &'static str { "Gravity Turn" }

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
            // TODO: Do we want to base this off prograde vector pitch rather than vessel pitch?
            // TODO: Should we avoid any pitch commands when going trans-sonic? OR more generally when turning
            //          is more likely to lose control (atmospheric drag)
            StreamUpdate::Pitch(current_pitch) => {
                if self.started {
                    let within_low = 0.9 * self.desired_pitch;
                    let within_high = 1.1 * self.desired_pitch;

                    if current_pitch >= within_low && current_pitch <= within_high && !self.finished {
                        self.finished = true;
                        info!("Gravity turn finished");
                        self.cmd.do_send(Command::SetPitch(0.0));

                        // Spawn the BurnToApo actor and stop our own actor
                        self.spawn.do_send(SpawnerCommand::Spawn(Box::new(BurnToApoActor::new(self.cmd.clone(), self.spawn.clone()))));
                        return StreamResponse::Stop;
                    }
        
                    // TODO: Implement gradual pitch control level increasing over time to reduce chance of losing control
                    // TODO: ^^ will also be useful as we have to tweak these values for different launch vehicles
                    if current_pitch < self.desired_pitch {
                        self.cmd.do_send(Command::SetPitch(1.0));
                    } else if current_pitch > self.desired_pitch {
                        self.cmd.do_send(Command::SetPitch(1.0));
                    }
                }
            },
            _ => {}
        }

        StreamResponse::Ok
    }

}
