use krpc_mars::krpc as krpc;
use krpc_mars::protobuf as protobuf;
use krpc_mars::CallHandle;
use krpc_mars::codec::RPCEncodable;
use krpc_mars::codec::RPCExtractable;


use crate::krpc::space_center;

use std::fmt;



#[derive(Clone)]
pub struct Antenna {
    id: u32,
}

impl fmt::Debug for Antenna {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Antenna({})", self.id)
    }
}

impl RPCEncodable for Antenna {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Antenna {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Antenna { id })
    }
}


#[derive(Clone)]
pub struct Comms {
    id: u32,
}

impl fmt::Debug for Comms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Comms({})", self.id)
    }
}

impl RPCEncodable for Comms {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Comms {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Comms { id })
    }
}




#[derive(Debug, Copy, Clone)]
pub enum Target {
    ActiveVessel = 0,
    CelestialBody = 1,
    GroundStation = 2,
    Vessel = 3,
    None = 4,
}

impl From<i32> for Target {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => Target::ActiveVessel,
            1 => Target::CelestialBody,
            2 => Target::GroundStation,
            3 => Target::Vessel,
            4 => Target::None,
            _ => panic!("Could not convert '{}' to a RemoteTech::Target", source),
        }
    }
}

impl RPCEncodable for Target {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for Target {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(Target::from(value))
    }
}



/// <doc> <summary> Get the antenna object for a particular part. </summary> </doc>
pub fn antenna(p_part: &space_center::Part) -> CallHandle<Antenna> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("RemoteTech"));
    proc_call.set_procedure(String::from("Antenna"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_part.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Get a communications object, representing the communication capability of a particular vessel. </summary> </doc>
pub fn comms(p_vessel: &space_center::Vessel) -> CallHandle<Comms> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("RemoteTech"));
    proc_call.set_procedure(String::from("Comms"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_vessel.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Whether RemoteTech is installed. </summary> </doc>
pub fn get_available() -> CallHandle<bool> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("RemoteTech"));
    proc_call.set_procedure(String::from("get_Available"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The names of the ground stations. </summary> </doc>
pub fn get_ground_stations() -> CallHandle<Vec<String>> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("RemoteTech"));
    proc_call.set_procedure(String::from("get_GroundStations"));

    CallHandle::new(proc_call)
}


impl Antenna {
    /// <doc> <summary> Whether the antenna has a connection. </summary> </doc>
    pub fn get_has_connection(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Antenna_get_HasConnection"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Get the part containing this antenna. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<space_center::Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Antenna_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The object that the antenna is targetting. This property can be used to set the target to <see cref="M:RemoteTech.Target.None" /> or <see cref="M:RemoteTech.Target.ActiveVessel" />. To set the target to a celestial body, ground station or vessel see <see cref="M:RemoteTech.Antenna.TargetBody" />, <see cref="M:RemoteTech.Antenna.TargetGroundStation" /> and <see cref="M:RemoteTech.Antenna.TargetVessel" />. </summary> </doc>
    pub fn get_target(&self, ) -> CallHandle<Target> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Antenna_get_Target"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The celestial body the antenna is targetting. </summary> </doc>
    pub fn get_target_body(&self, ) -> CallHandle<space_center::CelestialBody> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Antenna_get_TargetBody"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The ground station the antenna is targetting. </summary> </doc>
    pub fn get_target_ground_station(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Antenna_get_TargetGroundStation"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The vessel the antenna is targetting. </summary> </doc>
    pub fn get_target_vessel(&self, ) -> CallHandle<space_center::Vessel> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Antenna_get_TargetVessel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The object that the antenna is targetting. This property can be used to set the target to <see cref="M:RemoteTech.Target.None" /> or <see cref="M:RemoteTech.Target.ActiveVessel" />. To set the target to a celestial body, ground station or vessel see <see cref="M:RemoteTech.Antenna.TargetBody" />, <see cref="M:RemoteTech.Antenna.TargetGroundStation" /> and <see cref="M:RemoteTech.Antenna.TargetVessel" />. </summary> </doc>
    pub fn set_target(&self, p_value: Target) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Antenna_set_Target"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_value.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The celestial body the antenna is targetting. </summary> </doc>
    pub fn set_target_body(&self, p_value: &space_center::CelestialBody) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Antenna_set_TargetBody"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_value.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The ground station the antenna is targetting. </summary> </doc>
    pub fn set_target_ground_station(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Antenna_set_TargetGroundStation"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_value.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The vessel the antenna is targetting. </summary> </doc>
    pub fn set_target_vessel(&self, p_value: &space_center::Vessel) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Antenna_set_TargetVessel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_value.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl Comms {
    /// <doc> <summary> The signal delay between the this vessel and another vessel, in seconds. </summary> <param name="other"></param> </doc>
    pub fn signal_delay_to_vessel(&self, p_other: &space_center::Vessel) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Comms_SignalDelayToVessel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_other.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The antennas for this vessel. </summary> </doc>
    pub fn get_antennas(&self, ) -> CallHandle<Vec<Antenna>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Comms_get_Antennas"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the vessel has any connection. </summary> </doc>
    pub fn get_has_connection(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Comms_get_HasConnection"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the vessel has a connection to a ground station. </summary> </doc>
    pub fn get_has_connection_to_ground_station(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Comms_get_HasConnectionToGroundStation"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the vessel has a flight computer on board. </summary> </doc>
    pub fn get_has_flight_computer(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Comms_get_HasFlightComputer"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the vessel can be controlled locally. </summary> </doc>
    pub fn get_has_local_control(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Comms_get_HasLocalControl"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The shortest signal delay to the vessel, in seconds. </summary> </doc>
    pub fn get_signal_delay(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Comms_get_SignalDelay"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The signal delay between the vessel and the closest ground station, in seconds. </summary> </doc>
    pub fn get_signal_delay_to_ground_station(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Comms_get_SignalDelayToGroundStation"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Get the vessel. </summary> </doc>
    pub fn get_vessel(&self, ) -> CallHandle<space_center::Vessel> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("RemoteTech"));
        proc_call.set_procedure(String::from("Comms_get_Vessel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}

