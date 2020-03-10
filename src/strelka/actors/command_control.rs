use actix::prelude::{Actor, Context, Handler, Message};

#[derive(Debug)]
pub enum Command {
    Stage,
}

/// CommandAndControl takes ActorCommands, translates them into kRPC calls and executes those calls.
/// This should be the only Actor in strelka that emits KRPC calls
pub struct CommandAndControl {}

impl Message for Command {
    type Result = ();
}

impl Actor for CommandAndControl {
    type Context = Context<Self>;
}

impl Handler<Command> for CommandAndControl {
    type Result = ();

    fn handle(&mut self, command: Command, _: &mut Context<Self>) -> Self::Result {
        println!("Received command {:?}", command);
    }
}