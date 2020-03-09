use krpc_mars::krpc as krpc;
use krpc_mars::protobuf as protobuf;
use krpc_mars::CallHandle;
use krpc_mars::codec::RPCEncodable;
use krpc_mars::codec::RPCExtractable;


use crate::krpc::ui;
use crate::krpc::space_center;

use std::fmt;



#[derive(Clone)]
pub struct Line {
    id: u32,
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line({})", self.id)
    }
}

impl RPCEncodable for Line {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Line {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Line { id })
    }
}


#[derive(Clone)]
pub struct Polygon {
    id: u32,
}

impl fmt::Debug for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Polygon({})", self.id)
    }
}

impl RPCEncodable for Polygon {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Polygon {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Polygon { id })
    }
}


#[derive(Clone)]
pub struct Text {
    id: u32,
}

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Text({})", self.id)
    }
}

impl RPCEncodable for Text {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Text {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Text { id })
    }
}






/// <doc> <summary> Draw a direction vector in the scene, from the center of mass of the active vessel. </summary> <param name="direction">Direction to draw the line in.</param> <param name="referenceFrame">Reference frame that the direction is in.</param> <param name="length">The length of the line.</param> <param name="visible">Whether the line is visible.</param> </doc>
pub fn add_direction(p_direction: (f64, f64, f64), p_reference_frame: &space_center::ReferenceFrame, p_length: f32, p_visible: bool) -> CallHandle<Line> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("Drawing"));
    proc_call.set_procedure(String::from("AddDirection"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_direction.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_length.encode_to_bytes().unwrap());
    arguments.push(arg2);

    let mut arg3 = krpc::Argument::new();
    arg3.set_position(3);
    arg3.set_value(p_visible.encode_to_bytes().unwrap());
    arguments.push(arg3);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Draw a line in the scene. </summary> <param name="start">Position of the start of the line.</param> <param name="end">Position of the end of the line.</param> <param name="referenceFrame">Reference frame that the positions are in.</param> <param name="visible">Whether the line is visible.</param> </doc>
pub fn add_line(p_start: (f64, f64, f64), p_end: (f64, f64, f64), p_reference_frame: &space_center::ReferenceFrame, p_visible: bool) -> CallHandle<Line> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("Drawing"));
    proc_call.set_procedure(String::from("AddLine"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_start.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_end.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_reference_frame.encode_to_bytes().unwrap());
    arguments.push(arg2);

    let mut arg3 = krpc::Argument::new();
    arg3.set_position(3);
    arg3.set_value(p_visible.encode_to_bytes().unwrap());
    arguments.push(arg3);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Draw a polygon in the scene, defined by a list of vertices. </summary> <param name="vertices">Vertices of the polygon.</param> <param name="referenceFrame">Reference frame that the vertices are in.</param> <param name="visible">Whether the polygon is visible.</param> </doc>
pub fn add_polygon(p_vertices: &Vec<(f64, f64, f64)>, p_reference_frame: &space_center::ReferenceFrame, p_visible: bool) -> CallHandle<Polygon> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("Drawing"));
    proc_call.set_procedure(String::from("AddPolygon"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_vertices.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_visible.encode_to_bytes().unwrap());
    arguments.push(arg2);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Draw text in the scene. </summary> <param name="text">The string to draw.</param> <param name="referenceFrame">Reference frame that the text position is in.</param> <param name="position">Position of the text.</param> <param name="rotation">Rotation of the text, as a quaternion.</param> <param name="visible">Whether the text is visible.</param> </doc>
pub fn add_text(p_text: String, p_reference_frame: &space_center::ReferenceFrame, p_position: (f64, f64, f64), p_rotation: (f64, f64, f64, f64), p_visible: bool) -> CallHandle<Text> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("Drawing"));
    proc_call.set_procedure(String::from("AddText"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_text.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_position.encode_to_bytes().unwrap());
    arguments.push(arg2);

    let mut arg3 = krpc::Argument::new();
    arg3.set_position(3);
    arg3.set_value(p_rotation.encode_to_bytes().unwrap());
    arguments.push(arg3);

    let mut arg4 = krpc::Argument::new();
    arg4.set_position(4);
    arg4.set_value(p_visible.encode_to_bytes().unwrap());
    arguments.push(arg4);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Remove all objects being drawn. </summary> <param name="clientOnly">If true, only remove objects created by the calling client.</param> </doc>
pub fn clear(p_client_only: bool) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("Drawing"));
    proc_call.set_procedure(String::from("Clear"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_client_only.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> A list of all available fonts. </summary> </doc>
pub fn text_static_available_fonts() -> CallHandle<Vec<String>> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("Drawing"));
    proc_call.set_procedure(String::from("Text_static_AvailableFonts"));

    CallHandle::new(proc_call)
}


impl Line {
    /// <doc> <summary> Remove the object. </summary> </doc>
    pub fn remove(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Line_Remove"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Set the color </summary> </doc>
    pub fn get_color(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Line_get_Color"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> End position of the line. </summary> </doc>
    pub fn get_end(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Line_get_End"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Material used to render the object. Creates the material from a shader with the given name. </summary> </doc>
    pub fn get_material(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Line_get_Material"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Reference frame for the positions of the object. </summary> </doc>
    pub fn get_reference_frame(&self, ) -> CallHandle<space_center::ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Line_get_ReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Start position of the line. </summary> </doc>
    pub fn get_start(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Line_get_Start"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Set the thickness </summary> </doc>
    pub fn get_thickness(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Line_get_Thickness"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the object is visible. </summary> </doc>
    pub fn get_visible(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Line_get_Visible"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Set the color </summary> </doc>
    pub fn set_color(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Line_set_Color"));

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

    /// <doc> <summary> End position of the line. </summary> </doc>
    pub fn set_end(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Line_set_End"));

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

    /// <doc> <summary> Material used to render the object. Creates the material from a shader with the given name. </summary> </doc>
    pub fn set_material(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Line_set_Material"));

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

    /// <doc> <summary> Reference frame for the positions of the object. </summary> </doc>
    pub fn set_reference_frame(&self, p_value: &space_center::ReferenceFrame) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Line_set_ReferenceFrame"));

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

    /// <doc> <summary> Start position of the line. </summary> </doc>
    pub fn set_start(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Line_set_Start"));

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

    /// <doc> <summary> Set the thickness </summary> </doc>
    pub fn set_thickness(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Line_set_Thickness"));

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

    /// <doc> <summary> Whether the object is visible. </summary> </doc>
    pub fn set_visible(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Line_set_Visible"));

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
impl Polygon {
    /// <doc> <summary> Remove the object. </summary> </doc>
    pub fn remove(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Polygon_Remove"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Set the color </summary> </doc>
    pub fn get_color(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Polygon_get_Color"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Material used to render the object. Creates the material from a shader with the given name. </summary> </doc>
    pub fn get_material(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Polygon_get_Material"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Reference frame for the positions of the object. </summary> </doc>
    pub fn get_reference_frame(&self, ) -> CallHandle<space_center::ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Polygon_get_ReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Set the thickness </summary> </doc>
    pub fn get_thickness(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Polygon_get_Thickness"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Vertices for the polygon. </summary> </doc>
    pub fn get_vertices(&self, ) -> CallHandle<Vec<(f64, f64, f64)>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Polygon_get_Vertices"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the object is visible. </summary> </doc>
    pub fn get_visible(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Polygon_get_Visible"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Set the color </summary> </doc>
    pub fn set_color(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Polygon_set_Color"));

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

    /// <doc> <summary> Material used to render the object. Creates the material from a shader with the given name. </summary> </doc>
    pub fn set_material(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Polygon_set_Material"));

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

    /// <doc> <summary> Reference frame for the positions of the object. </summary> </doc>
    pub fn set_reference_frame(&self, p_value: &space_center::ReferenceFrame) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Polygon_set_ReferenceFrame"));

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

    /// <doc> <summary> Set the thickness </summary> </doc>
    pub fn set_thickness(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Polygon_set_Thickness"));

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

    /// <doc> <summary> Vertices for the polygon. </summary> </doc>
    pub fn set_vertices(&self, p_value: &Vec<(f64, f64, f64)>) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Polygon_set_Vertices"));

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

    /// <doc> <summary> Whether the object is visible. </summary> </doc>
    pub fn set_visible(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Polygon_set_Visible"));

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
impl Text {
    /// <doc> <summary> Remove the object. </summary> </doc>
    pub fn remove(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_Remove"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Alignment. </summary> </doc>
    pub fn get_alignment(&self, ) -> CallHandle<ui::TextAlignment> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_get_Alignment"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Anchor. </summary> </doc>
    pub fn get_anchor(&self, ) -> CallHandle<ui::TextAnchor> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_get_Anchor"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Character size. </summary> </doc>
    pub fn get_character_size(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_get_CharacterSize"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Set the color </summary> </doc>
    pub fn get_color(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_get_Color"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The text string </summary> </doc>
    pub fn get_content(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_get_Content"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Name of the font </summary> </doc>
    pub fn get_font(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_get_Font"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Line spacing. </summary> </doc>
    pub fn get_line_spacing(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_get_LineSpacing"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Material used to render the object. Creates the material from a shader with the given name. </summary> </doc>
    pub fn get_material(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_get_Material"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Position of the text. </summary> </doc>
    pub fn get_position(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_get_Position"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Reference frame for the positions of the object. </summary> </doc>
    pub fn get_reference_frame(&self, ) -> CallHandle<space_center::ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_get_ReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Rotation of the text as a quaternion. </summary> </doc>
    pub fn get_rotation(&self, ) -> CallHandle<(f64, f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_get_Rotation"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Font size. </summary> </doc>
    pub fn get_size(&self, ) -> CallHandle<i32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_get_Size"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Font style. </summary> </doc>
    pub fn get_style(&self, ) -> CallHandle<ui::FontStyle> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_get_Style"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the object is visible. </summary> </doc>
    pub fn get_visible(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_get_Visible"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Alignment. </summary> </doc>
    pub fn set_alignment(&self, p_value: ui::TextAlignment) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_set_Alignment"));

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

    /// <doc> <summary> Anchor. </summary> </doc>
    pub fn set_anchor(&self, p_value: ui::TextAnchor) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_set_Anchor"));

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

    /// <doc> <summary> Character size. </summary> </doc>
    pub fn set_character_size(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_set_CharacterSize"));

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

    /// <doc> <summary> Set the color </summary> </doc>
    pub fn set_color(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_set_Color"));

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

    /// <doc> <summary> The text string </summary> </doc>
    pub fn set_content(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_set_Content"));

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

    /// <doc> <summary> Name of the font </summary> </doc>
    pub fn set_font(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_set_Font"));

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

    /// <doc> <summary> Line spacing. </summary> </doc>
    pub fn set_line_spacing(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_set_LineSpacing"));

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

    /// <doc> <summary> Material used to render the object. Creates the material from a shader with the given name. </summary> </doc>
    pub fn set_material(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_set_Material"));

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

    /// <doc> <summary> Position of the text. </summary> </doc>
    pub fn set_position(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_set_Position"));

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

    /// <doc> <summary> Reference frame for the positions of the object. </summary> </doc>
    pub fn set_reference_frame(&self, p_value: &space_center::ReferenceFrame) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_set_ReferenceFrame"));

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

    /// <doc> <summary> Rotation of the text as a quaternion. </summary> </doc>
    pub fn set_rotation(&self, p_value: (f64, f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_set_Rotation"));

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

    /// <doc> <summary> Font size. </summary> </doc>
    pub fn set_size(&self, p_value: i32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_set_Size"));

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

    /// <doc> <summary> Font style. </summary> </doc>
    pub fn set_style(&self, p_value: ui::FontStyle) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_set_Style"));

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

    /// <doc> <summary> Whether the object is visible. </summary> </doc>
    pub fn set_visible(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("Drawing"));
        proc_call.set_procedure(String::from("Text_set_Visible"));

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

