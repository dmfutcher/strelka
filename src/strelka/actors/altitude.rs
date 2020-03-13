use actix::prelude::{Actor, Context};

use crate::strelka::actors::{StreamActor, StreamResponse};
use crate::strelka::streams::StreamUpdate;

pub struct AltitudeActor {}

impl Actor for AltitudeActor {
    type Context = Context<Self>;
}

impl StreamActor for AltitudeActor {

    fn request_streams(&self) -> Vec<String> {
        vec!("Altitude".to_owned())
    }

    fn receive(&mut self, msg: StreamUpdate) -> StreamResponse {
        match msg {
            StreamUpdate::Altitude(v) => {
                println!("AltitudeActor: {}m", v);


            },
            _ => {}
        }

        StreamResponse::Ok
    }
}

