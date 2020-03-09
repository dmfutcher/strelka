use krpc_mars::krpc as krpc;
use krpc_mars::protobuf as protobuf;
use krpc_mars::CallHandle;
use krpc_mars::codec::RPCEncodable;
use krpc_mars::codec::RPCExtractable;


use crate::krpc::space_center;

use std::fmt;



#[derive(Clone)]
pub struct Alarm {
    id: u32,
}

impl fmt::Debug for Alarm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alarm({})", self.id)
    }
}

impl RPCEncodable for Alarm {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Alarm {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Alarm { id })
    }
}




#[derive(Debug, Copy, Clone)]
pub enum AlarmAction {
    DoNothing = 0,
    DoNothingDeleteWhenPassed = 1,
    KillWarp = 2,
    KillWarpOnly = 3,
    MessageOnly = 4,
    PauseGame = 5,
}

impl From<i32> for AlarmAction {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => AlarmAction::DoNothing,
            1 => AlarmAction::DoNothingDeleteWhenPassed,
            2 => AlarmAction::KillWarp,
            3 => AlarmAction::KillWarpOnly,
            4 => AlarmAction::MessageOnly,
            5 => AlarmAction::PauseGame,
            _ => panic!("Could not convert '{}' to a KerbalAlarmClock::AlarmAction", source),
        }
    }
}

impl RPCEncodable for AlarmAction {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for AlarmAction {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(AlarmAction::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AlarmType {
    Raw = 0,
    Maneuver = 1,
    ManeuverAuto = 2,
    Apoapsis = 3,
    Periapsis = 4,
    AscendingNode = 5,
    DescendingNode = 6,
    Closest = 7,
    Contract = 8,
    ContractAuto = 9,
    Crew = 10,
    Distance = 11,
    EarthTime = 12,
    LaunchRendevous = 13,
    SOIChange = 14,
    SOIChangeAuto = 15,
    Transfer = 16,
    TransferModelled = 17,
}

impl From<i32> for AlarmType {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => AlarmType::Raw,
            1 => AlarmType::Maneuver,
            2 => AlarmType::ManeuverAuto,
            3 => AlarmType::Apoapsis,
            4 => AlarmType::Periapsis,
            5 => AlarmType::AscendingNode,
            6 => AlarmType::DescendingNode,
            7 => AlarmType::Closest,
            8 => AlarmType::Contract,
            9 => AlarmType::ContractAuto,
            10 => AlarmType::Crew,
            11 => AlarmType::Distance,
            12 => AlarmType::EarthTime,
            13 => AlarmType::LaunchRendevous,
            14 => AlarmType::SOIChange,
            15 => AlarmType::SOIChangeAuto,
            16 => AlarmType::Transfer,
            17 => AlarmType::TransferModelled,
            _ => panic!("Could not convert '{}' to a KerbalAlarmClock::AlarmType", source),
        }
    }
}

impl RPCEncodable for AlarmType {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for AlarmType {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(AlarmType::from(value))
    }
}



/// <doc> <summary> Get the alarm with the given <paramref name="name" />, or <c>null</c> if no alarms have that name. If more than one alarm has the name, only returns one of them. </summary> <param name="name">Name of the alarm to search for.</param> </doc>
pub fn alarm_with_name(p_name: String) -> CallHandle<Alarm> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KerbalAlarmClock"));
    proc_call.set_procedure(String::from("AlarmWithName"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_name.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Get a list of alarms of the specified <paramref name="type" />. </summary> <param name="type">Type of alarm to return.</param> </doc>
pub fn alarms_with_type(p_type: AlarmType) -> CallHandle<Vec<Alarm>> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KerbalAlarmClock"));
    proc_call.set_procedure(String::from("AlarmsWithType"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_type.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Create a new alarm and return it. </summary> <param name="type">Type of the new alarm.</param> <param name="name">Name of the new alarm.</param> <param name="ut">Time at which the new alarm should trigger.</param> </doc>
pub fn create_alarm(p_type: AlarmType, p_name: String, p_ut: f64) -> CallHandle<Alarm> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KerbalAlarmClock"));
    proc_call.set_procedure(String::from("CreateAlarm"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_type.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_name.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_ut.encode_to_bytes().unwrap());
    arguments.push(arg2);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> A list of all the alarms. </summary> </doc>
pub fn get_alarms() -> CallHandle<Vec<Alarm>> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KerbalAlarmClock"));
    proc_call.set_procedure(String::from("get_Alarms"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Whether Kerbal Alarm Clock is available. </summary> </doc>
pub fn get_available() -> CallHandle<bool> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KerbalAlarmClock"));
    proc_call.set_procedure(String::from("get_Available"));

    CallHandle::new(proc_call)
}


impl Alarm {
    /// <doc> <summary> Removes the alarm. </summary> </doc>
    pub fn remove(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_Remove"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The action that the alarm triggers. </summary> </doc>
    pub fn get_action(&self, ) -> CallHandle<AlarmAction> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_get_Action"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The unique identifier for the alarm. </summary> </doc>
    pub fn get_id(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_get_ID"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The number of seconds before the event that the alarm will fire. </summary> </doc>
    pub fn get_margin(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_get_Margin"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The short name of the alarm. </summary> </doc>
    pub fn get_name(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_get_Name"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The long description of the alarm. </summary> </doc>
    pub fn get_notes(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_get_Notes"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The number of seconds until the alarm will fire. </summary> </doc>
    pub fn get_remaining(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_get_Remaining"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the alarm will be repeated after it has fired. </summary> </doc>
    pub fn get_repeat(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_get_Repeat"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The time delay to automatically create an alarm after it has fired. </summary> </doc>
    pub fn get_repeat_period(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_get_RepeatPeriod"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The time at which the alarm will fire. </summary> </doc>
    pub fn get_time(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_get_Time"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The type of the alarm. </summary> </doc>
    pub fn get_type(&self, ) -> CallHandle<AlarmType> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_get_Type"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The vessel that the alarm is attached to. </summary> </doc>
    pub fn get_vessel(&self, ) -> CallHandle<space_center::Vessel> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_get_Vessel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The celestial body the vessel is departing from. </summary> </doc>
    pub fn get_xfer_origin_body(&self, ) -> CallHandle<space_center::CelestialBody> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_get_XferOriginBody"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The celestial body the vessel is arriving at. </summary> </doc>
    pub fn get_xfer_target_body(&self, ) -> CallHandle<space_center::CelestialBody> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_get_XferTargetBody"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The action that the alarm triggers. </summary> </doc>
    pub fn set_action(&self, p_value: AlarmAction) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_set_Action"));

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

    /// <doc> <summary> The number of seconds before the event that the alarm will fire. </summary> </doc>
    pub fn set_margin(&self, p_value: f64) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_set_Margin"));

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

    /// <doc> <summary> The short name of the alarm. </summary> </doc>
    pub fn set_name(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_set_Name"));

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

    /// <doc> <summary> The long description of the alarm. </summary> </doc>
    pub fn set_notes(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_set_Notes"));

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

    /// <doc> <summary> Whether the alarm will be repeated after it has fired. </summary> </doc>
    pub fn set_repeat(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_set_Repeat"));

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

    /// <doc> <summary> The time delay to automatically create an alarm after it has fired. </summary> </doc>
    pub fn set_repeat_period(&self, p_value: f64) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_set_RepeatPeriod"));

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

    /// <doc> <summary> The time at which the alarm will fire. </summary> </doc>
    pub fn set_time(&self, p_value: f64) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_set_Time"));

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

    /// <doc> <summary> The vessel that the alarm is attached to. </summary> </doc>
    pub fn set_vessel(&self, p_value: &space_center::Vessel) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_set_Vessel"));

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

    /// <doc> <summary> The celestial body the vessel is departing from. </summary> </doc>
    pub fn set_xfer_origin_body(&self, p_value: &space_center::CelestialBody) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_set_XferOriginBody"));

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

    /// <doc> <summary> The celestial body the vessel is arriving at. </summary> </doc>
    pub fn set_xfer_target_body(&self, p_value: &space_center::CelestialBody) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("KerbalAlarmClock"));
        proc_call.set_procedure(String::from("Alarm_set_XferTargetBody"));

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

