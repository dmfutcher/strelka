use actix::prelude::{Actor, Context};

use crate::strelka::actors::{StreamActor, StreamResponse};
use crate::strelka::streams::StreamUpdate;

pub struct AltitudeActor {}

impl Actor for AltitudeActor {
    type Context = Context<Self>;
}

impl StreamActor for AltitudeActor {

    fn name(&self) -> &'static str { "Altitude" }

    fn request_streams(&self) -> Vec<&'static str> {
        vec!("Altitude")
    }

    fn receive(&mut self, _: StreamUpdate) -> StreamResponse {
        // TODO: Currently does nothing
        StreamResponse::Ok
    }
}

