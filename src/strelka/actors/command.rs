use actix::prelude::{Actor, Context, Handler, Message};
use krpc_mars::RPCClient;

use crate::krpc::space_center;

#[derive(Debug)]
pub enum Command {
    Stage,
    Pitch(f32),
}

/// CommandActor takes ActorCommands, translates them into kRPC calls and executes those calls.
/// This should be the only Actor in strelka that emits KRPC calls
pub struct CommandActor {
    client: RPCClient,
}

impl CommandActor {
    pub fn new(krpc_client: RPCClient) -> Self {
        CommandActor{ client: krpc_client }
    }

    fn handle_cmd_stage(&self)-> Result<(), failure::Error> {
        let vessel = self.client.mk_call(&space_center::get_active_vessel())?;
        let control = self.client.mk_call(&vessel.get_control())?;
        self.client.mk_call(&control.activate_next_stage());
        Ok(())
    }

    fn handle_cmd_pitch(&self) -> Result<(), failure::Error> {
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
            Stage => self.handle_cmd_stage(),
        };
    }
}