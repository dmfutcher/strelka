use krpc_mars::krpc as krpc;
use krpc_mars::protobuf as protobuf;
use krpc_mars::CallHandle;
use krpc_mars::codec::RPCEncodable;
use krpc_mars::codec::RPCExtractable;



use std::fmt;



#[derive(Clone)]
pub struct Button {
    id: u32,
}

impl fmt::Debug for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Button({})", self.id)
    }
}

impl RPCEncodable for Button {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Button {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Button { id })
    }
}


#[derive(Clone)]
pub struct Canvas {
    id: u32,
}

impl fmt::Debug for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Canvas({})", self.id)
    }
}

impl RPCEncodable for Canvas {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Canvas {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Canvas { id })
    }
}


#[derive(Clone)]
pub struct InputField {
    id: u32,
}

impl fmt::Debug for InputField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InputField({})", self.id)
    }
}

impl RPCEncodable for InputField {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for InputField {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(InputField { id })
    }
}


#[derive(Clone)]
pub struct Panel {
    id: u32,
}

impl fmt::Debug for Panel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Panel({})", self.id)
    }
}

impl RPCEncodable for Panel {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Panel {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Panel { id })
    }
}


#[derive(Clone)]
pub struct RectTransform {
    id: u32,
}

impl fmt::Debug for RectTransform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RectTransform({})", self.id)
    }
}

impl RPCEncodable for RectTransform {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for RectTransform {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(RectTransform { id })
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




#[derive(Debug, Copy, Clone)]
pub enum FontStyle {
    Normal = 0,
    Bold = 1,
    Italic = 2,
    BoldAndItalic = 3,
}

impl From<i32> for FontStyle {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => FontStyle::Normal,
            1 => FontStyle::Bold,
            2 => FontStyle::Italic,
            3 => FontStyle::BoldAndItalic,
            _ => panic!("Could not convert '{}' to a UI::FontStyle", source),
        }
    }
}

impl RPCEncodable for FontStyle {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for FontStyle {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(FontStyle::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MessagePosition {
    BottomCenter = 0,
    TopCenter = 1,
    TopLeft = 2,
    TopRight = 3,
}

impl From<i32> for MessagePosition {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => MessagePosition::BottomCenter,
            1 => MessagePosition::TopCenter,
            2 => MessagePosition::TopLeft,
            3 => MessagePosition::TopRight,
            _ => panic!("Could not convert '{}' to a UI::MessagePosition", source),
        }
    }
}

impl RPCEncodable for MessagePosition {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for MessagePosition {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(MessagePosition::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TextAlignment {
    Left = 0,
    Right = 1,
    Center = 2,
}

impl From<i32> for TextAlignment {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => TextAlignment::Left,
            1 => TextAlignment::Right,
            2 => TextAlignment::Center,
            _ => panic!("Could not convert '{}' to a UI::TextAlignment", source),
        }
    }
}

impl RPCEncodable for TextAlignment {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for TextAlignment {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(TextAlignment::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TextAnchor {
    LowerCenter = 0,
    LowerLeft = 1,
    LowerRight = 2,
    MiddleCenter = 3,
    MiddleLeft = 4,
    MiddleRight = 5,
    UpperCenter = 6,
    UpperLeft = 7,
    UpperRight = 8,
}

impl From<i32> for TextAnchor {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => TextAnchor::LowerCenter,
            1 => TextAnchor::LowerLeft,
            2 => TextAnchor::LowerRight,
            3 => TextAnchor::MiddleCenter,
            4 => TextAnchor::MiddleLeft,
            5 => TextAnchor::MiddleRight,
            6 => TextAnchor::UpperCenter,
            7 => TextAnchor::UpperLeft,
            8 => TextAnchor::UpperRight,
            _ => panic!("Could not convert '{}' to a UI::TextAnchor", source),
        }
    }
}

impl RPCEncodable for TextAnchor {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for TextAnchor {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(TextAnchor::from(value))
    }
}



/// <doc> <summary> Add a new canvas. </summary> <remarks> If you want to add UI elements to KSPs stock UI canvas, use <see cref="M:UI.StockCanvas" />. </remarks> </doc>
pub fn add_canvas() -> CallHandle<Canvas> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("UI"));
    proc_call.set_procedure(String::from("AddCanvas"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Remove all user interface elements. </summary> <param name="clientOnly">If true, only remove objects created by the calling client.</param> </doc>
pub fn clear(p_client_only: bool) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("UI"));
    proc_call.set_procedure(String::from("Clear"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_client_only.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Display a message on the screen. </summary> <remarks> The message appears just like a stock message, for example quicksave or quickload messages. </remarks> <param name="content">Message content.</param> <param name="duration">Duration before the message disappears, in seconds.</param> <param name="position">Position to display the message.</param> <param name="size">Size of the message, differs per position.</param> <param name="color">The color of the message.</param> </doc>
pub fn message(p_content: String, p_duration: f32, p_position: MessagePosition, p_color: (f64, f64, f64), p_size: f32) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("UI"));
    proc_call.set_procedure(String::from("Message"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_content.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_duration.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_position.encode_to_bytes().unwrap());
    arguments.push(arg2);

    let mut arg3 = krpc::Argument::new();
    arg3.set_position(3);
    arg3.set_value(p_color.encode_to_bytes().unwrap());
    arguments.push(arg3);

    let mut arg4 = krpc::Argument::new();
    arg4.set_position(4);
    arg4.set_value(p_size.encode_to_bytes().unwrap());
    arguments.push(arg4);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> The stock UI canvas. </summary> </doc>
pub fn get_stock_canvas() -> CallHandle<Canvas> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("UI"));
    proc_call.set_procedure(String::from("get_StockCanvas"));

    CallHandle::new(proc_call)
}


impl Button {
    /// <doc> <summary> Remove the UI object. </summary> </doc>
    pub fn remove(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Button_Remove"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the button has been clicked. </summary> <remarks> This property is set to true when the user clicks the button. A client script should reset the property to false in order to detect subsequent button presses. </remarks> </doc>
    pub fn get_clicked(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Button_get_Clicked"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rect transform for the text. </summary> </doc>
    pub fn get_rect_transform(&self, ) -> CallHandle<RectTransform> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Button_get_RectTransform"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The text for the button. </summary> </doc>
    pub fn get_text(&self, ) -> CallHandle<Text> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Button_get_Text"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the UI object is visible. </summary> </doc>
    pub fn get_visible(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Button_get_Visible"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the button has been clicked. </summary> <remarks> This property is set to true when the user clicks the button. A client script should reset the property to false in order to detect subsequent button presses. </remarks> </doc>
    pub fn set_clicked(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Button_set_Clicked"));

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

    /// <doc> <summary> Whether the UI object is visible. </summary> </doc>
    pub fn set_visible(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Button_set_Visible"));

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
impl Canvas {
    /// <doc> <summary> Add a button to the canvas. </summary> <param name="content">The label for the button.</param> <param name="visible">Whether the button is visible.</param> </doc>
    pub fn add_button(&self, p_content: String, p_visible: bool) -> CallHandle<Button> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Canvas_AddButton"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_content.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_visible.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Add an input field to the canvas. </summary> <param name="visible">Whether the input field is visible.</param> </doc>
    pub fn add_input_field(&self, p_visible: bool) -> CallHandle<InputField> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Canvas_AddInputField"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_visible.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Create a new container for user interface elements. </summary> <param name="visible">Whether the panel is visible.</param> </doc>
    pub fn add_panel(&self, p_visible: bool) -> CallHandle<Panel> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Canvas_AddPanel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_visible.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Add text to the canvas. </summary> <param name="content">The text.</param> <param name="visible">Whether the text is visible.</param> </doc>
    pub fn add_text(&self, p_content: String, p_visible: bool) -> CallHandle<Text> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Canvas_AddText"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_content.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_visible.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Remove the UI object. </summary> </doc>
    pub fn remove(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Canvas_Remove"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rect transform for the canvas. </summary> </doc>
    pub fn get_rect_transform(&self, ) -> CallHandle<RectTransform> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Canvas_get_RectTransform"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the UI object is visible. </summary> </doc>
    pub fn get_visible(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Canvas_get_Visible"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the UI object is visible. </summary> </doc>
    pub fn set_visible(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Canvas_set_Visible"));

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
impl InputField {
    /// <doc> <summary> Remove the UI object. </summary> </doc>
    pub fn remove(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("InputField_Remove"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the input field has been changed. </summary> <remarks> This property is set to true when the user modifies the value of the input field. A client script should reset the property to false in order to detect subsequent changes. </remarks> </doc>
    pub fn get_changed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("InputField_get_Changed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rect transform for the input field. </summary> </doc>
    pub fn get_rect_transform(&self, ) -> CallHandle<RectTransform> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("InputField_get_RectTransform"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The text component of the input field. </summary> <remarks> Use <see cref="M:UI.InputField.Value" /> to get and set the value in the field. This object can be used to alter the style of the input field's text. </remarks> </doc>
    pub fn get_text(&self, ) -> CallHandle<Text> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("InputField_get_Text"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The value of the input field. </summary> </doc>
    pub fn get_value(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("InputField_get_Value"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the UI object is visible. </summary> </doc>
    pub fn get_visible(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("InputField_get_Visible"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the input field has been changed. </summary> <remarks> This property is set to true when the user modifies the value of the input field. A client script should reset the property to false in order to detect subsequent changes. </remarks> </doc>
    pub fn set_changed(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("InputField_set_Changed"));

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

    /// <doc> <summary> The value of the input field. </summary> </doc>
    pub fn set_value(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("InputField_set_Value"));

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

    /// <doc> <summary> Whether the UI object is visible. </summary> </doc>
    pub fn set_visible(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("InputField_set_Visible"));

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
impl Panel {
    /// <doc> <summary> Add a button to the panel. </summary> <param name="content">The label for the button.</param> <param name="visible">Whether the button is visible.</param> </doc>
    pub fn add_button(&self, p_content: String, p_visible: bool) -> CallHandle<Button> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Panel_AddButton"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_content.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_visible.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Add an input field to the panel. </summary> <param name="visible">Whether the input field is visible.</param> </doc>
    pub fn add_input_field(&self, p_visible: bool) -> CallHandle<InputField> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Panel_AddInputField"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_visible.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Create a panel within this panel. </summary> <param name="visible">Whether the new panel is visible.</param> </doc>
    pub fn add_panel(&self, p_visible: bool) -> CallHandle<Panel> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Panel_AddPanel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_visible.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Add text to the panel. </summary> <param name="content">The text.</param> <param name="visible">Whether the text is visible.</param> </doc>
    pub fn add_text(&self, p_content: String, p_visible: bool) -> CallHandle<Text> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Panel_AddText"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_content.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_visible.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Remove the UI object. </summary> </doc>
    pub fn remove(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Panel_Remove"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rect transform for the panel. </summary> </doc>
    pub fn get_rect_transform(&self, ) -> CallHandle<RectTransform> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Panel_get_RectTransform"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the UI object is visible. </summary> </doc>
    pub fn get_visible(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Panel_get_Visible"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the UI object is visible. </summary> </doc>
    pub fn set_visible(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Panel_set_Visible"));

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
impl RectTransform {
    /// <doc> <summary> The anchor point for the lower left corner of the rectangle defined as a fraction of the size of the parent rectangle. </summary> </doc>
    pub fn get_anchor_max(&self, ) -> CallHandle<(f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_get_AnchorMax"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The anchor point for the upper right corner of the rectangle defined as a fraction of the size of the parent rectangle. </summary> </doc>
    pub fn get_anchor_min(&self, ) -> CallHandle<(f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_get_AnchorMin"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Position of the rectangles pivot point relative to the anchors. </summary> </doc>
    pub fn get_local_position(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_get_LocalPosition"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Position of the rectangles lower left corner relative to the anchors. </summary> </doc>
    pub fn get_lower_left(&self, ) -> CallHandle<(f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_get_LowerLeft"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Location of the pivot point around which the rectangle rotates, defined as a fraction of the size of the rectangle itself. </summary> </doc>
    pub fn get_pivot(&self, ) -> CallHandle<(f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_get_Pivot"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Position of the rectangles pivot point relative to the anchors. </summary> </doc>
    pub fn get_position(&self, ) -> CallHandle<(f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_get_Position"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Rotation, as a quaternion, of the object around its pivot point. </summary> </doc>
    pub fn get_rotation(&self, ) -> CallHandle<(f64, f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_get_Rotation"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Scale factor applied to the object in the x, y and z dimensions. </summary> </doc>
    pub fn get_scale(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_get_Scale"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Width and height of the rectangle. </summary> </doc>
    pub fn get_size(&self, ) -> CallHandle<(f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_get_Size"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Position of the rectangles upper right corner relative to the anchors. </summary> </doc>
    pub fn get_upper_right(&self, ) -> CallHandle<(f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_get_UpperRight"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Set the minimum and maximum anchor points as a fraction of the size of the parent rectangle. </summary> </doc>
    pub fn set_anchor(&self, p_value: (f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_set_Anchor"));

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

    /// <doc> <summary> The anchor point for the lower left corner of the rectangle defined as a fraction of the size of the parent rectangle. </summary> </doc>
    pub fn set_anchor_max(&self, p_value: (f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_set_AnchorMax"));

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

    /// <doc> <summary> The anchor point for the upper right corner of the rectangle defined as a fraction of the size of the parent rectangle. </summary> </doc>
    pub fn set_anchor_min(&self, p_value: (f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_set_AnchorMin"));

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

    /// <doc> <summary> Position of the rectangles pivot point relative to the anchors. </summary> </doc>
    pub fn set_local_position(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_set_LocalPosition"));

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

    /// <doc> <summary> Position of the rectangles lower left corner relative to the anchors. </summary> </doc>
    pub fn set_lower_left(&self, p_value: (f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_set_LowerLeft"));

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

    /// <doc> <summary> Location of the pivot point around which the rectangle rotates, defined as a fraction of the size of the rectangle itself. </summary> </doc>
    pub fn set_pivot(&self, p_value: (f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_set_Pivot"));

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

    /// <doc> <summary> Position of the rectangles pivot point relative to the anchors. </summary> </doc>
    pub fn set_position(&self, p_value: (f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_set_Position"));

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

    /// <doc> <summary> Rotation, as a quaternion, of the object around its pivot point. </summary> </doc>
    pub fn set_rotation(&self, p_value: (f64, f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_set_Rotation"));

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

    /// <doc> <summary> Scale factor applied to the object in the x, y and z dimensions. </summary> </doc>
    pub fn set_scale(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_set_Scale"));

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

    /// <doc> <summary> Width and height of the rectangle. </summary> </doc>
    pub fn set_size(&self, p_value: (f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_set_Size"));

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

    /// <doc> <summary> Position of the rectangles upper right corner relative to the anchors. </summary> </doc>
    pub fn set_upper_right(&self, p_value: (f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("RectTransform_set_UpperRight"));

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
    /// <doc> <summary> Remove the UI object. </summary> </doc>
    pub fn remove(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
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
    pub fn get_alignment(&self, ) -> CallHandle<TextAnchor> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Text_get_Alignment"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all available fonts. </summary> </doc>
    pub fn get_available_fonts(&self, ) -> CallHandle<Vec<String>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Text_get_AvailableFonts"));

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
        proc_call.set_service(String::from("UI"));
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
        proc_call.set_service(String::from("UI"));
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
        proc_call.set_service(String::from("UI"));
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
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Text_get_LineSpacing"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rect transform for the text. </summary> </doc>
    pub fn get_rect_transform(&self, ) -> CallHandle<RectTransform> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Text_get_RectTransform"));

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
        proc_call.set_service(String::from("UI"));
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
    pub fn get_style(&self, ) -> CallHandle<FontStyle> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
        proc_call.set_procedure(String::from("Text_get_Style"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the UI object is visible. </summary> </doc>
    pub fn get_visible(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
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
    pub fn set_alignment(&self, p_value: TextAnchor) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
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

    /// <doc> <summary> Set the color </summary> </doc>
    pub fn set_color(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
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
        proc_call.set_service(String::from("UI"));
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
        proc_call.set_service(String::from("UI"));
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
        proc_call.set_service(String::from("UI"));
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

    /// <doc> <summary> Font size. </summary> </doc>
    pub fn set_size(&self, p_value: i32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
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
    pub fn set_style(&self, p_value: FontStyle) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
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

    /// <doc> <summary> Whether the UI object is visible. </summary> </doc>
    pub fn set_visible(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("UI"));
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

