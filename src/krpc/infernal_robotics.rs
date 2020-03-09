use krpc_mars::krpc as krpc;
use krpc_mars::protobuf as protobuf;
use krpc_mars::CallHandle;
use krpc_mars::codec::RPCEncodable;
use krpc_mars::codec::RPCExtractable;


use crate::krpc::space_center;

use std::fmt;



#[derive(Clone)]
pub struct Servo {
    id: u32,
}

impl fmt::Debug for Servo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Servo({})", self.id)
    }
}

impl RPCEncodable for Servo {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Servo {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Servo { id })
    }
}


#[derive(Clone)]
pub struct ServoGroup {
    id: u32,
}

impl fmt::Debug for ServoGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ServoGroup({})", self.id)
    }
}

impl RPCEncodable for ServoGroup {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for ServoGroup {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(ServoGroup { id })
    }
}






/// <doc> <summary> Returns the servo group in the given <paramref name="vessel" /> with the given <paramref name="name" />, or <c>null</c> if none exists. If multiple servo groups have the same name, only one of them is returned. </summary> <param name="vessel">Vessel to check.</param> <param name="name">Name of servo group to find.</param> </doc>
pub fn servo_group_with_name(p_vessel: &space_center::Vessel, p_name: String) -> CallHandle<ServoGroup> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("InfernalRobotics"));
    proc_call.set_procedure(String::from("ServoGroupWithName"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_vessel.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_name.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> A list of all the servo groups in the given <paramref name="vessel" />. </summary> </doc>
pub fn servo_groups(p_vessel: &space_center::Vessel) -> CallHandle<Vec<ServoGroup>> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("InfernalRobotics"));
    proc_call.set_procedure(String::from("ServoGroups"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_vessel.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Returns the servo in the given <paramref name="vessel" /> with the given <paramref name="name" /> or <c>null</c> if none exists. If multiple servos have the same name, only one of them is returned. </summary> <param name="vessel">Vessel to check.</param> <param name="name">Name of the servo to find.</param> </doc>
pub fn servo_with_name(p_vessel: &space_center::Vessel, p_name: String) -> CallHandle<Servo> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("InfernalRobotics"));
    proc_call.set_procedure(String::from("ServoWithName"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_vessel.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_name.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Whether Infernal Robotics is installed. </summary> </doc>
pub fn get_available() -> CallHandle<bool> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("InfernalRobotics"));
    proc_call.set_procedure(String::from("get_Available"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Whether Infernal Robotics API is ready. </summary> </doc>
pub fn get_ready() -> CallHandle<bool> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("InfernalRobotics"));
    proc_call.set_procedure(String::from("get_Ready"));

    CallHandle::new(proc_call)
}


impl Servo {
    /// <doc> <summary> Moves the servo to the center. </summary> </doc>
    pub fn move_center(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_MoveCenter"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Moves the servo to the left. </summary> </doc>
    pub fn move_left(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_MoveLeft"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Moves the servo to the next preset. </summary> </doc>
    pub fn move_next_preset(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_MoveNextPreset"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Moves the servo to the previous preset. </summary> </doc>
    pub fn move_prev_preset(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_MovePrevPreset"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Moves the servo to the right. </summary> </doc>
    pub fn move_right(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_MoveRight"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Moves the servo to <paramref name="position" /> and sets the speed multiplier to <paramref name="speed" />. </summary> <param name="position">The position to move the servo to.</param> <param name="speed">Speed multiplier for the movement.</param> </doc>
    pub fn move_to(&self, p_position: f32, p_speed: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_MoveTo"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_position.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_speed.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Stops the servo. </summary> </doc>
    pub fn stop(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_Stop"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current speed multiplier set in the UI. </summary> </doc>
    pub fn get_acceleration(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_get_Acceleration"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The speed multiplier of the servo, specified by the part configuration. </summary> </doc>
    pub fn get_config_speed(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_get_ConfigSpeed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current speed at which the servo is moving. </summary> </doc>
    pub fn get_current_speed(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_get_CurrentSpeed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the servos axis is inverted. </summary> </doc>
    pub fn get_is_axis_inverted(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_get_IsAxisInverted"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the servo is freely moving. </summary> </doc>
    pub fn get_is_free_moving(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_get_IsFreeMoving"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the servo is locked. </summary> </doc>
    pub fn get_is_locked(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_get_IsLocked"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the servo is moving. </summary> </doc>
    pub fn get_is_moving(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_get_IsMoving"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The maximum position of the servo, specified by the part configuration. </summary> </doc>
    pub fn get_max_config_position(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_get_MaxConfigPosition"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The maximum position of the servo, specified by the in-game tweak menu. </summary> </doc>
    pub fn get_max_position(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_get_MaxPosition"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The minimum position of the servo, specified by the part configuration. </summary> </doc>
    pub fn get_min_config_position(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_get_MinConfigPosition"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The minimum position of the servo, specified by the in-game tweak menu. </summary> </doc>
    pub fn get_min_position(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_get_MinPosition"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The name of the servo. </summary> </doc>
    pub fn get_name(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_get_Name"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part containing the servo. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<space_center::Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position of the servo. </summary> </doc>
    pub fn get_position(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_get_Position"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The speed multiplier of the servo, specified by the in-game tweak menu. </summary> </doc>
    pub fn get_speed(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_get_Speed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current speed multiplier set in the UI. </summary> </doc>
    pub fn set_acceleration(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_set_Acceleration"));

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

    /// <doc> <summary> The current speed at which the servo is moving. </summary> </doc>
    pub fn set_current_speed(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_set_CurrentSpeed"));

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

    /// <doc> <summary> Whether the servo should be highlighted in-game. </summary> </doc>
    pub fn set_highlight(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_set_Highlight"));

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

    /// <doc> <summary> Whether the servos axis is inverted. </summary> </doc>
    pub fn set_is_axis_inverted(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_set_IsAxisInverted"));

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

    /// <doc> <summary> Whether the servo is locked. </summary> </doc>
    pub fn set_is_locked(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_set_IsLocked"));

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

    /// <doc> <summary> The maximum position of the servo, specified by the in-game tweak menu. </summary> </doc>
    pub fn set_max_position(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_set_MaxPosition"));

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

    /// <doc> <summary> The minimum position of the servo, specified by the in-game tweak menu. </summary> </doc>
    pub fn set_min_position(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_set_MinPosition"));

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

    /// <doc> <summary> The name of the servo. </summary> </doc>
    pub fn set_name(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_set_Name"));

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

    /// <doc> <summary> The speed multiplier of the servo, specified by the in-game tweak menu. </summary> </doc>
    pub fn set_speed(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("Servo_set_Speed"));

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
impl ServoGroup {
    /// <doc> <summary> Moves all of the servos in the group to the center. </summary> </doc>
    pub fn move_center(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_MoveCenter"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Moves all of the servos in the group to the left. </summary> </doc>
    pub fn move_left(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_MoveLeft"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Moves all of the servos in the group to the next preset. </summary> </doc>
    pub fn move_next_preset(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_MoveNextPreset"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Moves all of the servos in the group to the previous preset. </summary> </doc>
    pub fn move_prev_preset(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_MovePrevPreset"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Moves all of the servos in the group to the right. </summary> </doc>
    pub fn move_right(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_MoveRight"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns the servo with the given <paramref name="name" /> from this group, or <c>null</c> if none exists. </summary> <param name="name">Name of servo to find.</param> </doc>
    pub fn servo_with_name(&self, p_name: String) -> CallHandle<Servo> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_ServoWithName"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_name.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Stops the servos in the group. </summary> </doc>
    pub fn stop(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_Stop"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the group is expanded in the InfernalRobotics UI. </summary> </doc>
    pub fn get_expanded(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_get_Expanded"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The key assigned to be the "forward" key for the group. </summary> </doc>
    pub fn get_forward_key(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_get_ForwardKey"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The name of the group. </summary> </doc>
    pub fn get_name(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_get_Name"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The parts containing the servos in the group. </summary> </doc>
    pub fn get_parts(&self, ) -> CallHandle<Vec<space_center::Part>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_get_Parts"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The key assigned to be the "reverse" key for the group. </summary> </doc>
    pub fn get_reverse_key(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_get_ReverseKey"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The servos that are in the group. </summary> </doc>
    pub fn get_servos(&self, ) -> CallHandle<Vec<Servo>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_get_Servos"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The speed multiplier for the group. </summary> </doc>
    pub fn get_speed(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_get_Speed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the group is expanded in the InfernalRobotics UI. </summary> </doc>
    pub fn set_expanded(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_set_Expanded"));

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

    /// <doc> <summary> The key assigned to be the "forward" key for the group. </summary> </doc>
    pub fn set_forward_key(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_set_ForwardKey"));

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

    /// <doc> <summary> The name of the group. </summary> </doc>
    pub fn set_name(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_set_Name"));

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

    /// <doc> <summary> The key assigned to be the "reverse" key for the group. </summary> </doc>
    pub fn set_reverse_key(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_set_ReverseKey"));

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

    /// <doc> <summary> The speed multiplier for the group. </summary> </doc>
    pub fn set_speed(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("InfernalRobotics"));
        proc_call.set_procedure(String::from("ServoGroup_set_Speed"));

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

