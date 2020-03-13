use actix::prelude::{Actor, Context};

use crate::strelka::actors::StreamActor;
use crate::strelka::streams::StreamUpdate;

pub struct PitchOverActor {

}

impl Actor for PitchOverActor {
    type Context = Context<Self>;
}

impl StreamActor for PitchOverActor {

    fn request_streams(&self) -> Vec<String> {
        vec!("ut".to_owned()) // Tick at 60hz
    }

    fn receive(&mut self, _: StreamUpdate) {

    }
}

