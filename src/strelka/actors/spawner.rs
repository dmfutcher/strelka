use actix::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::strelka::actors::{StreamActor};

// StreamAddrs is shared between this actor and the ActorController
pub type StreamAddrs = Arc<Mutex<HashMap<String, Vec<Addr<Box<dyn StreamActor>>>>>>;

pub struct Spawner {
    stream_addrs: StreamAddrs,
}

pub enum SpawnerCommand {
    Spawn(Box<dyn StreamActor>),
}

impl Spawner {
    pub fn new(stream_addrs: StreamAddrs) -> Self {
        Spawner{
            stream_addrs,
        }
    }

    fn handle_spawn(&mut self, actor: Box<dyn StreamActor>) {
        info!("Starting {} program", &actor.name());
        let linked_streams = actor.request_streams();
        let address = actor.start();

        let mut stream_addrs = self.stream_addrs.lock().unwrap();
    
        for stream_name in linked_streams {
            let addrs = stream_addrs.entry(stream_name.to_string()).or_insert(vec!());
            addrs.push(address.clone());
        }
    }
}

impl Actor for Spawner {
    type Context = Context<Self>;
}

impl Message for SpawnerCommand {
    type Result = ();
}

impl Handler<SpawnerCommand> for Spawner {
    type Result = ();

    fn handle(&mut self, cmd: SpawnerCommand, _: &mut Context<Self>) -> Self::Result {
        match cmd {
            SpawnerCommand::Spawn(a) => {
                self.handle_spawn(a);
            },
        }
    }
}
