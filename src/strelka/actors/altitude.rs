use crate::strelka::actors::StreamActor;
use crate::strelka::streams::StreamUpdate;
use actix::prelude::{Actor, Context};


pub struct AltitudeMonitor {}

impl StreamActor for AltitudeMonitor {
    fn request_streams(&self) -> Vec<String> {
        vec!("Altitude".to_owned())
    }

    fn receive(&self, msg: StreamUpdate) {
        match msg {
            StreamUpdate::Altitude(v) => {
                println!("AltitudeMonitor: {}m", v);
            }
        }
    }
}

impl Actor for AltitudeMonitor {
    type Context = Context<Self>;
}