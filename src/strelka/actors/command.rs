use actix::prelude::{Actor, Context, Handler, Message};
use krpc_mars::RPCClient;
use failure::format_err;

use crate::krpc::space_center;
use crate::krpc::space_center::Control;

#[derive(Debug)]
pub enum Command {
    Stage,
    SetPitch(f32),
    SetSAS(bool),
    SetSASMode(space_center::SASMode),
    SetRCS(bool),
    SetThrottle(f32),
    CreateOrbitNode(f64),
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
        self.with_control(|ctl| { 
            self.client.mk_call(&ctl.activate_next_stage())?;
            Ok(())
        })
    }

    fn handle_cmd_pitch(&self, val: f32) -> Result<(), failure::Error> {
        self.with_control(|ctl| { 
            // XXX: This is a can of worms but what I've been calling "Pitch" seems to be set by the Yaw control ...
            self.client.mk_call(&ctl.set_yaw(val))
        })
    }

    fn handle_cmd_set_sas(&self, toggle: bool) -> Result<(), failure::Error> {
        self.with_control(|ctl| { 
            self.client.mk_call(&ctl.set_sas(toggle))
        })
    }

    fn handle_cmd_set_sas_mode(&self, mode: space_center::SASMode) -> Result<(), failure::Error> {
        self.with_control(|ctl| { 
            self.client.mk_call(&ctl.set_sas_mode(mode))
        })
    }

    fn handle_cmd_set_rcs(&self, toggle: bool) -> Result<(), failure::Error> {
        self.with_control(|ctl| { 
            self.client.mk_call(&ctl.set_rcs(toggle))
        })
    }

    fn handle_cmd_set_throttle(&self, throttle: f32) -> Result<(), failure::Error> {
        self.with_control(|ctl| { 
            self.client.mk_call(&ctl.set_throttle(throttle))
        })
    }

    fn handle_cmd_create_orbit_node(&self, apo_alt: f64) -> Result<(), failure::Error> {
            //  XXX: TODO: etc. 
            // Here we hit a point where the stream abstraction starts to fall apart a bit. We need the body
            // we're orbiting's gravitational parameter and the orbit's semi_major axis, as well as our time
            // to apoapse and apo altitude all at the same time.
            // 
            // I'd love to do this calculation inside the burn to apo actor & putting it here feels like a 
            // bit of a cop-out. However, I wanna make it to orbit today, so here we go ...
            let vessel = self.client.mk_call(&space_center::get_active_vessel())?;
            let control = self.client.mk_call(&vessel.get_control())?;
            let orbit = self.client.mk_call(&vessel.get_orbit())?;
            let body = self.client.mk_call(&orbit.get_body())?;
            let ut = self.client.mk_call(&space_center::get_ut())?;
            let time_to_apo = self.client.mk_call(&orbit.get_time_to_apoapsis())?;
            let kerbin_ref = self.client.mk_call(&body.get_reference_frame())?;
            let flight = self.client.mk_call(&vessel.flight(&kerbin_ref))?;

            let mu = self.client.mk_call(&body.get_gravitational_parameter())? as f64;
            let r = apo_alt;
            let a1 = self.client.mk_call(&orbit.get_semi_major_axis())?;
            let a2 = r;
            let v1 = (mu * ((2.0 / r) - (1.0 / a1))).sqrt();
            let v2 = (mu * ((2.0 / r) - (1.0 / a2))).sqrt();

            // XXX: This calculation is consistently coming out rougly 100m/s short every time 
            //      I've stared long enough I'm just going to fudge it for now.
            let delta_v = -(v2 - v1) + 100.0; 
            
            trace!("apo_alt = {}", apo_alt);
            trace!("a1 = {}", a1);
            trace!("a2 = {}", a2);
            trace!("r = {}", r);
            trace!("mu = {}", mu);
            trace!("v1 = {}", v1);
            trace!("v2 = {}", v2);
            trace!("dV = {}", delta_v);

            let current_v = self.client.mk_call(&flight.get_speed())?;
            self.client.mk_call(&control.add_node(ut + time_to_apo, (delta_v - current_v) as f32, 0.0, 0.0))?;
            Ok(())
    }

    // Get the controls struct, unwrap it safely, then run the given function with the controls struct,
    // avoiding duplicated krpc boilerplate wherever possible.
    fn with_control<F>(&self, f: F) -> Result<(), failure::Error>
        where F: Fn(&Control) -> Result<(), krpc_mars::error::Error>
    {
        let vessel = self.client.mk_call(&space_center::get_active_vessel())?;
        let control = self.client.mk_call(&vessel.get_control())?; 

        f(&control).map_err(|e| format_err!("Command failed: {}", e))
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
        debug!("Executing {:?}", command);
        let result = match command {
            Command::Stage => self.handle_cmd_stage(),
            Command::SetPitch(pitch_ctl_val) => self.handle_cmd_pitch(pitch_ctl_val),
            Command::SetSAS(toggle) => self.handle_cmd_set_sas(toggle),
            Command::SetSASMode(mode) => self.handle_cmd_set_sas_mode(mode),
            Command::SetRCS(toggle) => self.handle_cmd_set_rcs(toggle),
            Command::SetThrottle(throttle_val) => self.handle_cmd_set_throttle(throttle_val),
            Command::CreateOrbitNode(apo) => self.handle_cmd_create_orbit_node(apo),
        };

        if let Err(e) = result {
            error!("Executing command {:?} failed: {}", command, e);
        }
    }

}
