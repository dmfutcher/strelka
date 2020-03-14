use actix::prelude::{Actor, Context};

use crate::strelka::actors::{StreamActor, StreamResponse};
use crate::strelka::streams::StreamUpdate;

pub struct AltitudeActor {
    alt_start: Option<f64>,
    liftoff_confirm: bool,
}

impl AltitudeActor {

    pub fn new() -> Self {
        AltitudeActor{ 
            alt_start: None,
            liftoff_confirm: false
        }
    }

}

impl Actor for AltitudeActor {
    type Context = Context<Self>;
}

impl StreamActor for AltitudeActor {

    fn name(&self) -> &'static str { "Altitude" }

    fn request_streams(&self) -> Vec<&'static str> {
        vec!("Altitude")
    }

    fn receive(&mut self, update: StreamUpdate) -> StreamResponse {
        match update {
            StreamUpdate::Altitude(altitude) => {
                if self.alt_start.is_none() {
                    self.alt_start = Some(altitude);
                }

                if !self.liftoff_confirm {
                    if let Some(start_altitude) = self.alt_start {
                        if (altitude - start_altitude) > 10.0 {
                            self.liftoff_confirm = true;
                            info!("Vehicle has cleared the launchpad")
                        }
                    }
                }
            },
            _ => {}
        }


        StreamResponse::Ok
    }
}

