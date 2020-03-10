use actix::prelude::{Actor, Context};

use crate::strelka::actors::StreamActor;
use crate::strelka::streams::StreamUpdate;

pub struct AltitudeMonitor {}

impl Actor for AltitudeMonitor {
    type Context = Context<Self>;
}

impl StreamActor for AltitudeMonitor {

    fn request_streams(&self) -> Vec<String> {
        vec!("Altitude".to_owned())
    }

    fn receive(&mut self, msg: StreamUpdate) {
        match msg {
            StreamUpdate::Altitude(v) => {
                println!("AltitudeMonitor: {}m", v);
            },
            _ => {}
        }
    }
}

