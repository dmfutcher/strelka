use actix::prelude::{Actor, Context, Handler, Message};
use krpc_mars::RPCClient;

use crate::krpc::space_center;

#[derive(Debug)]
pub enum ControlAction {
    Tick,
}

/// Controller is the main control Actor in the Strelka system, it organises every other Actor
pub struct ControlActor {
    client: RPCClient,
}

impl ControlActor {
    pub fn new(krpc_client: RPCClient) -> Self {
        ControlActor {
            client: krpc_client
        }
    }

}

impl Message for ControlAction {
    type Result = ();
}

impl Actor for ControlActor {
    type Context = Context<Self>;
}

impl Handler<ControlAction> for ControlActor {
    type Result = ();

    fn handle(&mut self, act: ControlAction, _: &mut Context<Self>) -> Self::Result {
        println!("Controller: {:?}", act);
    }
}


/*

    pub fn new(krpc_client: krpc_mars::RPCClient, stream_client: krpc_mars::StreamClient) -> Self {
        ActorController{ 
            actors: vec!(),
            stream_map: HashMap::new(),
            command_actor: CommandActor::new(krpc_client.clone()).start(),
            stream_actor: Streamer::new(krpc_client, stream_client).start(),
        }
    }

    pub fn register_actor(&mut self, actor: Box<dyn StreamActor>) {
        let linked_streams = actor.request_streams();
        let address = actor.start();
        self.actors.push(address.clone());

        for stream_name in linked_streams {
            let addrs = self.stream_map.entry(stream_name).or_insert(vec!());
            addrs.push(address.clone());
        }
    }

    pub async fn broadcast(&self, update: StreamUpdate) {
        match self.stream_map.get(&update.to_string()) {
            Some(actors) => {
                for a in actors {
                    (*a).send(update).await;
                }
            },
            None => {}
        }
    }

    // TODO: All of this stuff should probably live in its own Actor instead of the main thread
    pub async fn tick(&self) {
        let result = self.stream_actor.send(StreamValues{}).await;
        if let Ok(updated_values) = result {
            for update in updated_values {
                self.broadcast(update).await;
            }
        }

        thread::sleep(time::Duration::from_secs(1));
    }

    pub async fn start(&mut self) {
        self.register_actor(Box::new(AltitudeMonitor{}));
        self.register_actor(Box::new(IgnitionActor::new(self.command_actor.clone())));

        println!("{:?}", self.stream_map.keys());
    }
    
    pub async fn stop_actors(&self) {
        System::current().stop();
    }

    */