use actix::prelude::{Actor, Context, Handler, Message};
use krpc_mars::RPCClient;

use crate::krpc::space_center;

#[derive(Debug)]
pub enum Command {
    Stage,
}

/// CommandActor takes Commands, translates them into kRPC calls and executes those calls.
pub struct CommandActor {
    client: RPCClient,
}

impl CommandActor {
    pub fn new(krpc_client: RPCClient) -> Self {
        CommandActor{ client: krpc_client }
    }

    fn handle_cmd__stage(&self)-> Result<(), failure::Error> {
        let vessel = self.client.mk_call(&space_center::get_active_vessel())?;
        let control = self.client.mk_call(&vessel.get_control())?;
        self.client.mk_call(&control.activate_next_stage());
        Ok(())
    }

}

impl Message for Command {
    type Result = ();
}

impl Actor for CommandActor {
    type Context = Context<Self>;
}

impl Handler<Command> for CommandActor {
    type Result = ();

    fn handle(&mut self, command: Command, _: &mut Context<Self>) -> Self::Result {
        match command {
            Stage => self.handle_cmd__stage(),
        };
    }
}