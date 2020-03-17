use actix::prelude::*;

use std::thread;
use std::time::Duration;
use std::boxed::Box;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::strelka::actors::StreamUpdate;
use crate::strelka::actors::command::CommandActor;
use crate::strelka::actors::stager::StageActor;
use crate::strelka::actors::streamer::{Streamer, StreamValues};
use crate::strelka::actors::spawner::{Spawner, SpawnerCommand, StreamAddrs};
use crate::strelka::actors::altitude::AltitudeActor;
use crate::strelka::actors::ignition::IgnitionActor;
use crate::strelka::actors::gravity_turn::GravityTurnActor;

pub struct ActorController {
    cmd_actor: actix::Addr<CommandActor>,
    stream_actor: actix::Addr<Streamer>,
    spawner: Addr<Spawner>,
    stream_addrs: StreamAddrs,
}

// Ideally we'd put all the logic in here into its own Actor inside the system; however there are
// complications with calling async fns in actix message handlers, so do it here for now.
impl ActorController {

    pub fn new(krpc_client: krpc_mars::RPCClient, stream_client: krpc_mars::StreamClient) -> Self {
        let stream_addrs = Arc::new(Mutex::new(HashMap::new()));

        ActorController{
            cmd_actor: CommandActor::new(krpc_client.clone()).start(),
            stream_actor: Streamer::new(krpc_client, stream_client).start(),
            spawner: Spawner::new(stream_addrs.clone()).start(),
            stream_addrs,
        }
    }

    pub async fn broadcast_stream_update(&self, update: StreamUpdate) {
        let stream_addrs = self.stream_addrs.lock().unwrap();

        // In response to stream updates, actors can call the Spawner to create new actors. That requires
        // the lock on stream_addrs, so make a clone and unlock the mutex so we don't hang on new actor create.
        let addrs_copy = stream_addrs.clone();
        drop(stream_addrs);

        match addrs_copy.get(&update.to_string()) {
            Some(actors) => {
                for a in actors {
                    match (*a).send(update).await {
                        Err(e) => {
                            if a.connected() {
                                error!("Stream broadcast failed: Message: {:?}, Err: {:?}", update, e)
                            }

                            // TODO: Actors should tidy up on stopping .. not essential yet
                        },
                        _ => {}
                    }
                }
            },
            None => {}
        }
    }

    pub async fn tick(&self) {
        let result = self.stream_actor.send(StreamValues{}).await;
        trace!("Stream updates: {:?}", result);

        if let Ok(updated_values) = result {
            for update in updated_values {
                self.broadcast_stream_update(*update).await;
            }
        }

        // TODO: We'll want to make this sleep smaller so we can react faster to streams.
        //       However the countdown relies on ticks being a second, so we need the timer
        //       actor infx to work before we can do this. 
        thread::sleep(Duration::from_millis(1000));
    }

    pub async fn start(&mut self) {
        self.spawner.do_send(SpawnerCommand::Spawn(Box::new(AltitudeActor::new())));
        self.spawner.do_send(SpawnerCommand::Spawn(Box::new(GravityTurnActor::new(self.cmd_actor.clone(), self.spawner.clone()))));
        self.spawner.do_send(SpawnerCommand::Spawn(Box::new(IgnitionActor::new(self.cmd_actor.clone()))));
        self.spawner.do_send(SpawnerCommand::Spawn(Box::new(StageActor::new(self.cmd_actor.clone()))));
    }
    
    // pub async fn stop_actors(&self) {
    //     System::current().stop();
    // }

}
