use actix::prelude::{Actor, Addr, Context};
use crate::krpc::space_center;

use crate::strelka::actors::{StreamActor, StreamResponse};
use crate::strelka::actors::command::{Command, CommandActor};
use crate::strelka::streams::StreamUpdate;

enum BurnPhase {
    PreBurn,
    Burning,
}

pub struct CircularisationBurnActor {
    cmd: Addr<CommandActor>,

    burn_calculated: bool,
    oriented: bool,
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
            oriented: false,
            phase: BurnPhase::PreBurn
        }
    }

}

impl StreamActor for CircularisationBurnActor {

    fn name(&self) -> &'static str { "Circularisation Burn" }

    fn request_streams(&self) -> Vec<&'static str> {
        vec!("TimeToApoapsis", "Apoapsis", "Altitude", "ManoeuvreInfo")
    }

    fn receive(&mut self, update: StreamUpdate) -> StreamResponse {
        match self.phase{
            BurnPhase::PreBurn => {
                if !self.burn_calculated {
                    if let StreamUpdate::Apoapsis(apo_alt) = update {
                        info!("Calculating circularisation burn");
                        self.cmd.do_send(Command::CreateOrbitNode(apo_alt));
                        self.burn_calculated = true;
                    }
                } else {
                    if !self.oriented {
                        if let StreamUpdate::Altitude(altitude) = update {
                            if altitude > 70_000.0 { // TODO: Hardcoded kerbin atmosphere altitude
                                info!("Vessel is out of Kerbin atmosphere");
                                info!("Orienting for circularisation burn");
                                self.cmd.do_send(Command::SetSASMode(space_center::SASMode::Maneuver));
                                
                                self.oriented = true;
                            }
                        }
                    }

                    if let StreamUpdate::ManoeuvreInfo(info) = update {
                        if info.time_to <= (info.burn_time as f64 / 2.0) {
                            info!("Starting circularisation burn");
                            self.cmd.do_send(Command::SetThrottle(1.0));
                            self.phase = BurnPhase::Burning;
                        }
                    }
                }
            },
            BurnPhase::Burning => {
                if let StreamUpdate::ManoeuvreInfo(info) = update {
                    // TODO: Remaining dV is never 0, it will increase as you overshoot past your mark
                    //       We could use the burn_time here if the calculation worked better. 
                    //       Could try using apo/peri alt values but they behave oddly as you reach orbit.
                    if info.remaining_delta_v <= 10.0 {
                        info!("Ending circularisation burn");
                        self.cmd.do_send(Command::SetThrottle(0.0));
                        return StreamResponse::Stop;
                    }
                }
            },
        }

        StreamResponse::Ok
    }

}
