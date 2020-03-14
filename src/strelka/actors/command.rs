use actix::prelude::{Actor, Context, Handler, Message};
use krpc_mars::RPCClient;

use crate::krpc::space_center;

#[derive(Debug)]
pub enum Command {
    Stage,
    SetPitch(f32),
    SetSAS(bool),
    SetRCS(bool),
    SetThrottle(f32),
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

    fn handle_cmd_pitch(&self, val: f32) -> Result<(), failure::Error> {
        let vessel = self.client.mk_call(&space_center::get_active_vessel())?;
        let control = self.client.mk_call(&vessel.get_control())?; 

        // TODO: Definitely getting to the point we want proper error handling and logging implemented ...
        self.client.mk_call(&control.set_pitch(val))?;
        Ok(())
    }

    fn handle_cmd_set_sas(&self, toggle: bool) -> Result<(), failure::Error> {
        let vessel = self.client.mk_call(&space_center::get_active_vessel())?;
        let control = self.client.mk_call(&vessel.get_control())?; 

        self.client.mk_call(&control.set_sas(toggle));
        Ok(())
    }

    fn handle_cmd_set_rcs(&self, toggle: bool) -> Result<(), failure::Error> {
        let vessel = self.client.mk_call(&space_center::get_active_vessel())?;
        let control = self.client.mk_call(&vessel.get_control())?; 

        self.client.mk_call(&control.set_rcs(toggle));
        Ok(())
    }

    fn handle_cmd_set_throttle(&self, throttle: f32) -> Result<(), failure::Error> {
        let vessel = self.client.mk_call(&space_center::get_active_vessel())?;
        let control = self.client.mk_call(&vessel.get_control())?; 

        self.client.mk_call(&control.set_throttle(throttle));
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
            Command::Stage => self.handle_cmd_stage(),
            Command::SetPitch(pitch_ctl_val) => self.handle_cmd_pitch(pitch_ctl_val),
            Command::SetSAS(toggle) => self.handle_cmd_set_sas(toggle),
            Command::SetRCS(toggle) => self.handle_cmd_set_rcs(toggle),
            Command::SetThrottle(throttle_val) => self.handle_cmd_set_throttle(throttle_val)
        };
    }
}