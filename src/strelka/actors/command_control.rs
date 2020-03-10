use actix::prelude::{Actor, Context, Message};
use crate::strelka::streams::StreamUpdate;
use crate::strelka::actors::StreamActor;

pub enum ActorCommand {
    Abort
}

impl Message for ActorCommand {
    type Result = ();
}

pub struct CommandAndControl {}

impl StreamActor for CommandAndControl {
    fn request_streams(&self) -> Vec<String> {
        vec!("Commands".to_owned())
    }

    fn receive(&self, msg: StreamUpdate) {
        
    }
}

impl Actor for CommandAndControl {
    type Context = Context<Self>;
}
