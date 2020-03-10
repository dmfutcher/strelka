use actix::prelude::*;
use std::collections::HashMap;
use strum_macros::Display;

#[derive(Display, Copy, Clone)]
enum StreamUpdate {
    Altitude(f64),
}

enum ActorCommand {
    Abort
}

impl Message for StreamUpdate {
    type Result = ();
}

trait StreamActor {
    fn request_streams(&self) -> Vec<String>;
    fn receive(&self, msg: StreamUpdate);
}

impl Actor for Box<dyn StreamActor> {
    type Context = Context<Self>;
}

impl Handler<StreamUpdate> for Box<dyn StreamActor> {
    type Result = ();

    fn handle(&mut self, msg: StreamUpdate, _: &mut Context<Self>) -> Self::Result {
        (*self).receive(msg);
    }
}

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

impl Handler<StreamUpdate> for AltitudeMonitor {
    type Result = ();

    fn handle(&mut self, msg: StreamUpdate, _: &mut Context<Self>) -> Self::Result {
        match msg { 
            StreamUpdate::Altitude(a) => {
                println!("Altitude Monitor says {:?}", a);
            }
        }
    }
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

impl Message for ActorCommand {
    type Result = ();
}

pub struct ActorController {
    actors: Vec<actix::Addr<Box<dyn StreamActor>>>,
    stream_map: HashMap<String, Vec<actix::Addr<Box<dyn StreamActor>>>>,
}

impl ActorController {
    fn new() -> Self {
        ActorController{ actors: vec!(), stream_map: HashMap::new() }
    }

    fn register_actor(&mut self, actor: Box<dyn StreamActor>) {
        let linked_streams = actor.request_streams();
        let address = actor.start();
        self.actors.push(address.clone());

        for stream_name in linked_streams {
            let addrs = self.stream_map.entry(stream_name).or_insert(vec!());
            addrs.push(address.clone());
        }
    }

    async fn broadcast(&self, update: StreamUpdate) {
        match self.stream_map.get(&update.to_string()) {
            Some(actors) => {
                for a in actors {
                    (*a).send(update).await;
                }
            },
            None => {}
        }
    }
}

#[actix_rt::main]
pub async fn run_actors() {
    let mut actor_controller = ActorController::new();
    let candc = CommandAndControl{ };
    let alt = AltitudeMonitor{ };
    actor_controller.register_actor(Box::new(candc));
    actor_controller.register_actor(Box::new(alt));

    actor_controller.broadcast(StreamUpdate::Altitude(100.0)).await;

    System::current().stop();
}