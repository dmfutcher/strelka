use krpc_mars::krpc as krpc;
use krpc_mars::protobuf as protobuf;
use krpc_mars::CallHandle;
use krpc_mars::codec::RPCEncodable;
use krpc_mars::codec::RPCExtractable;


use std::collections::HashMap;
use std::collections::HashSet;

use std::fmt;



#[derive(Clone)]
pub struct Expression {
    id: u32,
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expression({})", self.id)
    }
}

impl RPCEncodable for Expression {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Expression {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Expression { id })
    }
}


#[derive(Clone)]
pub struct Type {
    id: u32,
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type({})", self.id)
    }
}

impl RPCEncodable for Type {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Type {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Type { id })
    }
}




#[derive(Debug, Copy, Clone)]
pub enum GameScene {
    SpaceCenter = 0,
    Flight = 1,
    TrackingStation = 2,
    EditorVAB = 3,
    EditorSPH = 4,
}

impl From<i32> for GameScene {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => GameScene::SpaceCenter,
            1 => GameScene::Flight,
            2 => GameScene::TrackingStation,
            3 => GameScene::EditorVAB,
            4 => GameScene::EditorSPH,
            _ => panic!("Could not convert '{}' to a KRPC::GameScene", source),
        }
    }
}

impl RPCEncodable for GameScene {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for GameScene {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(GameScene::from(value))
    }
}



/// <doc> <summary> Create an event from a server side expression. </summary> </doc>
pub fn add_event(p_expression: &Expression) -> CallHandle<::krpc_mars::krpc::Event> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("AddEvent"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_expression.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Add a streaming request and return its identifier. </summary> </doc>
pub fn add_stream(p_call: ::krpc_mars::krpc::ProcedureCall, p_start: bool) -> CallHandle<::krpc_mars::krpc::Stream> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("AddStream"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_call.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_start.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Numerical addition. </summary> <param name="arg0"></param> <param name="arg1"></param> </doc>
pub fn expression_static_add(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Add"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Applies an accumulator function over a sequence. </summary> <returns>The accumulated value.</returns> <param name="arg">The collection.</param> <param name="func">The accumulator function.</param> </doc>
pub fn expression_static_aggregate(p_arg: &Expression, p_func: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Aggregate"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_func.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Applies an accumulator function over a sequence, with a given seed. </summary> <returns>The accumulated value.</returns> <param name="arg">The collection.</param> <param name="seed">The seed value.</param> <param name="func">The accumulator function.</param> </doc>
pub fn expression_static_aggregate_with_seed(p_arg: &Expression, p_seed: &Expression, p_func: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_AggregateWithSeed"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_seed.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_func.encode_to_bytes().unwrap());
    arguments.push(arg2);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Determine whether all items in a collection satisfy a boolean predicate. </summary> <returns>Whether all items satisfy the predicate.</returns> <param name="arg">The collection.</param> <param name="predicate">The predicate function.</param> </doc>
pub fn expression_static_all(p_arg: &Expression, p_predicate: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_All"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_predicate.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Boolean and operator. </summary> <param name="arg0"></param> <param name="arg1"></param> </doc>
pub fn expression_static_and(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_And"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Determine whether any item in a collection satisfies a boolean predicate. </summary> <returns>Whether any item satisfies the predicate.</returns> <param name="arg">The collection.</param> <param name="predicate">The predicate function.</param> </doc>
pub fn expression_static_any(p_arg: &Expression, p_predicate: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Any"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_predicate.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Minimum of all elements in a collection. </summary> <returns>The minimum elements in the collection.</returns> <param name="arg">The list or set.</param> </doc>
pub fn expression_static_average(p_arg: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Average"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> An RPC call. </summary> <param name="call"></param> </doc>
pub fn expression_static_call(p_call: ::krpc_mars::krpc::ProcedureCall) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Call"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_call.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Perform a cast to the given type. </summary> <param name="arg"></param> <param name="type">Type to cast the argument to.</param> </doc>
pub fn expression_static_cast(p_arg: &Expression, p_type: &Type) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Cast"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_type.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Concatenate two sequences. </summary> <returns>The first sequence followed by the second sequence.</returns> <param name="arg1">The first sequence.</param> <param name="arg2">The second sequence.</param> </doc>
pub fn expression_static_concat(p_arg1: &Expression, p_arg2: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Concat"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg2.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> A constant value of boolean type. </summary> <param name="value"></param> </doc>
pub fn expression_static_constant_bool(p_value: bool) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_ConstantBool"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_value.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> A constant value of double precision floating point type. </summary> <param name="value"></param> </doc>
pub fn expression_static_constant_double(p_value: f64) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_ConstantDouble"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_value.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> A constant value of single precision floating point type. </summary> <param name="value"></param> </doc>
pub fn expression_static_constant_float(p_value: f32) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_ConstantFloat"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_value.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> A constant value of integer type. </summary> <param name="value"></param> </doc>
pub fn expression_static_constant_int(p_value: i32) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_ConstantInt"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_value.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> A constant value of string type. </summary> <param name="value"></param> </doc>
pub fn expression_static_constant_string(p_value: String) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_ConstantString"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_value.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Determine if a collection contains a value. </summary> <returns>Whether the collection contains a value.</returns> <param name="arg">The collection.</param> <param name="value">The value to look for.</param> </doc>
pub fn expression_static_contains(p_arg: &Expression, p_value: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Contains"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_value.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Number of elements in a collection. </summary> <returns>The number of elements in the collection.</returns> <param name="arg">The list, set or dictionary.</param> </doc>
pub fn expression_static_count(p_arg: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Count"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Construct a dictionary, from a list of corresponding keys and values. </summary> <returns>The dictionary.</returns> <param name="keys">The keys. Should all be of the same type.</param> <param name="values">The values. Should all be of the same type.</param> </doc>
pub fn expression_static_create_dictionary(p_keys: &Vec<Expression>, p_values: &Vec<Expression>) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_CreateDictionary"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_keys.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_values.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Construct a list. </summary> <returns>The list.</returns> <param name="values">The value. Should all be of the same type.</param> </doc>
pub fn expression_static_create_list(p_values: &Vec<Expression>) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_CreateList"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_values.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Construct a set. </summary> <returns>The set.</returns> <param name="values">The values. Should all be of the same type.</param> </doc>
pub fn expression_static_create_set(p_values: &HashSet<Expression>) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_CreateSet"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_values.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Construct a tuple. </summary> <returns>The tuple.</returns> <param name="elements">The elements.</param> </doc>
pub fn expression_static_create_tuple(p_elements: &Vec<Expression>) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_CreateTuple"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_elements.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Numerical division. </summary> <param name="arg0"></param> <param name="arg1"></param> </doc>
pub fn expression_static_divide(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Divide"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Equality comparison. </summary> <param name="arg0"></param> <param name="arg1"></param> </doc>
pub fn expression_static_equal(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Equal"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Boolean exclusive-or operator. </summary> <param name="arg0"></param> <param name="arg1"></param> </doc>
pub fn expression_static_exclusive_or(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_ExclusiveOr"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> A function. </summary> <returns>A function.</returns> <param name="parameters">The parameters of the function.</param> <param name="body">The body of the function.</param> </doc>
pub fn expression_static_function(p_parameters: &Vec<Expression>, p_body: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Function"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_parameters.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_body.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Access an element in a tuple, list or dictionary. </summary> <returns>The element.</returns> <param name="arg">The tuple, list or dictionary.</param> <param name="index">The index of the element to access. A zero indexed integer for a tuple or list, or a key for a dictionary.</param> </doc>
pub fn expression_static_get(p_arg: &Expression, p_index: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Get"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_index.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Greater than numerical comparison. </summary> <param name="arg0"></param> <param name="arg1"></param> </doc>
pub fn expression_static_greater_than(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_GreaterThan"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Greater than or equal numerical comparison. </summary> <param name="arg0"></param> <param name="arg1"></param> </doc>
pub fn expression_static_greater_than_or_equal(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_GreaterThanOrEqual"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> A function call. </summary> <returns>A function call.</returns> <param name="function">The function to call.</param> <param name="args">The arguments to call the function with.</param> </doc>
pub fn expression_static_invoke(p_function: &Expression, p_args: &HashMap<String, Expression>) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Invoke"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_function.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_args.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Bitwise left shift. </summary> <param name="arg0"></param> <param name="arg1"></param> </doc>
pub fn expression_static_left_shift(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_LeftShift"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Less than numerical comparison. </summary> <param name="arg0"></param> <param name="arg1"></param> </doc>
pub fn expression_static_less_than(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_LessThan"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Less than or equal numerical comparison. </summary> <param name="arg0"></param> <param name="arg1"></param> </doc>
pub fn expression_static_less_than_or_equal(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_LessThanOrEqual"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Maximum of all elements in a collection. </summary> <returns>The maximum elements in the collection.</returns> <param name="arg">The list or set.</param> </doc>
pub fn expression_static_max(p_arg: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Max"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Minimum of all elements in a collection. </summary> <returns>The minimum elements in the collection.</returns> <param name="arg">The list or set.</param> </doc>
pub fn expression_static_min(p_arg: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Min"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Numerical modulo operator. </summary> <param name="arg0"></param> <param name="arg1"></param> <returns>The remainder of arg0 divided by arg1</returns> </doc>
pub fn expression_static_modulo(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Modulo"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Numerical multiplication. </summary> <param name="arg0"></param> <param name="arg1"></param> </doc>
pub fn expression_static_multiply(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Multiply"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Boolean negation operator. </summary> <param name="arg"></param> </doc>
pub fn expression_static_not(p_arg: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Not"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Inequality comparison. </summary> <param name="arg0"></param> <param name="arg1"></param> </doc>
pub fn expression_static_not_equal(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_NotEqual"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Boolean or operator. </summary> <param name="arg0"></param> <param name="arg1"></param> </doc>
pub fn expression_static_or(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Or"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Order a collection using a key function. </summary> <returns>The ordered collection.</returns> <param name="arg">The collection to order.</param> <param name="key">A function that takes a value from the collection and generates a key to sort on.</param> </doc>
pub fn expression_static_order_by(p_arg: &Expression, p_key: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_OrderBy"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_key.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> A named parameter of type double. </summary> <returns>A named parameter.</returns> <param name="name">The name of the parameter.</param> <param name="type">The type of the parameter.</param> </doc>
pub fn expression_static_parameter(p_name: String, p_type: &Type) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Parameter"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_name.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_type.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Numerical power operator. </summary> <param name="arg0"></param> <param name="arg1"></param> <returns>arg0 raised to the power of arg1, with type of arg0</returns> </doc>
pub fn expression_static_power(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Power"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Bitwise right shift. </summary> <param name="arg0"></param> <param name="arg1"></param> </doc>
pub fn expression_static_right_shift(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_RightShift"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Run a function on every element in the collection. </summary> <returns>The modified collection.</returns> <param name="arg">The list or set.</param> <param name="func">The function.</param> </doc>
pub fn expression_static_select(p_arg: &Expression, p_func: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Select"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_func.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Numerical subtraction. </summary> <param name="arg0"></param> <param name="arg1"></param> </doc>
pub fn expression_static_subtract(p_arg0: &Expression, p_arg1: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Subtract"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg0.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_arg1.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Sum all elements of a collection. </summary> <returns>The sum of the elements in the collection.</returns> <param name="arg">The list or set.</param> </doc>
pub fn expression_static_sum(p_arg: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Sum"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Convert a collection to a list. </summary> <returns>The collection as a list.</returns> <param name="arg">The collection.</param> </doc>
pub fn expression_static_to_list(p_arg: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_ToList"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Convert a collection to a set. </summary> <returns>The collection as a set.</returns> <param name="arg">The collection.</param> </doc>
pub fn expression_static_to_set(p_arg: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_ToSet"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Run a function on every element in the collection. </summary> <returns>The modified collection.</returns> <param name="arg">The list or set.</param> <param name="func">The function.</param> </doc>
pub fn expression_static_where(p_arg: &Expression, p_func: &Expression) -> CallHandle<Expression> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Expression_static_Where"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_arg.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_func.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Returns the identifier for the current client. </summary> </doc>
pub fn get_client_id() -> CallHandle<&[u8]> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("GetClientID"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Returns the name of the current client. This is an empty string if the client has no name. </summary> </doc>
pub fn get_client_name() -> CallHandle<String> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("GetClientName"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Returns information on all services, procedures, classes, properties etc. provided by the server. Can be used by client libraries to automatically create functionality such as stubs. </summary> </doc>
pub fn get_services() -> CallHandle<::krpc_mars::krpc::Services> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("GetServices"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Returns some information about the server, such as the version. </summary> </doc>
pub fn get_status() -> CallHandle<::krpc_mars::krpc::Status> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("GetStatus"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Remove a streaming request. </summary> </doc>
pub fn remove_stream(p_id: u64) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("RemoveStream"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_id.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Set the update rate for a stream in Hz. </summary> </doc>
pub fn set_stream_rate(p_id: u64, p_rate: f32) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("SetStreamRate"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_id.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_rate.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Start a previously added streaming request. </summary> </doc>
pub fn start_stream(p_id: u64) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("StartStream"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_id.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Bool type. </summary> </doc>
pub fn type_static_bool() -> CallHandle<Type> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Type_static_Bool"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Double type. </summary> </doc>
pub fn type_static_double() -> CallHandle<Type> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Type_static_Double"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Float type. </summary> </doc>
pub fn type_static_float() -> CallHandle<Type> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Type_static_Float"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Int type. </summary> </doc>
pub fn type_static_int() -> CallHandle<Type> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Type_static_Int"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> String type. </summary> </doc>
pub fn type_static_string() -> CallHandle<Type> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("Type_static_String"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> A list of RPC clients that are currently connected to the server. Each entry in the list is a clients identifier, name and address. </summary> </doc>
pub fn get_clients() -> CallHandle<Vec<(&[u8], String, String)>> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("get_Clients"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Get the current game scene. </summary> </doc>
pub fn get_current_game_scene() -> CallHandle<GameScene> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("get_CurrentGameScene"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Whether the game is paused. </summary> </doc>
pub fn get_paused() -> CallHandle<bool> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("get_Paused"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Whether the game is paused. </summary> </doc>
pub fn set_paused(p_value: bool) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("KRPC"));
    proc_call.set_procedure(String::from("set_Paused"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_value.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}



