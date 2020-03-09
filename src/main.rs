extern crate krpc_mars;
extern crate failure;

mod krpc;

use krpc::space_center;

fn main() -> Result<(), failure::Error> {
    let client = krpc_mars::RPCClient::connect("Example", "127.0.0.1:50000")?;

    let vessel = client.mk_call(&space_center::get_active_vessel())?;
    println!("Active vessel: {:?}", vessel);

    let control = client.mk_call(&vessel.get_control())?;

    let result = client.mk_call(&control.activate_next_stage())?;
    println!("{:?}", result);

    Ok(())
}