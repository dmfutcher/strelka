use krpc_mars::krpc as krpc;
use krpc_mars::protobuf as protobuf;
use krpc_mars::CallHandle;
use krpc_mars::codec::RPCEncodable;
use krpc_mars::codec::RPCExtractable;


use std::collections::HashSet;
use std::collections::HashMap;

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
pub struct AutoPilot {
    id: u32,
}

impl fmt::Debug for AutoPilot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AutoPilot({})", self.id)
    }
}

impl RPCEncodable for AutoPilot {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for AutoPilot {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(AutoPilot { id })
    }
}


#[derive(Clone)]
pub struct Camera {
    id: u32,
}

impl fmt::Debug for Camera {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Camera({})", self.id)
    }
}

impl RPCEncodable for Camera {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Camera {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Camera { id })
    }
}


#[derive(Clone)]
pub struct CargoBay {
    id: u32,
}

impl fmt::Debug for CargoBay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CargoBay({})", self.id)
    }
}

impl RPCEncodable for CargoBay {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for CargoBay {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(CargoBay { id })
    }
}


#[derive(Clone)]
pub struct CelestialBody {
    id: u32,
}

impl fmt::Debug for CelestialBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CelestialBody({})", self.id)
    }
}

impl RPCEncodable for CelestialBody {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for CelestialBody {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(CelestialBody { id })
    }
}


#[derive(Clone)]
pub struct CommLink {
    id: u32,
}

impl fmt::Debug for CommLink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CommLink({})", self.id)
    }
}

impl RPCEncodable for CommLink {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for CommLink {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(CommLink { id })
    }
}


#[derive(Clone)]
pub struct CommNode {
    id: u32,
}

impl fmt::Debug for CommNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CommNode({})", self.id)
    }
}

impl RPCEncodable for CommNode {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for CommNode {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(CommNode { id })
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


#[derive(Clone)]
pub struct Contract {
    id: u32,
}

impl fmt::Debug for Contract {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Contract({})", self.id)
    }
}

impl RPCEncodable for Contract {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Contract {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Contract { id })
    }
}


#[derive(Clone)]
pub struct ContractManager {
    id: u32,
}

impl fmt::Debug for ContractManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ContractManager({})", self.id)
    }
}

impl RPCEncodable for ContractManager {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for ContractManager {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(ContractManager { id })
    }
}


#[derive(Clone)]
pub struct ContractParameter {
    id: u32,
}

impl fmt::Debug for ContractParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ContractParameter({})", self.id)
    }
}

impl RPCEncodable for ContractParameter {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for ContractParameter {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(ContractParameter { id })
    }
}


#[derive(Clone)]
pub struct Control {
    id: u32,
}

impl fmt::Debug for Control {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Control({})", self.id)
    }
}

impl RPCEncodable for Control {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Control {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Control { id })
    }
}


#[derive(Clone)]
pub struct ControlSurface {
    id: u32,
}

impl fmt::Debug for ControlSurface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ControlSurface({})", self.id)
    }
}

impl RPCEncodable for ControlSurface {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for ControlSurface {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(ControlSurface { id })
    }
}


#[derive(Clone)]
pub struct CrewMember {
    id: u32,
}

impl fmt::Debug for CrewMember {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CrewMember({})", self.id)
    }
}

impl RPCEncodable for CrewMember {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for CrewMember {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(CrewMember { id })
    }
}


#[derive(Clone)]
pub struct Decoupler {
    id: u32,
}

impl fmt::Debug for Decoupler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decoupler({})", self.id)
    }
}

impl RPCEncodable for Decoupler {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Decoupler {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Decoupler { id })
    }
}


#[derive(Clone)]
pub struct DockingPort {
    id: u32,
}

impl fmt::Debug for DockingPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DockingPort({})", self.id)
    }
}

impl RPCEncodable for DockingPort {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for DockingPort {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(DockingPort { id })
    }
}


#[derive(Clone)]
pub struct Engine {
    id: u32,
}

impl fmt::Debug for Engine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Engine({})", self.id)
    }
}

impl RPCEncodable for Engine {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Engine {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Engine { id })
    }
}


#[derive(Clone)]
pub struct Experiment {
    id: u32,
}

impl fmt::Debug for Experiment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Experiment({})", self.id)
    }
}

impl RPCEncodable for Experiment {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Experiment {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Experiment { id })
    }
}


#[derive(Clone)]
pub struct Fairing {
    id: u32,
}

impl fmt::Debug for Fairing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fairing({})", self.id)
    }
}

impl RPCEncodable for Fairing {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Fairing {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Fairing { id })
    }
}


#[derive(Clone)]
pub struct Flight {
    id: u32,
}

impl fmt::Debug for Flight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Flight({})", self.id)
    }
}

impl RPCEncodable for Flight {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Flight {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Flight { id })
    }
}


#[derive(Clone)]
pub struct Force {
    id: u32,
}

impl fmt::Debug for Force {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Force({})", self.id)
    }
}

impl RPCEncodable for Force {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Force {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Force { id })
    }
}


#[derive(Clone)]
pub struct Intake {
    id: u32,
}

impl fmt::Debug for Intake {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Intake({})", self.id)
    }
}

impl RPCEncodable for Intake {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Intake {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Intake { id })
    }
}


#[derive(Clone)]
pub struct LaunchClamp {
    id: u32,
}

impl fmt::Debug for LaunchClamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LaunchClamp({})", self.id)
    }
}

impl RPCEncodable for LaunchClamp {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for LaunchClamp {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(LaunchClamp { id })
    }
}


#[derive(Clone)]
pub struct Leg {
    id: u32,
}

impl fmt::Debug for Leg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Leg({})", self.id)
    }
}

impl RPCEncodable for Leg {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Leg {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Leg { id })
    }
}


#[derive(Clone)]
pub struct Light {
    id: u32,
}

impl fmt::Debug for Light {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Light({})", self.id)
    }
}

impl RPCEncodable for Light {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Light {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Light { id })
    }
}


#[derive(Clone)]
pub struct Module {
    id: u32,
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Module({})", self.id)
    }
}

impl RPCEncodable for Module {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Module {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Module { id })
    }
}


#[derive(Clone)]
pub struct Node {
    id: u32,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node({})", self.id)
    }
}

impl RPCEncodable for Node {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Node {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Node { id })
    }
}


#[derive(Clone)]
pub struct Orbit {
    id: u32,
}

impl fmt::Debug for Orbit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Orbit({})", self.id)
    }
}

impl RPCEncodable for Orbit {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Orbit {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Orbit { id })
    }
}


#[derive(Clone)]
pub struct Parachute {
    id: u32,
}

impl fmt::Debug for Parachute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parachute({})", self.id)
    }
}

impl RPCEncodable for Parachute {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Parachute {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Parachute { id })
    }
}


#[derive(Clone)]
pub struct Part {
    id: u32,
}

impl fmt::Debug for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Part({})", self.id)
    }
}

impl RPCEncodable for Part {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Part {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Part { id })
    }
}


#[derive(Clone)]
pub struct Parts {
    id: u32,
}

impl fmt::Debug for Parts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parts({})", self.id)
    }
}

impl RPCEncodable for Parts {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Parts {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Parts { id })
    }
}


#[derive(Clone)]
pub struct Propellant {
    id: u32,
}

impl fmt::Debug for Propellant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Propellant({})", self.id)
    }
}

impl RPCEncodable for Propellant {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Propellant {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Propellant { id })
    }
}


#[derive(Clone)]
pub struct RCS {
    id: u32,
}

impl fmt::Debug for RCS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RCS({})", self.id)
    }
}

impl RPCEncodable for RCS {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for RCS {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(RCS { id })
    }
}


#[derive(Clone)]
pub struct Radiator {
    id: u32,
}

impl fmt::Debug for Radiator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Radiator({})", self.id)
    }
}

impl RPCEncodable for Radiator {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Radiator {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Radiator { id })
    }
}


#[derive(Clone)]
pub struct ReactionWheel {
    id: u32,
}

impl fmt::Debug for ReactionWheel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ReactionWheel({})", self.id)
    }
}

impl RPCEncodable for ReactionWheel {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for ReactionWheel {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(ReactionWheel { id })
    }
}


#[derive(Clone)]
pub struct ReferenceFrame {
    id: u32,
}

impl fmt::Debug for ReferenceFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ReferenceFrame({})", self.id)
    }
}

impl RPCEncodable for ReferenceFrame {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for ReferenceFrame {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(ReferenceFrame { id })
    }
}


#[derive(Clone)]
pub struct Resource {
    id: u32,
}

impl fmt::Debug for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Resource({})", self.id)
    }
}

impl RPCEncodable for Resource {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Resource {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Resource { id })
    }
}


#[derive(Clone)]
pub struct ResourceConverter {
    id: u32,
}

impl fmt::Debug for ResourceConverter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ResourceConverter({})", self.id)
    }
}

impl RPCEncodable for ResourceConverter {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for ResourceConverter {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(ResourceConverter { id })
    }
}


#[derive(Clone)]
pub struct ResourceHarvester {
    id: u32,
}

impl fmt::Debug for ResourceHarvester {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ResourceHarvester({})", self.id)
    }
}

impl RPCEncodable for ResourceHarvester {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for ResourceHarvester {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(ResourceHarvester { id })
    }
}


#[derive(Clone)]
pub struct ResourceTransfer {
    id: u32,
}

impl fmt::Debug for ResourceTransfer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ResourceTransfer({})", self.id)
    }
}

impl RPCEncodable for ResourceTransfer {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for ResourceTransfer {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(ResourceTransfer { id })
    }
}


#[derive(Clone)]
pub struct Resources {
    id: u32,
}

impl fmt::Debug for Resources {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Resources({})", self.id)
    }
}

impl RPCEncodable for Resources {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Resources {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Resources { id })
    }
}


#[derive(Clone)]
pub struct ScienceData {
    id: u32,
}

impl fmt::Debug for ScienceData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScienceData({})", self.id)
    }
}

impl RPCEncodable for ScienceData {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for ScienceData {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(ScienceData { id })
    }
}


#[derive(Clone)]
pub struct ScienceSubject {
    id: u32,
}

impl fmt::Debug for ScienceSubject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScienceSubject({})", self.id)
    }
}

impl RPCEncodable for ScienceSubject {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for ScienceSubject {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(ScienceSubject { id })
    }
}


#[derive(Clone)]
pub struct Sensor {
    id: u32,
}

impl fmt::Debug for Sensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sensor({})", self.id)
    }
}

impl RPCEncodable for Sensor {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Sensor {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Sensor { id })
    }
}


#[derive(Clone)]
pub struct SolarPanel {
    id: u32,
}

impl fmt::Debug for SolarPanel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SolarPanel({})", self.id)
    }
}

impl RPCEncodable for SolarPanel {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for SolarPanel {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(SolarPanel { id })
    }
}


#[derive(Clone)]
pub struct Thruster {
    id: u32,
}

impl fmt::Debug for Thruster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Thruster({})", self.id)
    }
}

impl RPCEncodable for Thruster {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Thruster {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Thruster { id })
    }
}


#[derive(Clone)]
pub struct Vessel {
    id: u32,
}

impl fmt::Debug for Vessel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vessel({})", self.id)
    }
}

impl RPCEncodable for Vessel {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Vessel {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Vessel { id })
    }
}


#[derive(Clone)]
pub struct Waypoint {
    id: u32,
}

impl fmt::Debug for Waypoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Waypoint({})", self.id)
    }
}

impl RPCEncodable for Waypoint {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Waypoint {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Waypoint { id })
    }
}


#[derive(Clone)]
pub struct WaypointManager {
    id: u32,
}

impl fmt::Debug for WaypointManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WaypointManager({})", self.id)
    }
}

impl RPCEncodable for WaypointManager {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for WaypointManager {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(WaypointManager { id })
    }
}


#[derive(Clone)]
pub struct Wheel {
    id: u32,
}

impl fmt::Debug for Wheel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wheel({})", self.id)
    }
}

impl RPCEncodable for Wheel {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        self.id.encode(output)
    }
}

impl RPCExtractable for Wheel {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let id = RPCExtractable::extract_value(input)?;
        Ok(Wheel { id })
    }
}




#[derive(Debug, Copy, Clone)]
pub enum AntennaState {
    Deployed = 0,
    Retracted = 1,
    Deploying = 2,
    Retracting = 3,
    Broken = 4,
}

impl From<i32> for AntennaState {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => AntennaState::Deployed,
            1 => AntennaState::Retracted,
            2 => AntennaState::Deploying,
            3 => AntennaState::Retracting,
            4 => AntennaState::Broken,
            _ => panic!("Could not convert '{}' to a SpaceCenter::AntennaState", source),
        }
    }
}

impl RPCEncodable for AntennaState {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for AntennaState {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(AntennaState::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CameraMode {
    Automatic = 0,
    Free = 1,
    Chase = 2,
    Locked = 3,
    Orbital = 4,
    IVA = 5,
    Map = 6,
}

impl From<i32> for CameraMode {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => CameraMode::Automatic,
            1 => CameraMode::Free,
            2 => CameraMode::Chase,
            3 => CameraMode::Locked,
            4 => CameraMode::Orbital,
            5 => CameraMode::IVA,
            6 => CameraMode::Map,
            _ => panic!("Could not convert '{}' to a SpaceCenter::CameraMode", source),
        }
    }
}

impl RPCEncodable for CameraMode {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for CameraMode {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(CameraMode::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CargoBayState {
    Open = 0,
    Closed = 1,
    Opening = 2,
    Closing = 3,
}

impl From<i32> for CargoBayState {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => CargoBayState::Open,
            1 => CargoBayState::Closed,
            2 => CargoBayState::Opening,
            3 => CargoBayState::Closing,
            _ => panic!("Could not convert '{}' to a SpaceCenter::CargoBayState", source),
        }
    }
}

impl RPCEncodable for CargoBayState {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for CargoBayState {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(CargoBayState::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CommLinkType {
    Home = 0,
    Control = 1,
    Relay = 2,
}

impl From<i32> for CommLinkType {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => CommLinkType::Home,
            1 => CommLinkType::Control,
            2 => CommLinkType::Relay,
            _ => panic!("Could not convert '{}' to a SpaceCenter::CommLinkType", source),
        }
    }
}

impl RPCEncodable for CommLinkType {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for CommLinkType {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(CommLinkType::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ContractState {
    Active = 0,
    Canceled = 1,
    Completed = 2,
    DeadlineExpired = 3,
    Declined = 4,
    Failed = 5,
    Generated = 6,
    Offered = 7,
    OfferExpired = 8,
    Withdrawn = 9,
}

impl From<i32> for ContractState {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => ContractState::Active,
            1 => ContractState::Canceled,
            2 => ContractState::Completed,
            3 => ContractState::DeadlineExpired,
            4 => ContractState::Declined,
            5 => ContractState::Failed,
            6 => ContractState::Generated,
            7 => ContractState::Offered,
            8 => ContractState::OfferExpired,
            9 => ContractState::Withdrawn,
            _ => panic!("Could not convert '{}' to a SpaceCenter::ContractState", source),
        }
    }
}

impl RPCEncodable for ContractState {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for ContractState {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(ContractState::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ControlInputMode {
    Additive = 0,
    Override = 1,
}

impl From<i32> for ControlInputMode {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => ControlInputMode::Additive,
            1 => ControlInputMode::Override,
            _ => panic!("Could not convert '{}' to a SpaceCenter::ControlInputMode", source),
        }
    }
}

impl RPCEncodable for ControlInputMode {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for ControlInputMode {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(ControlInputMode::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ControlSource {
    Kerbal = 0,
    Probe = 1,
    None = 2,
}

impl From<i32> for ControlSource {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => ControlSource::Kerbal,
            1 => ControlSource::Probe,
            2 => ControlSource::None,
            _ => panic!("Could not convert '{}' to a SpaceCenter::ControlSource", source),
        }
    }
}

impl RPCEncodable for ControlSource {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for ControlSource {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(ControlSource::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ControlState {
    Full = 0,
    Partial = 1,
    None = 2,
}

impl From<i32> for ControlState {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => ControlState::Full,
            1 => ControlState::Partial,
            2 => ControlState::None,
            _ => panic!("Could not convert '{}' to a SpaceCenter::ControlState", source),
        }
    }
}

impl RPCEncodable for ControlState {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for ControlState {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(ControlState::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CrewMemberType {
    Applicant = 0,
    Crew = 1,
    Tourist = 2,
    Unowned = 3,
}

impl From<i32> for CrewMemberType {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => CrewMemberType::Applicant,
            1 => CrewMemberType::Crew,
            2 => CrewMemberType::Tourist,
            3 => CrewMemberType::Unowned,
            _ => panic!("Could not convert '{}' to a SpaceCenter::CrewMemberType", source),
        }
    }
}

impl RPCEncodable for CrewMemberType {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for CrewMemberType {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(CrewMemberType::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DockingPortState {
    Ready = 0,
    Docked = 1,
    Docking = 2,
    Undocking = 3,
    Shielded = 4,
    Moving = 5,
}

impl From<i32> for DockingPortState {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => DockingPortState::Ready,
            1 => DockingPortState::Docked,
            2 => DockingPortState::Docking,
            3 => DockingPortState::Undocking,
            4 => DockingPortState::Shielded,
            5 => DockingPortState::Moving,
            _ => panic!("Could not convert '{}' to a SpaceCenter::DockingPortState", source),
        }
    }
}

impl RPCEncodable for DockingPortState {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for DockingPortState {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(DockingPortState::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum GameMode {
    Sandbox = 0,
    Career = 1,
    Science = 2,
    ScienceSandbox = 3,
    Mission = 4,
    MissionBuilder = 5,
    Scenario = 6,
    ScenarioNonResumable = 7,
}

impl From<i32> for GameMode {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => GameMode::Sandbox,
            1 => GameMode::Career,
            2 => GameMode::Science,
            3 => GameMode::ScienceSandbox,
            4 => GameMode::Mission,
            5 => GameMode::MissionBuilder,
            6 => GameMode::Scenario,
            7 => GameMode::ScenarioNonResumable,
            _ => panic!("Could not convert '{}' to a SpaceCenter::GameMode", source),
        }
    }
}

impl RPCEncodable for GameMode {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for GameMode {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(GameMode::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum LegState {
    Deployed = 0,
    Retracted = 1,
    Deploying = 2,
    Retracting = 3,
    Broken = 4,
}

impl From<i32> for LegState {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => LegState::Deployed,
            1 => LegState::Retracted,
            2 => LegState::Deploying,
            3 => LegState::Retracting,
            4 => LegState::Broken,
            _ => panic!("Could not convert '{}' to a SpaceCenter::LegState", source),
        }
    }
}

impl RPCEncodable for LegState {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for LegState {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(LegState::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MotorState {
    Idle = 0,
    Running = 1,
    Disabled = 2,
    Inoperable = 3,
    NotEnoughResources = 4,
}

impl From<i32> for MotorState {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => MotorState::Idle,
            1 => MotorState::Running,
            2 => MotorState::Disabled,
            3 => MotorState::Inoperable,
            4 => MotorState::NotEnoughResources,
            _ => panic!("Could not convert '{}' to a SpaceCenter::MotorState", source),
        }
    }
}

impl RPCEncodable for MotorState {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for MotorState {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(MotorState::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ParachuteState {
    Stowed = 0,
    Armed = 1,
    Active = 2,
    SemiDeployed = 3,
    Deployed = 4,
    Cut = 5,
}

impl From<i32> for ParachuteState {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => ParachuteState::Stowed,
            1 => ParachuteState::Armed,
            2 => ParachuteState::Active,
            3 => ParachuteState::SemiDeployed,
            4 => ParachuteState::Deployed,
            5 => ParachuteState::Cut,
            _ => panic!("Could not convert '{}' to a SpaceCenter::ParachuteState", source),
        }
    }
}

impl RPCEncodable for ParachuteState {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for ParachuteState {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(ParachuteState::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum RadiatorState {
    Extended = 0,
    Retracted = 1,
    Extending = 2,
    Retracting = 3,
    Broken = 4,
}

impl From<i32> for RadiatorState {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => RadiatorState::Extended,
            1 => RadiatorState::Retracted,
            2 => RadiatorState::Extending,
            3 => RadiatorState::Retracting,
            4 => RadiatorState::Broken,
            _ => panic!("Could not convert '{}' to a SpaceCenter::RadiatorState", source),
        }
    }
}

impl RPCEncodable for RadiatorState {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for RadiatorState {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(RadiatorState::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ResourceConverterState {
    Running = 0,
    Idle = 1,
    MissingResource = 2,
    StorageFull = 3,
    Capacity = 4,
    Unknown = 5,
}

impl From<i32> for ResourceConverterState {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => ResourceConverterState::Running,
            1 => ResourceConverterState::Idle,
            2 => ResourceConverterState::MissingResource,
            3 => ResourceConverterState::StorageFull,
            4 => ResourceConverterState::Capacity,
            5 => ResourceConverterState::Unknown,
            _ => panic!("Could not convert '{}' to a SpaceCenter::ResourceConverterState", source),
        }
    }
}

impl RPCEncodable for ResourceConverterState {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for ResourceConverterState {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(ResourceConverterState::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ResourceFlowMode {
    Vessel = 0,
    Stage = 1,
    Adjacent = 2,
    None = 3,
}

impl From<i32> for ResourceFlowMode {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => ResourceFlowMode::Vessel,
            1 => ResourceFlowMode::Stage,
            2 => ResourceFlowMode::Adjacent,
            3 => ResourceFlowMode::None,
            _ => panic!("Could not convert '{}' to a SpaceCenter::ResourceFlowMode", source),
        }
    }
}

impl RPCEncodable for ResourceFlowMode {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for ResourceFlowMode {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(ResourceFlowMode::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ResourceHarvesterState {
    Deploying = 0,
    Deployed = 1,
    Retracting = 2,
    Retracted = 3,
    Active = 4,
}

impl From<i32> for ResourceHarvesterState {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => ResourceHarvesterState::Deploying,
            1 => ResourceHarvesterState::Deployed,
            2 => ResourceHarvesterState::Retracting,
            3 => ResourceHarvesterState::Retracted,
            4 => ResourceHarvesterState::Active,
            _ => panic!("Could not convert '{}' to a SpaceCenter::ResourceHarvesterState", source),
        }
    }
}

impl RPCEncodable for ResourceHarvesterState {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for ResourceHarvesterState {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(ResourceHarvesterState::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SASMode {
    StabilityAssist = 0,
    Maneuver = 1,
    Prograde = 2,
    Retrograde = 3,
    Normal = 4,
    AntiNormal = 5,
    Radial = 6,
    AntiRadial = 7,
    Target = 8,
    AntiTarget = 9,
}

impl From<i32> for SASMode {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => SASMode::StabilityAssist,
            1 => SASMode::Maneuver,
            2 => SASMode::Prograde,
            3 => SASMode::Retrograde,
            4 => SASMode::Normal,
            5 => SASMode::AntiNormal,
            6 => SASMode::Radial,
            7 => SASMode::AntiRadial,
            8 => SASMode::Target,
            9 => SASMode::AntiTarget,
            _ => panic!("Could not convert '{}' to a SpaceCenter::SASMode", source),
        }
    }
}

impl RPCEncodable for SASMode {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for SASMode {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(SASMode::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SolarPanelState {
    Extended = 0,
    Retracted = 1,
    Extending = 2,
    Retracting = 3,
    Broken = 4,
}

impl From<i32> for SolarPanelState {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => SolarPanelState::Extended,
            1 => SolarPanelState::Retracted,
            2 => SolarPanelState::Extending,
            3 => SolarPanelState::Retracting,
            4 => SolarPanelState::Broken,
            _ => panic!("Could not convert '{}' to a SpaceCenter::SolarPanelState", source),
        }
    }
}

impl RPCEncodable for SolarPanelState {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for SolarPanelState {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(SolarPanelState::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SpeedMode {
    Orbit = 0,
    Surface = 1,
    Target = 2,
}

impl From<i32> for SpeedMode {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => SpeedMode::Orbit,
            1 => SpeedMode::Surface,
            2 => SpeedMode::Target,
            _ => panic!("Could not convert '{}' to a SpaceCenter::SpeedMode", source),
        }
    }
}

impl RPCEncodable for SpeedMode {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for SpeedMode {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(SpeedMode::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum VesselSituation {
    PreLaunch = 0,
    Orbiting = 1,
    SubOrbital = 2,
    Escaping = 3,
    Flying = 4,
    Landed = 5,
    Splashed = 6,
    Docked = 7,
}

impl From<i32> for VesselSituation {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => VesselSituation::PreLaunch,
            1 => VesselSituation::Orbiting,
            2 => VesselSituation::SubOrbital,
            3 => VesselSituation::Escaping,
            4 => VesselSituation::Flying,
            5 => VesselSituation::Landed,
            6 => VesselSituation::Splashed,
            7 => VesselSituation::Docked,
            _ => panic!("Could not convert '{}' to a SpaceCenter::VesselSituation", source),
        }
    }
}

impl RPCEncodable for VesselSituation {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for VesselSituation {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(VesselSituation::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum VesselType {
    Base = 0,
    Debris = 1,
    Lander = 2,
    Plane = 3,
    Probe = 4,
    Relay = 5,
    Rover = 6,
    Ship = 7,
    Station = 8,
}

impl From<i32> for VesselType {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => VesselType::Base,
            1 => VesselType::Debris,
            2 => VesselType::Lander,
            3 => VesselType::Plane,
            4 => VesselType::Probe,
            5 => VesselType::Relay,
            6 => VesselType::Rover,
            7 => VesselType::Ship,
            8 => VesselType::Station,
            _ => panic!("Could not convert '{}' to a SpaceCenter::VesselType", source),
        }
    }
}

impl RPCEncodable for VesselType {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for VesselType {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(VesselType::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum WarpMode {
    Rails = 0,
    Physics = 1,
    None = 2,
}

impl From<i32> for WarpMode {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => WarpMode::Rails,
            1 => WarpMode::Physics,
            2 => WarpMode::None,
            _ => panic!("Could not convert '{}' to a SpaceCenter::WarpMode", source),
        }
    }
}

impl RPCEncodable for WarpMode {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for WarpMode {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(WarpMode::from(value))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum WheelState {
    Deployed = 0,
    Retracted = 1,
    Deploying = 2,
    Retracting = 3,
    Broken = 4,
}

impl From<i32> for WheelState {
    #[inline]
    fn from(source: i32) -> Self {
        match source {
            0 => WheelState::Deployed,
            1 => WheelState::Retracted,
            2 => WheelState::Deploying,
            3 => WheelState::Retracting,
            4 => WheelState::Broken,
            _ => panic!("Could not convert '{}' to a SpaceCenter::WheelState", source),
        }
    }
}

impl RPCEncodable for WheelState {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        (*self as i32).encode(output)
    }
}

impl RPCExtractable for WheelState {
    fn extract_value(input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let value : i32 = RPCExtractable::extract_value(input)?;
        Ok(WheelState::from(value))
    }
}



/// <doc> <summary> Returns <c>true</c> if regular "on-rails" time warp can be used, at the specified warp <paramref name="factor" />. The maximum time warp rate is limited by various things, including how close the active vessel is to a planet. See <a href="https://wiki.kerbalspaceprogram.com/wiki/Time_warp">the KSP wiki</a> for details. </summary> <param name="factor">The warp factor to check.</param> </doc>
pub fn can_rails_warp_at(p_factor: i32) -> CallHandle<bool> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("CanRailsWarpAt"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_factor.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Clears the current target. </summary> </doc>
pub fn clear_target() -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("ClearTarget"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Launch a vessel. </summary> <param name="craftDirectory">Name of the directory in the current saves "Ships" directory, that contains the craft file. For example <c>"VAB"</c> or <c>"SPH"</c>.</param> <param name="name">Name of the vessel to launch. This is the name of the ".craft" file in the save directory, without the ".craft" file extension.</param> <param name="launchSite">Name of the launch site. For example <c>"LaunchPad"</c> or <c>"Runway"</c>.</param> <param name="recover">If true and there is a vessel on the launch site, recover it before launching.</param> <remarks> Throws an exception if any of the games pre-flight checks fail. </remarks> </doc>
pub fn launch_vessel(p_craft_directory: String, p_name: String, p_launch_site: String, p_recover: bool) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("LaunchVessel"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_craft_directory.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_name.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_launch_site.encode_to_bytes().unwrap());
    arguments.push(arg2);

    let mut arg3 = krpc::Argument::new();
    arg3.set_position(3);
    arg3.set_value(p_recover.encode_to_bytes().unwrap());
    arguments.push(arg3);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Launch a new vessel from the SPH onto the runway. </summary> <param name="name">Name of the vessel to launch.</param> <param name="recover">If true and there is a vessel on the runway, recover it before launching.</param> <remarks> This is equivalent to calling <see cref="M:SpaceCenter.LaunchVessel" /> with the craft directory set to "SPH" and the launch site set to "Runway". Throws an exception if any of the games pre-flight checks fail. </remarks> </doc>
pub fn launch_vessel_from_sph(p_name: String, p_recover: bool) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("LaunchVesselFromSPH"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_name.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_recover.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Launch a new vessel from the VAB onto the launchpad. </summary> <param name="name">Name of the vessel to launch.</param> <param name="recover">If true and there is a vessel on the launch pad, recover it before launching.</param> <remarks> This is equivalent to calling <see cref="M:SpaceCenter.LaunchVessel" /> with the craft directory set to "VAB" and the launch site set to "LaunchPad". Throws an exception if any of the games pre-flight checks fail. </remarks> </doc>
pub fn launch_vessel_from_vab(p_name: String, p_recover: bool) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("LaunchVesselFromVAB"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_name.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_recover.encode_to_bytes().unwrap());
    arguments.push(arg1);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Returns a list of vessels from the given <paramref name="craftDirectory" /> that can be launched. </summary> <param name="craftDirectory">Name of the directory in the current saves "Ships" directory. For example <c>"VAB"</c> or <c>"SPH"</c>.</param> </doc>
pub fn launchable_vessels(p_craft_directory: String) -> CallHandle<Vec<String>> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("LaunchableVessels"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_craft_directory.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Load the game with the given name. This will create a load a save file called <c>name.sfs</c> from the folder of the current save game. </summary> </doc>
pub fn load(p_name: String) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("Load"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_name.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> The direction from which the orbits longitude of ascending node is measured, in the given reference frame. </summary> <returns>The direction as a unit vector.</returns> <param name="referenceFrame">The reference frame that the returned direction is in.</param> </doc>
pub fn orbit_static_reference_plane_direction(p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("Orbit_static_ReferencePlaneDirection"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_reference_frame.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> The direction that is normal to the orbits reference plane, in the given reference frame. The reference plane is the plane from which the orbits inclination is measured. </summary> <returns>The direction as a unit vector.</returns> <param name="referenceFrame">The reference frame that the returned direction is in.</param> </doc>
pub fn orbit_static_reference_plane_normal(p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("Orbit_static_ReferencePlaneNormal"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_reference_frame.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Load a quicksave. </summary> <remarks> This is the same as calling <see cref="M:SpaceCenter.Load" /> with the name "quicksave". </remarks> </doc>
pub fn quickload() -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("Quickload"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Save a quicksave. </summary> <remarks> This is the same as calling <see cref="M:SpaceCenter.Save" /> with the name "quicksave". </remarks> </doc>
pub fn quicksave() -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("Quicksave"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Cast a ray from a given position in a given direction, and return the distance to the hit point. If no hit occurs, returns infinity. </summary> <param name="position">Position, as a vector, of the origin of the ray.</param> <param name="direction">Direction of the ray, as a unit vector.</param> <param name="referenceFrame">The reference frame that the position and direction are in.</param> <returns>The distance to the hit, in meters, or infinity if there was no hit.</returns> </doc>
pub fn raycast_distance(p_position: (f64, f64, f64), p_direction: (f64, f64, f64), p_reference_frame: &ReferenceFrame) -> CallHandle<f64> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("RaycastDistance"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_position.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_direction.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_reference_frame.encode_to_bytes().unwrap());
    arguments.push(arg2);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Cast a ray from a given position in a given direction, and return the part that it hits. If no hit occurs, returns <c>null</c>. </summary> <param name="position">Position, as a vector, of the origin of the ray.</param> <param name="direction">Direction of the ray, as a unit vector.</param> <param name="referenceFrame">The reference frame that the position and direction are in.</param> <returns>The part that was hit or <c>null</c> if there was no hit.</returns> </doc>
pub fn raycast_part(p_position: (f64, f64, f64), p_direction: (f64, f64, f64), p_reference_frame: &ReferenceFrame) -> CallHandle<Part> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("RaycastPart"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_position.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_direction.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_reference_frame.encode_to_bytes().unwrap());
    arguments.push(arg2);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Create a hybrid reference frame. This is a custom reference frame whose components inherited from other reference frames. </summary> <param name="position">The reference frame providing the position of the origin.</param> <param name="rotation">The reference frame providing the rotation of the frame.</param> <param name="velocity">The reference frame providing the linear velocity of the frame. </param> <param name="angularVelocity">The reference frame providing the angular velocity of the frame.</param> <remarks> The <paramref name="position" /> reference frame is required but all other reference frames are optional. If omitted, they are set to the <paramref name="position" /> reference frame. </remarks> </doc>
pub fn reference_frame_static_create_hybrid(p_position: &ReferenceFrame, p_rotation: &ReferenceFrame, p_velocity: &ReferenceFrame, p_angular_velocity: &ReferenceFrame) -> CallHandle<ReferenceFrame> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("ReferenceFrame_static_CreateHybrid"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_position.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_rotation.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_velocity.encode_to_bytes().unwrap());
    arguments.push(arg2);

    let mut arg3 = krpc::Argument::new();
    arg3.set_position(3);
    arg3.set_value(p_angular_velocity.encode_to_bytes().unwrap());
    arguments.push(arg3);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Create a relative reference frame. This is a custom reference frame whose components offset the components of a parent reference frame. </summary> <param name="referenceFrame">The parent reference frame on which to base this reference frame.</param> <param name="position">The offset of the position of the origin, as a position vector. Defaults to <math>(0, 0, 0)</math></param> <param name="rotation">The rotation to apply to the parent frames rotation, as a quaternion of the form <math>(x, y, z, w)</math>. Defaults to <math>(0, 0, 0, 1)</math> (i.e. no rotation)</param> <param name="velocity">The linear velocity to offset the parent frame by, as a vector pointing in the direction of travel, whose magnitude is the speed in meters per second. Defaults to <math>(0, 0, 0)</math>.</param> <param name="angularVelocity">The angular velocity to offset the parent frame by, as a vector. This vector points in the direction of the axis of rotation, and its magnitude is the speed of the rotation in radians per second. Defaults to <math>(0, 0, 0)</math>.</param> </doc>
pub fn reference_frame_static_create_relative(p_reference_frame: &ReferenceFrame, p_position: (f64, f64, f64), p_rotation: (f64, f64, f64, f64), p_velocity: (f64, f64, f64), p_angular_velocity: (f64, f64, f64)) -> CallHandle<ReferenceFrame> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("ReferenceFrame_static_CreateRelative"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_reference_frame.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_position.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_rotation.encode_to_bytes().unwrap());
    arguments.push(arg2);

    let mut arg3 = krpc::Argument::new();
    arg3.set_position(3);
    arg3.set_value(p_velocity.encode_to_bytes().unwrap());
    arguments.push(arg3);

    let mut arg4 = krpc::Argument::new();
    arg4.set_position(4);
    arg4.set_value(p_angular_velocity.encode_to_bytes().unwrap());
    arguments.push(arg4);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Start transferring a resource transfer between a pair of parts. The transfer will move at most <paramref name="maxAmount" /> units of the resource, depending on how much of the resource is available in the source part and how much storage is available in the destination part. Use <see cref="M:SpaceCenter.ResourceTransfer.Complete" /> to check if the transfer is complete. Use <see cref="M:SpaceCenter.ResourceTransfer.Amount" /> to see how much of the resource has been transferred. </summary> <param name="fromPart">The part to transfer to.</param> <param name="toPart">The part to transfer from.</param> <param name="resource">The name of the resource to transfer.</param> <param name="maxAmount">The maximum amount of resource to transfer.</param> </doc>
pub fn resource_transfer_static_start(p_from_part: &Part, p_to_part: &Part, p_resource: String, p_max_amount: f32) -> CallHandle<ResourceTransfer> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("ResourceTransfer_static_Start"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_from_part.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_to_part.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_resource.encode_to_bytes().unwrap());
    arguments.push(arg2);

    let mut arg3 = krpc::Argument::new();
    arg3.set_position(3);
    arg3.set_value(p_max_amount.encode_to_bytes().unwrap());
    arguments.push(arg3);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Returns the density of a resource, in <math>kg/l</math>. </summary> <param name="name">The name of the resource.</param> </doc>
pub fn resources_static_density(p_name: String) -> CallHandle<f32> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("Resources_static_Density"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_name.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Returns the flow mode of a resource. </summary> <param name="name">The name of the resource.</param> </doc>
pub fn resources_static_flow_mode(p_name: String) -> CallHandle<ResourceFlowMode> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("Resources_static_FlowMode"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_name.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Save the game with a given name. This will create a save file called <c>name.sfs</c> in the folder of the current save game. </summary> </doc>
pub fn save(p_name: String) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("Save"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_name.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Converts a direction from one reference frame to another. </summary> <param name="direction">Direction, as a vector, in reference frame <paramref name="from" />. </param> <param name="from">The reference frame that the direction is in.</param> <param name="to">The reference frame to covert the direction to.</param> <returns>The corresponding direction, as a vector, in reference frame <paramref name="to" />.</returns> </doc>
pub fn transform_direction(p_direction: (f64, f64, f64), p_from: &ReferenceFrame, p_to: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("TransformDirection"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_direction.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_from.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_to.encode_to_bytes().unwrap());
    arguments.push(arg2);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Converts a position from one reference frame to another. </summary> <param name="position">Position, as a vector, in reference frame <paramref name="from" />.</param> <param name="from">The reference frame that the position is in.</param> <param name="to">The reference frame to covert the position to.</param> <returns>The corresponding position, as a vector, in reference frame <paramref name="to" />.</returns> </doc>
pub fn transform_position(p_position: (f64, f64, f64), p_from: &ReferenceFrame, p_to: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("TransformPosition"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_position.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_from.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_to.encode_to_bytes().unwrap());
    arguments.push(arg2);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Converts a rotation from one reference frame to another. </summary> <param name="rotation">Rotation, as a quaternion of the form <math>(x, y, z, w)</math>, in reference frame <paramref name="from" />.</param> <param name="from">The reference frame that the rotation is in.</param> <param name="to">The reference frame to covert the rotation to.</param> <returns>The corresponding rotation, as a quaternion of the form <math>(x, y, z, w)</math>, in reference frame <paramref name="to" />.</returns> </doc>
pub fn transform_rotation(p_rotation: (f64, f64, f64, f64), p_from: &ReferenceFrame, p_to: &ReferenceFrame) -> CallHandle<(f64, f64, f64, f64)> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("TransformRotation"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_rotation.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_from.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_to.encode_to_bytes().unwrap());
    arguments.push(arg2);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Converts a velocity (acting at the specified position) from one reference frame to another. The position is required to take the relative angular velocity of the reference frames into account. </summary> <param name="position">Position, as a vector, in reference frame <paramref name="from" />.</param> <param name="velocity">Velocity, as a vector that points in the direction of travel and whose magnitude is the speed in meters per second, in reference frame <paramref name="from" />.</param> <param name="from">The reference frame that the position and velocity are in.</param> <param name="to">The reference frame to covert the velocity to.</param> <returns>The corresponding velocity, as a vector, in reference frame <paramref name="to" />.</returns> </doc>
pub fn transform_velocity(p_position: (f64, f64, f64), p_velocity: (f64, f64, f64), p_from: &ReferenceFrame, p_to: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("TransformVelocity"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_position.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_velocity.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_from.encode_to_bytes().unwrap());
    arguments.push(arg2);

    let mut arg3 = krpc::Argument::new();
    arg3.set_position(3);
    arg3.set_value(p_to.encode_to_bytes().unwrap());
    arguments.push(arg3);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Uses time acceleration to warp forward to a time in the future, specified by universal time <paramref name="ut" />. This call blocks until the desired time is reached. Uses regular "on-rails" or physical time warp as appropriate. For example, physical time warp is used when the active vessel is traveling through an atmosphere. When using regular "on-rails" time warp, the warp rate is limited by <paramref name="maxRailsRate" />, and when using physical time warp, the warp rate is limited by <paramref name="maxPhysicsRate" />. </summary> <param name="ut">The universal time to warp to, in seconds.</param> <param name="maxRailsRate">The maximum warp rate in regular "on-rails" time warp. </param> <param name="maxPhysicsRate">The maximum warp rate in physical time warp.</param> <returns>When the time warp is complete.</returns> </doc>
pub fn warp_to(p_ut: f64, p_max_rails_rate: f32, p_max_physics_rate: f32) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("WarpTo"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_ut.encode_to_bytes().unwrap());
    arguments.push(arg0);

    let mut arg1 = krpc::Argument::new();
    arg1.set_position(1);
    arg1.set_value(p_max_rails_rate.encode_to_bytes().unwrap());
    arguments.push(arg1);

    let mut arg2 = krpc::Argument::new();
    arg2.set_position(2);
    arg2.set_value(p_max_physics_rate.encode_to_bytes().unwrap());
    arguments.push(arg2);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> The currently active vessel. </summary> </doc>
pub fn get_active_vessel() -> CallHandle<Vessel> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_ActiveVessel"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> A dictionary of all celestial bodies (planets, moons, etc.) in the game, keyed by the name of the body. </summary> </doc>
pub fn get_bodies() -> CallHandle<HashMap<String, CelestialBody>> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_Bodies"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> An object that can be used to control the camera. </summary> </doc>
pub fn get_camera() -> CallHandle<Camera> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_Camera"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The contract manager. </summary> </doc>
pub fn get_contract_manager() -> CallHandle<ContractManager> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_ContractManager"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Whether <a href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a> is installed. </summary> </doc>
pub fn get_far_available() -> CallHandle<bool> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_FARAvailable"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The current amount of funds. </summary> </doc>
pub fn get_funds() -> CallHandle<f64> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_Funds"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The value of the <a href="https://en.wikipedia.org/wiki/Gravitational_constant"> gravitational constant</a> G in <math>N(m/kg)^2</math>. </summary> </doc>
pub fn get_g() -> CallHandle<f64> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_G"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The current mode the game is in. </summary> </doc>
pub fn get_game_mode() -> CallHandle<GameMode> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_GameMode"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The current maximum regular "on-rails" warp factor that can be set. A value between 0 and 7 inclusive. See <a href="https://wiki.kerbalspaceprogram.com/wiki/Time_warp">the KSP wiki</a> for details. </summary> </doc>
pub fn get_maximum_rails_warp_factor() -> CallHandle<i32> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_MaximumRailsWarpFactor"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Whether the navball is visible. </summary> </doc>
pub fn get_navball() -> CallHandle<bool> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_Navball"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The physical time warp rate. A value between 0 and 3 inclusive. 0 means no time warp. Returns 0 if regular "on-rails" time warp is active. </summary> </doc>
pub fn get_physics_warp_factor() -> CallHandle<i32> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_PhysicsWarpFactor"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The time warp rate, using regular "on-rails" time warp. A value between 0 and 7 inclusive. 0 means no time warp. Returns 0 if physical time warp is active.  If requested time warp factor cannot be set, it will be set to the next lowest possible value. For example, if the vessel is too close to a planet. See <a href="https://wiki.kerbalspaceprogram.com/wiki/Time_warp"> the KSP wiki</a> for details. </summary> </doc>
pub fn get_rails_warp_factor() -> CallHandle<i32> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_RailsWarpFactor"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The current amount of reputation. </summary> </doc>
pub fn get_reputation() -> CallHandle<f32> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_Reputation"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The current amount of science. </summary> </doc>
pub fn get_science() -> CallHandle<f32> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_Science"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The currently targeted celestial body. </summary> </doc>
pub fn get_target_body() -> CallHandle<CelestialBody> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_TargetBody"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The currently targeted docking port. </summary> </doc>
pub fn get_target_docking_port() -> CallHandle<DockingPort> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_TargetDockingPort"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The currently targeted vessel. </summary> </doc>
pub fn get_target_vessel() -> CallHandle<Vessel> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_TargetVessel"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> Whether the UI is visible. </summary> </doc>
pub fn get_ui_visible() -> CallHandle<bool> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_UIVisible"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The current universal time in seconds. </summary> </doc>
pub fn get_ut() -> CallHandle<f64> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_UT"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> A list of all the vessels in the game. </summary> </doc>
pub fn get_vessels() -> CallHandle<Vec<Vessel>> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_Vessels"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The current warp factor. This is the index of the rate at which time is passing for either regular "on-rails" or physical time warp. Returns 0 if time warp is not active. When in on-rails time warp, this is equal to <see cref="M:SpaceCenter.RailsWarpFactor" />, and in physics time warp, this is equal to <see cref="M:SpaceCenter.PhysicsWarpFactor" />. </summary> </doc>
pub fn get_warp_factor() -> CallHandle<f32> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_WarpFactor"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The current time warp mode. Returns <see cref="M:SpaceCenter.WarpMode.None" /> if time warp is not active, <see cref="M:SpaceCenter.WarpMode.Rails" /> if regular "on-rails" time warp is active, or <see cref="M:SpaceCenter.WarpMode.Physics" /> if physical time warp is active. </summary> </doc>
pub fn get_warp_mode() -> CallHandle<WarpMode> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_WarpMode"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The current warp rate. This is the rate at which time is passing for either on-rails or physical time warp. For example, a value of 10 means time is passing 10x faster than normal. Returns 1 if time warp is not active. </summary> </doc>
pub fn get_warp_rate() -> CallHandle<f32> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_WarpRate"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The waypoint manager. </summary> </doc>
pub fn get_waypoint_manager() -> CallHandle<WaypointManager> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("get_WaypointManager"));

    CallHandle::new(proc_call)
}

/// <doc> <summary> The currently active vessel. </summary> </doc>
pub fn set_active_vessel(p_value: &Vessel) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("set_ActiveVessel"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_value.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Whether the navball is visible. </summary> </doc>
pub fn set_navball(p_value: bool) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("set_Navball"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_value.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> The physical time warp rate. A value between 0 and 3 inclusive. 0 means no time warp. Returns 0 if regular "on-rails" time warp is active. </summary> </doc>
pub fn set_physics_warp_factor(p_value: i32) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("set_PhysicsWarpFactor"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_value.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> The time warp rate, using regular "on-rails" time warp. A value between 0 and 7 inclusive. 0 means no time warp. Returns 0 if physical time warp is active.  If requested time warp factor cannot be set, it will be set to the next lowest possible value. For example, if the vessel is too close to a planet. See <a href="https://wiki.kerbalspaceprogram.com/wiki/Time_warp"> the KSP wiki</a> for details. </summary> </doc>
pub fn set_rails_warp_factor(p_value: i32) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("set_RailsWarpFactor"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_value.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> The currently targeted celestial body. </summary> </doc>
pub fn set_target_body(p_value: &CelestialBody) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("set_TargetBody"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_value.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> The currently targeted docking port. </summary> </doc>
pub fn set_target_docking_port(p_value: &DockingPort) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("set_TargetDockingPort"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_value.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> The currently targeted vessel. </summary> </doc>
pub fn set_target_vessel(p_value: &Vessel) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("set_TargetVessel"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_value.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}

/// <doc> <summary> Whether the UI is visible. </summary> </doc>
pub fn set_ui_visible(p_value: bool) -> CallHandle<()> {
    let mut proc_call = krpc::ProcedureCall::new();
    proc_call.set_service(String::from("SpaceCenter"));
    proc_call.set_procedure(String::from("set_UIVisible"));

    let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

    let mut arg0 = krpc::Argument::new();
    arg0.set_position(0);
    arg0.set_value(p_value.encode_to_bytes().unwrap());
    arguments.push(arg0);
    proc_call.set_arguments(arguments);

    CallHandle::new(proc_call)
}


impl Antenna {
    /// <doc> <summary> Cancel current transmission of data. </summary> </doc>
    pub fn cancel(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_Cancel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Transmit data. </summary> </doc>
    pub fn transmit(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_Transmit"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether partial data transmission is permitted. </summary> </doc>
    pub fn get_allow_partial(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_get_AllowPartial"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether data can be transmitted by this antenna. </summary> </doc>
    pub fn get_can_transmit(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_get_CanTransmit"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the antenna can be combined with other antennae on the vessel to boost the power. </summary> </doc>
    pub fn get_combinable(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_get_Combinable"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Exponent used to calculate the combined power of multiple antennae on a vessel. </summary> </doc>
    pub fn get_combinable_exponent(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_get_CombinableExponent"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the antenna is deployable. </summary> </doc>
    pub fn get_deployable(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_get_Deployable"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the antenna is deployed. </summary> <remarks> Fixed antennas are always deployed. Returns an error if you try to deploy a fixed antenna. </remarks> </doc>
    pub fn get_deployed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_get_Deployed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Interval between sending packets in seconds. </summary> </doc>
    pub fn get_packet_interval(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_get_PacketInterval"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Units of electric charge consumed per packet sent. </summary> </doc>
    pub fn get_packet_resource_cost(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_get_PacketResourceCost"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Amount of data sent per packet in Mits. </summary> </doc>
    pub fn get_packet_size(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_get_PacketSize"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this antenna. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The power of the antenna. </summary> </doc>
    pub fn get_power(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_get_Power"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current state of the antenna. </summary> </doc>
    pub fn get_state(&self, ) -> CallHandle<AntennaState> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_get_State"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether partial data transmission is permitted. </summary> </doc>
    pub fn set_allow_partial(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_set_AllowPartial"));

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

    /// <doc> <summary> Whether the antenna is deployed. </summary> <remarks> Fixed antennas are always deployed. Returns an error if you try to deploy a fixed antenna. </remarks> </doc>
    pub fn set_deployed(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Antenna_set_Deployed"));

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
impl AutoPilot {
    /// <doc> <summary> Disengage the auto-pilot. </summary> </doc>
    pub fn disengage(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_Disengage"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Engage the auto-pilot. </summary> </doc>
    pub fn engage(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_Engage"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Set target pitch and heading angles. </summary> <param name="pitch">Target pitch angle, in degrees between -90° and +90°.</param> <param name="heading">Target heading angle, in degrees between 0° and 360°.</param> </doc>
    pub fn target_pitch_and_heading(&self, p_pitch: f32, p_heading: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_TargetPitchAndHeading"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_pitch.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_heading.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Blocks until the vessel is pointing in the target direction and has the target roll (if set). Throws an exception if the auto-pilot has not been engaged. </summary> </doc>
    pub fn wait(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_Wait"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The angle at which the autopilot considers the vessel to be pointing close to the target. This determines the midpoint of the target velocity attenuation function. A vector of three angles, in degrees, one for each of the pitch, roll and yaw axes. Defaults to 1° for each axis. </summary> </doc>
    pub fn get_attenuation_angle(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_AttenuationAngle"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the rotation rate controllers PID parameters should be automatically tuned using the vessels moment of inertia and available torque. Defaults to <c>true</c>. See <see cref="M:SpaceCenter.AutoPilot.TimeToPeak" /> and <see cref="M:SpaceCenter.AutoPilot.Overshoot" />. </summary> </doc>
    pub fn get_auto_tune(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_AutoTune"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The time the vessel should take to come to a stop pointing in the target direction. This determines the angular acceleration used to decelerate the vessel. A vector of three times, in seconds, one for each of the pitch, roll and yaw axes. Defaults to 5 seconds for each axis. </summary> </doc>
    pub fn get_deceleration_time(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_DecelerationTime"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The error, in degrees, between the direction the ship has been asked to point in and the direction it is pointing in. Throws an exception if the auto-pilot has not been engaged and SAS is not enabled or is in stability assist mode. </summary> </doc>
    pub fn get_error(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_Error"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The error, in degrees, between the vessels current and target heading. Throws an exception if the auto-pilot has not been engaged. </summary> </doc>
    pub fn get_heading_error(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_HeadingError"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The target overshoot percentage used to autotune the PID controllers. A vector of three values, between 0 and 1, for each of the pitch, roll and yaw axes. Defaults to 0.01 for each axis. </summary> </doc>
    pub fn get_overshoot(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_Overshoot"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The error, in degrees, between the vessels current and target pitch. Throws an exception if the auto-pilot has not been engaged. </summary> </doc>
    pub fn get_pitch_error(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_PitchError"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Gains for the pitch PID controller. </summary> <remarks> When <see cref="M:SpaceCenter.AutoPilot.AutoTune" /> is true, these values are updated automatically, which will overwrite any manual changes. </remarks> </doc>
    pub fn get_pitch_pid_gains(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_PitchPIDGains"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The reference frame for the target direction (<see cref="M:SpaceCenter.AutoPilot.TargetDirection" />). </summary> <remarks> An error will be thrown if this property is set to a reference frame that rotates with the vessel being controlled, as it is impossible to rotate the vessel in such a reference frame. </remarks> </doc>
    pub fn get_reference_frame(&self, ) -> CallHandle<ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_ReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The error, in degrees, between the vessels current and target roll. Throws an exception if the auto-pilot has not been engaged or no target roll is set. </summary> </doc>
    pub fn get_roll_error(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_RollError"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Gains for the roll PID controller. </summary> <remarks> When <see cref="M:SpaceCenter.AutoPilot.AutoTune" /> is true, these values are updated automatically, which will overwrite any manual changes. </remarks> </doc>
    pub fn get_roll_pid_gains(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_RollPIDGains"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The threshold at which the autopilot will try to match the target roll angle, if any. Defaults to 5 degrees. </summary> </doc>
    pub fn get_roll_threshold(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_RollThreshold"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of SAS. </summary> <remarks>Equivalent to <see cref="M:SpaceCenter.Control.SAS" /></remarks> </doc>
    pub fn get_sas(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_SAS"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current <see cref="T:SpaceCenter.SASMode" />. These modes are equivalent to the mode buttons to the left of the navball that appear when SAS is enabled. </summary> <remarks>Equivalent to <see cref="M:SpaceCenter.Control.SASMode" /></remarks> </doc>
    pub fn get_sas_mode(&self, ) -> CallHandle<SASMode> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_SASMode"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The maximum amount of time that the vessel should need to come to a complete stop. This determines the maximum angular velocity of the vessel. A vector of three stopping times, in seconds, one for each of the pitch, roll and yaw axes. Defaults to 0.5 seconds for each axis. </summary> </doc>
    pub fn get_stopping_time(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_StoppingTime"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Direction vector corresponding to the target pitch and heading. This is in the reference frame specified by <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> </doc>
    pub fn get_target_direction(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_TargetDirection"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The target heading, in degrees, between 0° and 360°. </summary> </doc>
    pub fn get_target_heading(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_TargetHeading"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The target pitch, in degrees, between -90° and +90°. </summary> </doc>
    pub fn get_target_pitch(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_TargetPitch"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The target roll, in degrees. <c>NaN</c> if no target roll is set. </summary> </doc>
    pub fn get_target_roll(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_TargetRoll"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The target time to peak used to autotune the PID controllers. A vector of three times, in seconds, for each of the pitch, roll and yaw axes. Defaults to 3 seconds for each axis. </summary> </doc>
    pub fn get_time_to_peak(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_TimeToPeak"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Gains for the yaw PID controller. </summary> <remarks> When <see cref="M:SpaceCenter.AutoPilot.AutoTune" /> is true, these values are updated automatically, which will overwrite any manual changes. </remarks> </doc>
    pub fn get_yaw_pid_gains(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_get_YawPIDGains"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The angle at which the autopilot considers the vessel to be pointing close to the target. This determines the midpoint of the target velocity attenuation function. A vector of three angles, in degrees, one for each of the pitch, roll and yaw axes. Defaults to 1° for each axis. </summary> </doc>
    pub fn set_attenuation_angle(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_AttenuationAngle"));

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

    /// <doc> <summary> Whether the rotation rate controllers PID parameters should be automatically tuned using the vessels moment of inertia and available torque. Defaults to <c>true</c>. See <see cref="M:SpaceCenter.AutoPilot.TimeToPeak" /> and <see cref="M:SpaceCenter.AutoPilot.Overshoot" />. </summary> </doc>
    pub fn set_auto_tune(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_AutoTune"));

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

    /// <doc> <summary> The time the vessel should take to come to a stop pointing in the target direction. This determines the angular acceleration used to decelerate the vessel. A vector of three times, in seconds, one for each of the pitch, roll and yaw axes. Defaults to 5 seconds for each axis. </summary> </doc>
    pub fn set_deceleration_time(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_DecelerationTime"));

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

    /// <doc> <summary> The target overshoot percentage used to autotune the PID controllers. A vector of three values, between 0 and 1, for each of the pitch, roll and yaw axes. Defaults to 0.01 for each axis. </summary> </doc>
    pub fn set_overshoot(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_Overshoot"));

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

    /// <doc> <summary> Gains for the pitch PID controller. </summary> <remarks> When <see cref="M:SpaceCenter.AutoPilot.AutoTune" /> is true, these values are updated automatically, which will overwrite any manual changes. </remarks> </doc>
    pub fn set_pitch_pid_gains(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_PitchPIDGains"));

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

    /// <doc> <summary> The reference frame for the target direction (<see cref="M:SpaceCenter.AutoPilot.TargetDirection" />). </summary> <remarks> An error will be thrown if this property is set to a reference frame that rotates with the vessel being controlled, as it is impossible to rotate the vessel in such a reference frame. </remarks> </doc>
    pub fn set_reference_frame(&self, p_value: &ReferenceFrame) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_ReferenceFrame"));

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

    /// <doc> <summary> Gains for the roll PID controller. </summary> <remarks> When <see cref="M:SpaceCenter.AutoPilot.AutoTune" /> is true, these values are updated automatically, which will overwrite any manual changes. </remarks> </doc>
    pub fn set_roll_pid_gains(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_RollPIDGains"));

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

    /// <doc> <summary> The threshold at which the autopilot will try to match the target roll angle, if any. Defaults to 5 degrees. </summary> </doc>
    pub fn set_roll_threshold(&self, p_value: f64) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_RollThreshold"));

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

    /// <doc> <summary> The state of SAS. </summary> <remarks>Equivalent to <see cref="M:SpaceCenter.Control.SAS" /></remarks> </doc>
    pub fn set_sas(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_SAS"));

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

    /// <doc> <summary> The current <see cref="T:SpaceCenter.SASMode" />. These modes are equivalent to the mode buttons to the left of the navball that appear when SAS is enabled. </summary> <remarks>Equivalent to <see cref="M:SpaceCenter.Control.SASMode" /></remarks> </doc>
    pub fn set_sas_mode(&self, p_value: SASMode) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_SASMode"));

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

    /// <doc> <summary> The maximum amount of time that the vessel should need to come to a complete stop. This determines the maximum angular velocity of the vessel. A vector of three stopping times, in seconds, one for each of the pitch, roll and yaw axes. Defaults to 0.5 seconds for each axis. </summary> </doc>
    pub fn set_stopping_time(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_StoppingTime"));

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

    /// <doc> <summary> Direction vector corresponding to the target pitch and heading. This is in the reference frame specified by <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> </doc>
    pub fn set_target_direction(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_TargetDirection"));

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

    /// <doc> <summary> The target heading, in degrees, between 0° and 360°. </summary> </doc>
    pub fn set_target_heading(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_TargetHeading"));

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

    /// <doc> <summary> The target pitch, in degrees, between -90° and +90°. </summary> </doc>
    pub fn set_target_pitch(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_TargetPitch"));

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

    /// <doc> <summary> The target roll, in degrees. <c>NaN</c> if no target roll is set. </summary> </doc>
    pub fn set_target_roll(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_TargetRoll"));

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

    /// <doc> <summary> The target time to peak used to autotune the PID controllers. A vector of three times, in seconds, for each of the pitch, roll and yaw axes. Defaults to 3 seconds for each axis. </summary> </doc>
    pub fn set_time_to_peak(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_TimeToPeak"));

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

    /// <doc> <summary> Gains for the yaw PID controller. </summary> <remarks> When <see cref="M:SpaceCenter.AutoPilot.AutoTune" /> is true, these values are updated automatically, which will overwrite any manual changes. </remarks> </doc>
    pub fn set_yaw_pid_gains(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("AutoPilot_set_YawPIDGains"));

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
impl Camera {
    /// <doc> <summary> Default distance from the camera to the subject, in meters. </summary> </doc>
    pub fn get_default_distance(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_get_DefaultDistance"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The distance from the camera to the subject, in meters. A value between <see cref="M:SpaceCenter.Camera.MinDistance" /> and <see cref="M:SpaceCenter.Camera.MaxDistance" />. </summary> </doc>
    pub fn get_distance(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_get_Distance"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> In map mode, the celestial body that the camera is focussed on. Returns <c>null</c> if the camera is not focussed on a celestial body. Returns an error is the camera is not in map mode. </summary> </doc>
    pub fn get_focussed_body(&self, ) -> CallHandle<CelestialBody> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_get_FocussedBody"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> In map mode, the maneuver node that the camera is focussed on. Returns <c>null</c> if the camera is not focussed on a maneuver node. Returns an error is the camera is not in map mode. </summary> </doc>
    pub fn get_focussed_node(&self, ) -> CallHandle<Node> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_get_FocussedNode"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> In map mode, the vessel that the camera is focussed on. Returns <c>null</c> if the camera is not focussed on a vessel. Returns an error is the camera is not in map mode. </summary> </doc>
    pub fn get_focussed_vessel(&self, ) -> CallHandle<Vessel> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_get_FocussedVessel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The heading of the camera, in degrees. </summary> </doc>
    pub fn get_heading(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_get_Heading"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Maximum distance from the camera to the subject, in meters. </summary> </doc>
    pub fn get_max_distance(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_get_MaxDistance"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The maximum pitch of the camera. </summary> </doc>
    pub fn get_max_pitch(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_get_MaxPitch"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Minimum distance from the camera to the subject, in meters. </summary> </doc>
    pub fn get_min_distance(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_get_MinDistance"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The minimum pitch of the camera. </summary> </doc>
    pub fn get_min_pitch(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_get_MinPitch"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current mode of the camera. </summary> </doc>
    pub fn get_mode(&self, ) -> CallHandle<CameraMode> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_get_Mode"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The pitch of the camera, in degrees. A value between <see cref="M:SpaceCenter.Camera.MinPitch" /> and <see cref="M:SpaceCenter.Camera.MaxPitch" /></summary> </doc>
    pub fn get_pitch(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_get_Pitch"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The distance from the camera to the subject, in meters. A value between <see cref="M:SpaceCenter.Camera.MinDistance" /> and <see cref="M:SpaceCenter.Camera.MaxDistance" />. </summary> </doc>
    pub fn set_distance(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_set_Distance"));

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

    /// <doc> <summary> In map mode, the celestial body that the camera is focussed on. Returns <c>null</c> if the camera is not focussed on a celestial body. Returns an error is the camera is not in map mode. </summary> </doc>
    pub fn set_focussed_body(&self, p_value: &CelestialBody) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_set_FocussedBody"));

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

    /// <doc> <summary> In map mode, the maneuver node that the camera is focussed on. Returns <c>null</c> if the camera is not focussed on a maneuver node. Returns an error is the camera is not in map mode. </summary> </doc>
    pub fn set_focussed_node(&self, p_value: &Node) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_set_FocussedNode"));

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

    /// <doc> <summary> In map mode, the vessel that the camera is focussed on. Returns <c>null</c> if the camera is not focussed on a vessel. Returns an error is the camera is not in map mode. </summary> </doc>
    pub fn set_focussed_vessel(&self, p_value: &Vessel) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_set_FocussedVessel"));

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

    /// <doc> <summary> The heading of the camera, in degrees. </summary> </doc>
    pub fn set_heading(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_set_Heading"));

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

    /// <doc> <summary> The current mode of the camera. </summary> </doc>
    pub fn set_mode(&self, p_value: CameraMode) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_set_Mode"));

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

    /// <doc> <summary> The pitch of the camera, in degrees. A value between <see cref="M:SpaceCenter.Camera.MinPitch" /> and <see cref="M:SpaceCenter.Camera.MaxPitch" /></summary> </doc>
    pub fn set_pitch(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Camera_set_Pitch"));

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
impl CargoBay {
    /// <doc> <summary> Whether the cargo bay is open. </summary> </doc>
    pub fn get_open(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CargoBay_get_Open"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this cargo bay. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CargoBay_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the cargo bay. </summary> </doc>
    pub fn get_state(&self, ) -> CallHandle<CargoBayState> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CargoBay_get_State"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the cargo bay is open. </summary> </doc>
    pub fn set_open(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CargoBay_set_Open"));

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
impl CelestialBody {
    /// <doc> <summary> The altitude, in meters, of the given position in the given reference frame. </summary> <param name="position">Position as a vector.</param> <param name="referenceFrame">Reference frame for the position vector.</param> </doc>
    pub fn altitude_at_position(&self, p_position: (f64, f64, f64), p_reference_frame: &ReferenceFrame) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_AltitudeAtPosition"));

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
        arg2.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The angular velocity of the body in the specified reference frame. </summary> <returns>The angular velocity as a vector. The magnitude of the vector is the rotational speed of the body, in radians per second. The direction of the vector indicates the axis of rotation, using the right-hand rule.</returns> <param name="referenceFrame">The reference frame the returned angular velocity is in.</param> </doc>
    pub fn angular_velocity(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_AngularVelocity"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The atmospheric density at the given position, in <math>kg/m^3</math>, in the given reference frame. </summary> <param name="position">The position vector at which to measure the density.</param> <param name="referenceFrame">Reference frame that the position vector is in.</param> </doc>
    pub fn atmospheric_density_at_position(&self, p_position: (f64, f64, f64), p_reference_frame: &ReferenceFrame) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_AtmosphericDensityAtPosition"));

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
        arg2.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The height of the surface relative to mean sea level, in meters, at the given position. When over water, this is the height of the sea-bed and is therefore  negative value. </summary> <param name="latitude">Latitude in degrees.</param> <param name="longitude">Longitude in degrees.</param> </doc>
    pub fn bedrock_height(&self, p_latitude: f64, p_longitude: f64) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_BedrockHeight"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_latitude.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_longitude.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position of the surface at the given latitude and longitude, in the given reference frame. When over water, this is the position at the bottom of the sea-bed. </summary> <returns>Position as a vector.</returns> <param name="latitude">Latitude in degrees.</param> <param name="longitude">Longitude in degrees.</param> <param name="referenceFrame">Reference frame for the returned position vector.</param> </doc>
    pub fn bedrock_position(&self, p_latitude: f64, p_longitude: f64, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_BedrockPosition"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_latitude.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_longitude.encode_to_bytes().unwrap());
        arguments.push(arg2);

        let mut arg3 = krpc::Argument::new();
        arg3.set_position(3);
        arg3.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg3);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The biome at the given latitude and longitude, in degrees. </summary> </doc>
    pub fn biome_at(&self, p_latitude: f64, p_longitude: f64) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_BiomeAt"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_latitude.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_longitude.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Gets the air density, in <math>kg/m^3</math>, for the specified altitude above sea level, in meters. </summary> <remarks> This is an approximation, because actual calculations, taking sun exposure into account to compute air temperature, require us to know the exact point on the body where the density is to be computed (knowing the altitude is not enough). However, the difference is small for high altitudes, so it makes very little difference for trajectory prediction. </remarks> </doc>
    pub fn density_at(&self, p_altitude: f64) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_DensityAt"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_altitude.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The direction in which the north pole of the celestial body is pointing, in the specified reference frame. </summary> <returns>The direction as a unit vector.</returns> <param name="referenceFrame">The reference frame that the returned direction is in.</param> </doc>
    pub fn direction(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_Direction"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The latitude of the given position, in the given reference frame. </summary> <param name="position">Position as a vector.</param> <param name="referenceFrame">Reference frame for the position vector.</param> </doc>
    pub fn latitude_at_position(&self, p_position: (f64, f64, f64), p_reference_frame: &ReferenceFrame) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_LatitudeAtPosition"));

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
        arg2.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The longitude of the given position, in the given reference frame. </summary> <param name="position">Position as a vector.</param> <param name="referenceFrame">Reference frame for the position vector.</param> </doc>
    pub fn longitude_at_position(&self, p_position: (f64, f64, f64), p_reference_frame: &ReferenceFrame) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_LongitudeAtPosition"));

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
        arg2.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position at mean sea level at the given latitude and longitude, in the given reference frame. </summary> <returns>Position as a vector.</returns> <param name="latitude">Latitude in degrees.</param> <param name="longitude">Longitude in degrees.</param> <param name="referenceFrame">Reference frame for the returned position vector.</param> </doc>
    pub fn msl_position(&self, p_latitude: f64, p_longitude: f64, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_MSLPosition"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_latitude.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_longitude.encode_to_bytes().unwrap());
        arguments.push(arg2);

        let mut arg3 = krpc::Argument::new();
        arg3.set_position(3);
        arg3.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg3);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position of the center of the body, in the specified reference frame. </summary> <returns>The position as a vector.</returns> <param name="referenceFrame">The reference frame that the returned position vector is in.</param> </doc>
    pub fn position(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_Position"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position at the given latitude, longitude and altitude, in the given reference frame. </summary> <returns>Position as a vector.</returns> <param name="latitude">Latitude in degrees.</param> <param name="longitude">Longitude in degrees.</param> <param name="altitude">Altitude in meters above sea level.</param> <param name="referenceFrame">Reference frame for the returned position vector.</param> </doc>
    pub fn position_at_altitude(&self, p_latitude: f64, p_longitude: f64, p_altitude: f64, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_PositionAtAltitude"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_latitude.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_longitude.encode_to_bytes().unwrap());
        arguments.push(arg2);

        let mut arg3 = krpc::Argument::new();
        arg3.set_position(3);
        arg3.set_value(p_altitude.encode_to_bytes().unwrap());
        arguments.push(arg3);

        let mut arg4 = krpc::Argument::new();
        arg4.set_position(4);
        arg4.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg4);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Gets the air pressure, in Pascals, for the specified altitude above sea level, in meters. </summary> </doc>
    pub fn pressure_at(&self, p_altitude: f64) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_PressureAt"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_altitude.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rotation of the body, in the specified reference frame. </summary> <returns>The rotation as a quaternion of the form <math>(x, y, z, w)</math>.</returns> <param name="referenceFrame">The reference frame that the returned rotation is in.</param> </doc>
    pub fn rotation(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_Rotation"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The height of the surface relative to mean sea level, in meters, at the given position. When over water this is equal to 0. </summary> <param name="latitude">Latitude in degrees.</param> <param name="longitude">Longitude in degrees.</param> </doc>
    pub fn surface_height(&self, p_latitude: f64, p_longitude: f64) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_SurfaceHeight"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_latitude.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_longitude.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position of the surface at the given latitude and longitude, in the given reference frame. When over water, this is the position of the surface of the water. </summary> <returns>Position as a vector.</returns> <param name="latitude">Latitude in degrees.</param> <param name="longitude">Longitude in degrees.</param> <param name="referenceFrame">Reference frame for the returned position vector.</param> </doc>
    pub fn surface_position(&self, p_latitude: f64, p_longitude: f64, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_SurfacePosition"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_latitude.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_longitude.encode_to_bytes().unwrap());
        arguments.push(arg2);

        let mut arg3 = krpc::Argument::new();
        arg3.set_position(3);
        arg3.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg3);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The temperature on the body at the given position, in the given reference frame. </summary> <param name="position">Position as a vector.</param> <param name="referenceFrame">The reference frame that the position is in.</param> <remarks> This calculation is performed using the bodies current position, which means that the value could be wrong if you want to know the temperature in the far future. </remarks> </doc>
    pub fn temperature_at(&self, p_position: (f64, f64, f64), p_reference_frame: &ReferenceFrame) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_TemperatureAt"));

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
        arg2.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The linear velocity of the body, in the specified reference frame. </summary> <returns>The velocity as a vector. The vector points in the direction of travel, and its magnitude is the speed of the body in meters per second.</returns> <param name="referenceFrame">The reference frame that the returned velocity vector is in.</param> </doc>
    pub fn velocity(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_Velocity"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The depth of the atmosphere, in meters. </summary> </doc>
    pub fn get_atmosphere_depth(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_AtmosphereDepth"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The biomes present on this body. </summary> </doc>
    pub fn get_biomes(&self, ) -> CallHandle<HashSet<String>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_Biomes"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The equatorial radius of the body, in meters. </summary> </doc>
    pub fn get_equatorial_radius(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_EquatorialRadius"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The altitude, in meters, above which a vessel is considered to be flying "high" when doing science. </summary> </doc>
    pub fn get_flying_high_altitude_threshold(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_FlyingHighAltitudeThreshold"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Standard_gravitational_parameter">standard gravitational parameter</a> of the body in <math>m^3s^{-2}</math>. </summary> </doc>
    pub fn get_gravitational_parameter(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_GravitationalParameter"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary><c>true</c> if the body has an atmosphere. </summary> </doc>
    pub fn get_has_atmosphere(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_HasAtmosphere"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary><c>true</c> if there is oxygen in the atmosphere, required for air-breathing engines. </summary> </doc>
    pub fn get_has_atmospheric_oxygen(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_HasAtmosphericOxygen"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The initial rotation angle of the body (at UT 0), in radians. A value between 0 and <math>2\pi</math></summary> </doc>
    pub fn get_initial_rotation(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_InitialRotation"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The mass of the body, in kilograms. </summary> </doc>
    pub fn get_mass(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_Mass"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The name of the body. </summary> </doc>
    pub fn get_name(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_Name"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The reference frame that is fixed relative to this celestial body, and orientated in a fixed direction (it does not rotate with the body). <list type="bullet"><item><description>The origin is at the center of the body.</description></item><item><description>The axes do not rotate.</description></item><item><description>The x-axis points in an arbitrary direction through the equator.</description></item><item><description>The y-axis points from the center of the body towards the north pole.</description></item><item><description>The z-axis points in an arbitrary direction through the equator.</description></item></list></summary> </doc>
    pub fn get_non_rotating_reference_frame(&self, ) -> CallHandle<ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_NonRotatingReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The orbit of the body. </summary> </doc>
    pub fn get_orbit(&self, ) -> CallHandle<Orbit> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_Orbit"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The reference frame that is fixed relative to this celestial body, but orientated with the body's orbital prograde/normal/radial directions. <list type="bullet"><item><description>The origin is at the center of the body. </description></item><item><description>The axes rotate with the orbital prograde/normal/radial directions.</description></item><item><description>The x-axis points in the orbital anti-radial direction. </description></item><item><description>The y-axis points in the orbital prograde direction. </description></item><item><description>The z-axis points in the orbital normal direction. </description></item></list></summary> </doc>
    pub fn get_orbital_reference_frame(&self, ) -> CallHandle<ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_OrbitalReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The reference frame that is fixed relative to the celestial body. <list type="bullet"><item><description>The origin is at the center of the body. </description></item><item><description>The axes rotate with the body.</description></item><item><description>The x-axis points from the center of the body towards the intersection of the prime meridian and equator (the position at 0° longitude, 0° latitude).</description></item><item><description>The y-axis points from the center of the body towards the north pole.</description></item><item><description>The z-axis points from the center of the body towards the equator at 90°E longitude.</description></item></list></summary> </doc>
    pub fn get_reference_frame(&self, ) -> CallHandle<ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_ReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current rotation angle of the body, in radians. A value between 0 and <math>2\pi</math></summary> </doc>
    pub fn get_rotation_angle(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_RotationAngle"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The sidereal rotational period of the body, in seconds. </summary> </doc>
    pub fn get_rotational_period(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_RotationalPeriod"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rotational speed of the body, in radians per second. </summary> </doc>
    pub fn get_rotational_speed(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_RotationalSpeed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of celestial bodies that are in orbit around this celestial body. </summary> </doc>
    pub fn get_satellites(&self, ) -> CallHandle<Vec<CelestialBody>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_Satellites"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The altitude, in meters, above which a vessel is considered to be in "high" space when doing science. </summary> </doc>
    pub fn get_space_high_altitude_threshold(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_SpaceHighAltitudeThreshold"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The radius of the sphere of influence of the body, in meters. </summary> </doc>
    pub fn get_sphere_of_influence(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_SphereOfInfluence"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The acceleration due to gravity at sea level (mean altitude) on the body, in <math>m/s^2</math>. </summary> </doc>
    pub fn get_surface_gravity(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CelestialBody_get_SurfaceGravity"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl CommLink {
    /// <doc> <summary> Start point of the link. </summary> </doc>
    pub fn get_end(&self, ) -> CallHandle<CommNode> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CommLink_get_End"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Signal strength of the link. </summary> </doc>
    pub fn get_signal_strength(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CommLink_get_SignalStrength"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Start point of the link. </summary> </doc>
    pub fn get_start(&self, ) -> CallHandle<CommNode> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CommLink_get_Start"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The type of link. </summary> </doc>
    pub fn get_type(&self, ) -> CallHandle<CommLinkType> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CommLink_get_Type"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl CommNode {
    /// <doc> <summary> Whether the communication node is a control point, for example a manned vessel. </summary> </doc>
    pub fn get_is_control_point(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CommNode_get_IsControlPoint"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the communication node is on Kerbin. </summary> </doc>
    pub fn get_is_home(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CommNode_get_IsHome"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the communication node is a vessel. </summary> </doc>
    pub fn get_is_vessel(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CommNode_get_IsVessel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Name of the communication node. </summary> </doc>
    pub fn get_name(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CommNode_get_Name"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The vessel for this communication node. </summary> </doc>
    pub fn get_vessel(&self, ) -> CallHandle<Vessel> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CommNode_get_Vessel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl Comms {
    /// <doc> <summary> Whether the vessel can communicate with KSC. </summary> </doc>
    pub fn get_can_communicate(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Comms_get_CanCommunicate"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the vessel can transmit science data to KSC. </summary> </doc>
    pub fn get_can_transmit_science(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Comms_get_CanTransmitScience"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The communication path used to control the vessel. </summary> </doc>
    pub fn get_control_path(&self, ) -> CallHandle<Vec<CommLink>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Comms_get_ControlPath"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The combined power of all active antennae on the vessel. </summary> </doc>
    pub fn get_power(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Comms_get_Power"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Signal delay to KSC in seconds. </summary> </doc>
    pub fn get_signal_delay(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Comms_get_SignalDelay"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Signal strength to KSC. </summary> </doc>
    pub fn get_signal_strength(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Comms_get_SignalStrength"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl Contract {
    /// <doc> <summary> Accept an offered contract. </summary> </doc>
    pub fn accept(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_Accept"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Cancel an active contract. </summary> </doc>
    pub fn cancel(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_Cancel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Decline an offered contract. </summary> </doc>
    pub fn decline(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_Decline"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the contract is active. </summary> </doc>
    pub fn get_active(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_Active"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the contract can be canceled. </summary> </doc>
    pub fn get_can_be_canceled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_CanBeCanceled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the contract can be declined. </summary> </doc>
    pub fn get_can_be_declined(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_CanBeDeclined"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the contract can be failed. </summary> </doc>
    pub fn get_can_be_failed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_CanBeFailed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Description of the contract. </summary> </doc>
    pub fn get_description(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_Description"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the contract has been failed. </summary> </doc>
    pub fn get_failed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_Failed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Funds received when accepting the contract. </summary> </doc>
    pub fn get_funds_advance(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_FundsAdvance"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Funds received on completion of the contract. </summary> </doc>
    pub fn get_funds_completion(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_FundsCompletion"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Funds lost if the contract is failed. </summary> </doc>
    pub fn get_funds_failure(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_FundsFailure"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Keywords for the contract. </summary> </doc>
    pub fn get_keywords(&self, ) -> CallHandle<Vec<String>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_Keywords"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Notes for the contract. </summary> </doc>
    pub fn get_notes(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_Notes"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Parameters for the contract. </summary> </doc>
    pub fn get_parameters(&self, ) -> CallHandle<Vec<ContractParameter>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_Parameters"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the contract has been read. </summary> </doc>
    pub fn get_read(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_Read"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Reputation gained on completion of the contract. </summary> </doc>
    pub fn get_reputation_completion(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_ReputationCompletion"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Reputation lost if the contract is failed. </summary> </doc>
    pub fn get_reputation_failure(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_ReputationFailure"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Science gained on completion of the contract. </summary> </doc>
    pub fn get_science_completion(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_ScienceCompletion"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the contract has been seen. </summary> </doc>
    pub fn get_seen(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_Seen"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> State of the contract. </summary> </doc>
    pub fn get_state(&self, ) -> CallHandle<ContractState> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_State"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Synopsis for the contract. </summary> </doc>
    pub fn get_synopsis(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_Synopsis"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Title of the contract. </summary> </doc>
    pub fn get_title(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_Title"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Type of the contract. </summary> </doc>
    pub fn get_type(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Contract_get_Type"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl ContractManager {
    /// <doc> <summary> A list of all active contracts. </summary> </doc>
    pub fn get_active_contracts(&self, ) -> CallHandle<Vec<Contract>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractManager_get_ActiveContracts"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all contracts. </summary> </doc>
    pub fn get_all_contracts(&self, ) -> CallHandle<Vec<Contract>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractManager_get_AllContracts"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all completed contracts. </summary> </doc>
    pub fn get_completed_contracts(&self, ) -> CallHandle<Vec<Contract>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractManager_get_CompletedContracts"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all failed contracts. </summary> </doc>
    pub fn get_failed_contracts(&self, ) -> CallHandle<Vec<Contract>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractManager_get_FailedContracts"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all offered, but unaccepted, contracts. </summary> </doc>
    pub fn get_offered_contracts(&self, ) -> CallHandle<Vec<Contract>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractManager_get_OfferedContracts"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all contract types. </summary> </doc>
    pub fn get_types(&self, ) -> CallHandle<HashSet<String>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractManager_get_Types"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl ContractParameter {
    /// <doc> <summary> Child contract parameters. </summary> </doc>
    pub fn get_children(&self, ) -> CallHandle<Vec<ContractParameter>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractParameter_get_Children"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the parameter has been completed. </summary> </doc>
    pub fn get_completed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractParameter_get_Completed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the parameter has been failed. </summary> </doc>
    pub fn get_failed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractParameter_get_Failed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Funds received on completion of the contract parameter. </summary> </doc>
    pub fn get_funds_completion(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractParameter_get_FundsCompletion"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Funds lost if the contract parameter is failed. </summary> </doc>
    pub fn get_funds_failure(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractParameter_get_FundsFailure"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Notes for the parameter. </summary> </doc>
    pub fn get_notes(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractParameter_get_Notes"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the contract parameter is optional. </summary> </doc>
    pub fn get_optional(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractParameter_get_Optional"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Reputation gained on completion of the contract parameter. </summary> </doc>
    pub fn get_reputation_completion(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractParameter_get_ReputationCompletion"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Reputation lost if the contract parameter is failed. </summary> </doc>
    pub fn get_reputation_failure(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractParameter_get_ReputationFailure"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Science gained on completion of the contract parameter. </summary> </doc>
    pub fn get_science_completion(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractParameter_get_ScienceCompletion"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Title of the parameter. </summary> </doc>
    pub fn get_title(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ContractParameter_get_Title"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl Control {
    /// <doc> <summary> Activates the next stage. Equivalent to pressing the space bar in-game. </summary> <returns>A list of vessel objects that are jettisoned from the active vessel.</returns> <remarks> When called, the active vessel may change. It is therefore possible that, after calling this function, the object(s) returned by previous call(s) to <see cref="M:SpaceCenter.ActiveVessel" /> no longer refer to the active vessel. </remarks> </doc>
    pub fn activate_next_stage(&self, ) -> CallHandle<Vec<Vessel>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_ActivateNextStage"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Creates a maneuver node at the given universal time, and returns a <see cref="T:SpaceCenter.Node" /> object that can be used to modify it. Optionally sets the magnitude of the delta-v for the maneuver node in the prograde, normal and radial directions. </summary> <param name="ut">Universal time of the maneuver node.</param> <param name="prograde">Delta-v in the prograde direction.</param> <param name="normal">Delta-v in the normal direction.</param> <param name="radial">Delta-v in the radial direction.</param> </doc>
    pub fn add_node(&self, p_ut: f64, p_prograde: f32, p_normal: f32, p_radial: f32) -> CallHandle<Node> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_AddNode"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_ut.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_prograde.encode_to_bytes().unwrap());
        arguments.push(arg2);

        let mut arg3 = krpc::Argument::new();
        arg3.set_position(3);
        arg3.set_value(p_normal.encode_to_bytes().unwrap());
        arguments.push(arg3);

        let mut arg4 = krpc::Argument::new();
        arg4.set_position(4);
        arg4.set_value(p_radial.encode_to_bytes().unwrap());
        arguments.push(arg4);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns <c>true</c> if the given action group is enabled. </summary> <param name="group"> A number between 0 and 9 inclusive, or between 0 and 250 inclusive when the <a href="https://forum.kerbalspaceprogram.com/index.php?/topic/67235-122dec1016-action-groups-extended-250-action-groups-in-flight-editing-now-kosremotetech/">Extended Action Groups mod</a> is installed. </param> </doc>
    pub fn get_action_group(&self, p_group: u32) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_GetActionGroup"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_group.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Remove all maneuver nodes. </summary> </doc>
    pub fn remove_nodes(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_RemoveNodes"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Sets the state of the given action group. </summary> <param name="group"> A number between 0 and 9 inclusive, or between 0 and 250 inclusive when the <a href="https://forum.kerbalspaceprogram.com/index.php?/topic/67235-122dec1016-action-groups-extended-250-action-groups-in-flight-editing-now-kosremotetech/">Extended Action Groups mod</a> is installed. </param> <param name="state"></param> </doc>
    pub fn set_action_group(&self, p_group: u32, p_state: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_SetActionGroup"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_group.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_state.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Toggles the state of the given action group. </summary> <param name="group"> A number between 0 and 9 inclusive, or between 0 and 250 inclusive when the <a href="https://forum.kerbalspaceprogram.com/index.php?/topic/67235-122dec1016-action-groups-extended-250-action-groups-in-flight-editing-now-kosremotetech/">Extended Action Groups mod</a> is installed. </param> </doc>
    pub fn toggle_action_group(&self, p_group: u32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_ToggleActionGroup"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_group.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the abort action group. </summary> </doc>
    pub fn get_abort(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Abort"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns whether all antennas on the vessel are deployed, and sets the deployment state of all antennas. See <see cref="M:SpaceCenter.Antenna.Deployed" />. </summary> </doc>
    pub fn get_antennas(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Antennas"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the wheel brakes. </summary> </doc>
    pub fn get_brakes(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Brakes"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns whether any of the cargo bays on the vessel are open, and sets the open state of all cargo bays. See <see cref="M:SpaceCenter.CargoBay.Open" />. </summary> </doc>
    pub fn get_cargo_bays(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_CargoBays"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current stage of the vessel. Corresponds to the stage number in the in-game UI. </summary> </doc>
    pub fn get_current_stage(&self, ) -> CallHandle<i32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_CurrentStage"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the forward translational control. A value between -1 and 1. Equivalent to the h and n keys. </summary> </doc>
    pub fn get_forward(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Forward"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the landing gear/legs. </summary> </doc>
    pub fn get_gear(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Gear"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Sets the behavior of the pitch, yaw, roll and translation control inputs. When set to additive, these inputs are added to the vessels current inputs. This mode is the default. When set to override, these inputs (if non-zero) override the vessels inputs. This mode prevents keyboard control, or SAS, from interfering with the controls when they are set. </summary> </doc>
    pub fn get_input_mode(&self, ) -> CallHandle<ControlInputMode> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_InputMode"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns whether all of the air intakes on the vessel are open, and sets the open state of all air intakes. See <see cref="M:SpaceCenter.Intake.Open" />. </summary> </doc>
    pub fn get_intakes(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Intakes"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns whether all landing legs on the vessel are deployed, and sets the deployment state of all landing legs. Does not include wheels (for example landing gear). See <see cref="M:SpaceCenter.Leg.Deployed" />. </summary> </doc>
    pub fn get_legs(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Legs"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the lights. </summary> </doc>
    pub fn get_lights(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Lights"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns a list of all existing maneuver nodes, ordered by time from first to last. </summary> </doc>
    pub fn get_nodes(&self, ) -> CallHandle<Vec<Node>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Nodes"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns whether all parachutes on the vessel are deployed, and sets the deployment state of all parachutes. Cannot be set to <c>false</c>. See <see cref="M:SpaceCenter.Parachute.Deployed" />. </summary> </doc>
    pub fn get_parachutes(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Parachutes"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the pitch control. A value between -1 and 1. Equivalent to the w and s keys. </summary> </doc>
    pub fn get_pitch(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Pitch"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of RCS. </summary> </doc>
    pub fn get_rcs(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_RCS"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns whether all radiators on the vessel are deployed, and sets the deployment state of all radiators. See <see cref="M:SpaceCenter.Radiator.Deployed" />. </summary> </doc>
    pub fn get_radiators(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Radiators"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns whether all reactive wheels on the vessel are active, and sets the active state of all reaction wheels. See <see cref="M:SpaceCenter.ReactionWheel.Active" />. </summary> </doc>
    pub fn get_reaction_wheels(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_ReactionWheels"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns whether all of the resource harvesters on the vessel are deployed, and sets the deployment state of all resource harvesters. See <see cref="M:SpaceCenter.ResourceHarvester.Deployed" />. </summary> </doc>
    pub fn get_resource_harvesters(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_ResourceHarvesters"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns whether any of the resource harvesters on the vessel are active, and sets the active state of all resource harvesters. See <see cref="M:SpaceCenter.ResourceHarvester.Active" />. </summary> </doc>
    pub fn get_resource_harvesters_active(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_ResourceHarvestersActive"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the right translational control. A value between -1 and 1. Equivalent to the j and l keys. </summary> </doc>
    pub fn get_right(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Right"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the roll control. A value between -1 and 1. Equivalent to the q and e keys. </summary> </doc>
    pub fn get_roll(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Roll"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of SAS. </summary> <remarks>Equivalent to <see cref="M:SpaceCenter.AutoPilot.SAS" /></remarks> </doc>
    pub fn get_sas(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_SAS"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current <see cref="T:SpaceCenter.SASMode" />. These modes are equivalent to the mode buttons to the left of the navball that appear when SAS is enabled. </summary> <remarks>Equivalent to <see cref="M:SpaceCenter.AutoPilot.SASMode" /></remarks> </doc>
    pub fn get_sas_mode(&self, ) -> CallHandle<SASMode> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_SASMode"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns whether all solar panels on the vessel are deployed, and sets the deployment state of all solar panels. See <see cref="M:SpaceCenter.SolarPanel.Deployed" />. </summary> </doc>
    pub fn get_solar_panels(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_SolarPanels"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The source of the vessels control, for example by a kerbal or a probe core. </summary> </doc>
    pub fn get_source(&self, ) -> CallHandle<ControlSource> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Source"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current <see cref="T:SpaceCenter.SpeedMode" /> of the navball. This is the mode displayed next to the speed at the top of the navball. </summary> </doc>
    pub fn get_speed_mode(&self, ) -> CallHandle<SpeedMode> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_SpeedMode"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The control state of the vessel. </summary> </doc>
    pub fn get_state(&self, ) -> CallHandle<ControlState> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_State"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the throttle. A value between 0 and 1. </summary> </doc>
    pub fn get_throttle(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Throttle"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the up translational control. A value between -1 and 1. Equivalent to the i and k keys. </summary> </doc>
    pub fn get_up(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Up"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the wheel steering. A value between -1 and 1. A value of 1 steers to the left, and a value of -1 steers to the right. </summary> </doc>
    pub fn get_wheel_steering(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_WheelSteering"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the wheel throttle. A value between -1 and 1. A value of 1 rotates the wheels forwards, a value of -1 rotates the wheels backwards. </summary> </doc>
    pub fn get_wheel_throttle(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_WheelThrottle"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns whether all wheels on the vessel are deployed, and sets the deployment state of all wheels. Does not include landing legs. See <see cref="M:SpaceCenter.Wheel.Deployed" />. </summary> </doc>
    pub fn get_wheels(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Wheels"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the yaw control. A value between -1 and 1. Equivalent to the a and d keys. </summary> </doc>
    pub fn get_yaw(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_get_Yaw"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the abort action group. </summary> </doc>
    pub fn set_abort(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Abort"));

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

    /// <doc> <summary> Returns whether all antennas on the vessel are deployed, and sets the deployment state of all antennas. See <see cref="M:SpaceCenter.Antenna.Deployed" />. </summary> </doc>
    pub fn set_antennas(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Antennas"));

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

    /// <doc> <summary> The state of the wheel brakes. </summary> </doc>
    pub fn set_brakes(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Brakes"));

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

    /// <doc> <summary> Returns whether any of the cargo bays on the vessel are open, and sets the open state of all cargo bays. See <see cref="M:SpaceCenter.CargoBay.Open" />. </summary> </doc>
    pub fn set_cargo_bays(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_CargoBays"));

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

    /// <doc> <summary> The state of the forward translational control. A value between -1 and 1. Equivalent to the h and n keys. </summary> </doc>
    pub fn set_forward(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Forward"));

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

    /// <doc> <summary> The state of the landing gear/legs. </summary> </doc>
    pub fn set_gear(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Gear"));

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

    /// <doc> <summary> Sets the behavior of the pitch, yaw, roll and translation control inputs. When set to additive, these inputs are added to the vessels current inputs. This mode is the default. When set to override, these inputs (if non-zero) override the vessels inputs. This mode prevents keyboard control, or SAS, from interfering with the controls when they are set. </summary> </doc>
    pub fn set_input_mode(&self, p_value: ControlInputMode) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_InputMode"));

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

    /// <doc> <summary> Returns whether all of the air intakes on the vessel are open, and sets the open state of all air intakes. See <see cref="M:SpaceCenter.Intake.Open" />. </summary> </doc>
    pub fn set_intakes(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Intakes"));

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

    /// <doc> <summary> Returns whether all landing legs on the vessel are deployed, and sets the deployment state of all landing legs. Does not include wheels (for example landing gear). See <see cref="M:SpaceCenter.Leg.Deployed" />. </summary> </doc>
    pub fn set_legs(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Legs"));

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

    /// <doc> <summary> The state of the lights. </summary> </doc>
    pub fn set_lights(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Lights"));

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

    /// <doc> <summary> Returns whether all parachutes on the vessel are deployed, and sets the deployment state of all parachutes. Cannot be set to <c>false</c>. See <see cref="M:SpaceCenter.Parachute.Deployed" />. </summary> </doc>
    pub fn set_parachutes(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Parachutes"));

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

    /// <doc> <summary> The state of the pitch control. A value between -1 and 1. Equivalent to the w and s keys. </summary> </doc>
    pub fn set_pitch(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Pitch"));

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

    /// <doc> <summary> The state of RCS. </summary> </doc>
    pub fn set_rcs(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_RCS"));

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

    /// <doc> <summary> Returns whether all radiators on the vessel are deployed, and sets the deployment state of all radiators. See <see cref="M:SpaceCenter.Radiator.Deployed" />. </summary> </doc>
    pub fn set_radiators(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Radiators"));

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

    /// <doc> <summary> Returns whether all reactive wheels on the vessel are active, and sets the active state of all reaction wheels. See <see cref="M:SpaceCenter.ReactionWheel.Active" />. </summary> </doc>
    pub fn set_reaction_wheels(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_ReactionWheels"));

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

    /// <doc> <summary> Returns whether all of the resource harvesters on the vessel are deployed, and sets the deployment state of all resource harvesters. See <see cref="M:SpaceCenter.ResourceHarvester.Deployed" />. </summary> </doc>
    pub fn set_resource_harvesters(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_ResourceHarvesters"));

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

    /// <doc> <summary> Returns whether any of the resource harvesters on the vessel are active, and sets the active state of all resource harvesters. See <see cref="M:SpaceCenter.ResourceHarvester.Active" />. </summary> </doc>
    pub fn set_resource_harvesters_active(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_ResourceHarvestersActive"));

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

    /// <doc> <summary> The state of the right translational control. A value between -1 and 1. Equivalent to the j and l keys. </summary> </doc>
    pub fn set_right(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Right"));

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

    /// <doc> <summary> The state of the roll control. A value between -1 and 1. Equivalent to the q and e keys. </summary> </doc>
    pub fn set_roll(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Roll"));

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

    /// <doc> <summary> The state of SAS. </summary> <remarks>Equivalent to <see cref="M:SpaceCenter.AutoPilot.SAS" /></remarks> </doc>
    pub fn set_sas(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_SAS"));

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

    /// <doc> <summary> The current <see cref="T:SpaceCenter.SASMode" />. These modes are equivalent to the mode buttons to the left of the navball that appear when SAS is enabled. </summary> <remarks>Equivalent to <see cref="M:SpaceCenter.AutoPilot.SASMode" /></remarks> </doc>
    pub fn set_sas_mode(&self, p_value: SASMode) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_SASMode"));

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

    /// <doc> <summary> Returns whether all solar panels on the vessel are deployed, and sets the deployment state of all solar panels. See <see cref="M:SpaceCenter.SolarPanel.Deployed" />. </summary> </doc>
    pub fn set_solar_panels(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_SolarPanels"));

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

    /// <doc> <summary> The current <see cref="T:SpaceCenter.SpeedMode" /> of the navball. This is the mode displayed next to the speed at the top of the navball. </summary> </doc>
    pub fn set_speed_mode(&self, p_value: SpeedMode) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_SpeedMode"));

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

    /// <doc> <summary> The state of the throttle. A value between 0 and 1. </summary> </doc>
    pub fn set_throttle(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Throttle"));

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

    /// <doc> <summary> The state of the up translational control. A value between -1 and 1. Equivalent to the i and k keys. </summary> </doc>
    pub fn set_up(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Up"));

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

    /// <doc> <summary> The state of the wheel steering. A value between -1 and 1. A value of 1 steers to the left, and a value of -1 steers to the right. </summary> </doc>
    pub fn set_wheel_steering(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_WheelSteering"));

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

    /// <doc> <summary> The state of the wheel throttle. A value between -1 and 1. A value of 1 rotates the wheels forwards, a value of -1 rotates the wheels backwards. </summary> </doc>
    pub fn set_wheel_throttle(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_WheelThrottle"));

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

    /// <doc> <summary> Returns whether all wheels on the vessel are deployed, and sets the deployment state of all wheels. Does not include landing legs. See <see cref="M:SpaceCenter.Wheel.Deployed" />. </summary> </doc>
    pub fn set_wheels(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Wheels"));

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

    /// <doc> <summary> The state of the yaw control. A value between -1 and 1. Equivalent to the a and d keys. </summary> </doc>
    pub fn set_yaw(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Control_set_Yaw"));

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
impl ControlSurface {
    /// <doc> <summary> The authority limiter for the control surface, which controls how far the control surface will move. </summary> </doc>
    pub fn get_authority_limiter(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ControlSurface_get_AuthorityLimiter"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The available torque, in Newton meters, that can be produced by this control surface, in the positive and negative pitch, roll and yaw axes of the vessel. These axes correspond to the coordinate axes of the <see cref="M:SpaceCenter.Vessel.ReferenceFrame" />. </summary> </doc>
    pub fn get_available_torque(&self, ) -> CallHandle<((f64, f64, f64), (f64, f64, f64))> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ControlSurface_get_AvailableTorque"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the control surface has been fully deployed. </summary> </doc>
    pub fn get_deployed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ControlSurface_get_Deployed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the control surface movement is inverted. </summary> </doc>
    pub fn get_inverted(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ControlSurface_get_Inverted"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this control surface. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ControlSurface_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the control surface has pitch control enabled. </summary> </doc>
    pub fn get_pitch_enabled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ControlSurface_get_PitchEnabled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the control surface has roll control enabled. </summary> </doc>
    pub fn get_roll_enabled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ControlSurface_get_RollEnabled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Surface area of the control surface in <math>m^2</math>. </summary> </doc>
    pub fn get_surface_area(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ControlSurface_get_SurfaceArea"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the control surface has yaw control enabled. </summary> </doc>
    pub fn get_yaw_enabled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ControlSurface_get_YawEnabled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The authority limiter for the control surface, which controls how far the control surface will move. </summary> </doc>
    pub fn set_authority_limiter(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ControlSurface_set_AuthorityLimiter"));

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

    /// <doc> <summary> Whether the control surface has been fully deployed. </summary> </doc>
    pub fn set_deployed(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ControlSurface_set_Deployed"));

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

    /// <doc> <summary> Whether the control surface movement is inverted. </summary> </doc>
    pub fn set_inverted(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ControlSurface_set_Inverted"));

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

    /// <doc> <summary> Whether the control surface has pitch control enabled. </summary> </doc>
    pub fn set_pitch_enabled(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ControlSurface_set_PitchEnabled"));

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

    /// <doc> <summary> Whether the control surface has roll control enabled. </summary> </doc>
    pub fn set_roll_enabled(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ControlSurface_set_RollEnabled"));

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

    /// <doc> <summary> Whether the control surface has yaw control enabled. </summary> </doc>
    pub fn set_yaw_enabled(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ControlSurface_set_YawEnabled"));

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
impl CrewMember {
    /// <doc> <summary> Whether the crew member is a badass. </summary> </doc>
    pub fn get_badass(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CrewMember_get_Badass"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The crew members courage. </summary> </doc>
    pub fn get_courage(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CrewMember_get_Courage"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The crew members experience. </summary> </doc>
    pub fn get_experience(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CrewMember_get_Experience"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The crew members name. </summary> </doc>
    pub fn get_name(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CrewMember_get_Name"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the crew member is on a mission. </summary> </doc>
    pub fn get_on_mission(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CrewMember_get_OnMission"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The crew members stupidity. </summary> </doc>
    pub fn get_stupidity(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CrewMember_get_Stupidity"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The type of crew member. </summary> </doc>
    pub fn get_type(&self, ) -> CallHandle<CrewMemberType> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CrewMember_get_Type"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the crew member is a veteran. </summary> </doc>
    pub fn get_veteran(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CrewMember_get_Veteran"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the crew member is a badass. </summary> </doc>
    pub fn set_badass(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CrewMember_set_Badass"));

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

    /// <doc> <summary> The crew members courage. </summary> </doc>
    pub fn set_courage(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CrewMember_set_Courage"));

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

    /// <doc> <summary> The crew members experience. </summary> </doc>
    pub fn set_experience(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CrewMember_set_Experience"));

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

    /// <doc> <summary> The crew members name. </summary> </doc>
    pub fn set_name(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CrewMember_set_Name"));

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

    /// <doc> <summary> The crew members stupidity. </summary> </doc>
    pub fn set_stupidity(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CrewMember_set_Stupidity"));

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

    /// <doc> <summary> Whether the crew member is a veteran. </summary> </doc>
    pub fn set_veteran(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("CrewMember_set_Veteran"));

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
impl Decoupler {
    /// <doc> <summary> Fires the decoupler. Returns the new vessel created when the decoupler fires. Throws an exception if the decoupler has already fired. </summary> <remarks> When called, the active vessel may change. It is therefore possible that, after calling this function, the object(s) returned by previous call(s) to <see cref="M:SpaceCenter.ActiveVessel" /> no longer refer to the active vessel. </remarks> </doc>
    pub fn decouple(&self, ) -> CallHandle<Vessel> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Decoupler_Decouple"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the decoupler has fired. </summary> </doc>
    pub fn get_decoupled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Decoupler_get_Decoupled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The impulse that the decoupler imparts when it is fired, in Newton seconds. </summary> </doc>
    pub fn get_impulse(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Decoupler_get_Impulse"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this decoupler. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Decoupler_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the decoupler is enabled in the staging sequence. </summary> </doc>
    pub fn get_staged(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Decoupler_get_Staged"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl DockingPort {
    /// <doc> <summary> The direction that docking port points in, in the given reference frame. </summary> <returns>The direction as a unit vector.</returns> <param name="referenceFrame">The reference frame that the returned direction is in.</param> </doc>
    pub fn direction(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("DockingPort_Direction"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position of the docking port, in the given reference frame. </summary> <returns>The position as a vector.</returns> <param name="referenceFrame">The reference frame that the returned position vector is in.</param> </doc>
    pub fn position(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("DockingPort_Position"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rotation of the docking port, in the given reference frame. </summary> <returns>The rotation as a quaternion of the form <math>(x, y, z, w)</math>.</returns> <param name="referenceFrame">The reference frame that the returned rotation is in.</param> </doc>
    pub fn rotation(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("DockingPort_Rotation"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Undocks the docking port and returns the new <see cref="T:SpaceCenter.Vessel" /> that is created. This method can be called for either docking port in a docked pair. Throws an exception if the docking port is not docked to anything. </summary> <remarks> When called, the active vessel may change. It is therefore possible that, after calling this function, the object(s) returned by previous call(s) to <see cref="M:SpaceCenter.ActiveVessel" /> no longer refer to the active vessel. </remarks> </doc>
    pub fn undock(&self, ) -> CallHandle<Vessel> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("DockingPort_Undock"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part that this docking port is docked to. Returns <c>null</c> if this docking port is not docked to anything. </summary> </doc>
    pub fn get_docked_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("DockingPort_get_DockedPart"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the docking port has a shield. </summary> </doc>
    pub fn get_has_shield(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("DockingPort_get_HasShield"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this docking port. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("DockingPort_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The distance a docking port must move away when it undocks before it becomes ready to dock with another port, in meters. </summary> </doc>
    pub fn get_reengage_distance(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("DockingPort_get_ReengageDistance"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The reference frame that is fixed relative to this docking port, and oriented with the port. <list type="bullet"><item><description>The origin is at the position of the docking port. </description></item><item><description>The axes rotate with the docking port.</description></item><item><description>The x-axis points out to the right side of the docking port. </description></item><item><description>The y-axis points in the direction the docking port is facing. </description></item><item><description>The z-axis points out of the bottom off the docking port. </description></item></list></summary> <remarks> This reference frame is not necessarily equivalent to the reference frame for the part, returned by <see cref="M:SpaceCenter.Part.ReferenceFrame" />. </remarks> </doc>
    pub fn get_reference_frame(&self, ) -> CallHandle<ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("DockingPort_get_ReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the docking ports shield, if it has one.  Returns <c>true</c> if the docking port has a shield, and the shield is closed. Otherwise returns <c>false</c>. When set to <c>true</c>, the shield is closed, and when set to <c>false</c> the shield is opened. If the docking port does not have a shield, setting this attribute has no effect. </summary> </doc>
    pub fn get_shielded(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("DockingPort_get_Shielded"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current state of the docking port. </summary> </doc>
    pub fn get_state(&self, ) -> CallHandle<DockingPortState> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("DockingPort_get_State"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the docking ports shield, if it has one.  Returns <c>true</c> if the docking port has a shield, and the shield is closed. Otherwise returns <c>false</c>. When set to <c>true</c>, the shield is closed, and when set to <c>false</c> the shield is opened. If the docking port does not have a shield, setting this attribute has no effect. </summary> </doc>
    pub fn set_shielded(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("DockingPort_set_Shielded"));

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
impl Engine {
    /// <doc> <summary> Toggle the current engine mode. </summary> </doc>
    pub fn toggle_mode(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_ToggleMode"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the engine is active. Setting this attribute may have no effect, depending on <see cref="M:SpaceCenter.Engine.CanShutdown" /> and <see cref="M:SpaceCenter.Engine.CanRestart" />. </summary> </doc>
    pub fn get_active(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_Active"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the engine will automatically switch modes. </summary> </doc>
    pub fn get_auto_mode_switch(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_AutoModeSwitch"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The amount of thrust, in Newtons, that would be produced by the engine when activated and with its throttle set to 100%. Returns zero if the engine does not have any fuel. Takes the engine's current <see cref="M:SpaceCenter.Engine.ThrustLimit" /> and atmospheric conditions into account. </summary> </doc>
    pub fn get_available_thrust(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_AvailableThrust"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The available torque, in Newton meters, that can be produced by this engine, in the positive and negative pitch, roll and yaw axes of the vessel. These axes correspond to the coordinate axes of the <see cref="M:SpaceCenter.Vessel.ReferenceFrame" />. Returns zero if the engine is inactive, or not gimballed. </summary> </doc>
    pub fn get_available_torque(&self, ) -> CallHandle<((f64, f64, f64), (f64, f64, f64))> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_AvailableTorque"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the engine can be restarted once shutdown. If the engine cannot be shutdown, returns <c>false</c>. For example, this is <c>true</c> for liquid fueled rockets and <c>false</c> for solid rocket boosters. </summary> </doc>
    pub fn get_can_restart(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_CanRestart"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the engine can be shutdown once activated. For example, this is <c>true</c> for liquid fueled rockets and <c>false</c> for solid rocket boosters. </summary> </doc>
    pub fn get_can_shutdown(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_CanShutdown"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The gimbal limiter of the engine. A value between 0 and 1. Returns 0 if the gimbal is locked. </summary> </doc>
    pub fn get_gimbal_limit(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_GimbalLimit"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the engines gimbal is locked in place. Setting this attribute has no effect if the engine is not gimballed. </summary> </doc>
    pub fn get_gimbal_locked(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_GimbalLocked"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The range over which the gimbal can move, in degrees. Returns 0 if the engine is not gimballed. </summary> </doc>
    pub fn get_gimbal_range(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_GimbalRange"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the engine is gimballed. </summary> </doc>
    pub fn get_gimballed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_Gimballed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the engine has any fuel available. </summary> <remarks> The engine must be activated for this property to update correctly. </remarks> </doc>
    pub fn get_has_fuel(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_HasFuel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the engine has multiple modes of operation. </summary> </doc>
    pub fn get_has_modes(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_HasModes"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The specific impulse of the engine at sea level on Kerbin, in seconds. </summary> </doc>
    pub fn get_kerbin_sea_level_specific_impulse(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_KerbinSeaLevelSpecificImpulse"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The amount of thrust, in Newtons, that would be produced by the engine when activated and fueled, with its throttle and throttle limiter set to 100%. </summary> </doc>
    pub fn get_max_thrust(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_MaxThrust"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The maximum amount of thrust that can be produced by the engine in a vacuum, in Newtons. This is the amount of thrust produced by the engine when activated, <see cref="M:SpaceCenter.Engine.ThrustLimit" /> is set to 100%, the main vessel's throttle is set to 100% and the engine is in a vacuum. </summary> </doc>
    pub fn get_max_vacuum_thrust(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_MaxVacuumThrust"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The name of the current engine mode. </summary> </doc>
    pub fn get_mode(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_Mode"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The available modes for the engine. A dictionary mapping mode names to <see cref="T:SpaceCenter.Engine" /> objects. </summary> </doc>
    pub fn get_modes(&self, ) -> CallHandle<HashMap<String, Engine>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_Modes"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this engine. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The names of the propellants that the engine consumes. </summary> </doc>
    pub fn get_propellant_names(&self, ) -> CallHandle<Vec<String>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_PropellantNames"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The ratio of resources that the engine consumes. A dictionary mapping resource names to the ratio at which they are consumed by the engine. </summary> <remarks> For example, if the ratios are 0.6 for LiquidFuel and 0.4 for Oxidizer, then for every 0.6 units of LiquidFuel that the engine burns, it will burn 0.4 units of Oxidizer. </remarks> </doc>
    pub fn get_propellant_ratios(&self, ) -> CallHandle<HashMap<String, f32>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_PropellantRatios"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The propellants that the engine consumes. </summary> </doc>
    pub fn get_propellants(&self, ) -> CallHandle<Vec<Propellant>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_Propellants"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current specific impulse of the engine, in seconds. Returns zero if the engine is not active. </summary> </doc>
    pub fn get_specific_impulse(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_SpecificImpulse"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current throttle setting for the engine. A value between 0 and 1. This is not necessarily the same as the vessel's main throttle setting, as some engines take time to adjust their throttle (such as jet engines). </summary> </doc>
    pub fn get_throttle(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_Throttle"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the <see cref="M:SpaceCenter.Control.Throttle" /> affects the engine. For example, this is <c>true</c> for liquid fueled rockets, and <c>false</c> for solid rocket boosters. </summary> </doc>
    pub fn get_throttle_locked(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_ThrottleLocked"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current amount of thrust being produced by the engine, in Newtons. </summary> </doc>
    pub fn get_thrust(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_Thrust"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The thrust limiter of the engine. A value between 0 and 1. Setting this attribute may have no effect, for example the thrust limit for a solid rocket booster cannot be changed in flight. </summary> </doc>
    pub fn get_thrust_limit(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_ThrustLimit"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The components of the engine that generate thrust. </summary> <remarks> For example, this corresponds to the rocket nozzel on a solid rocket booster, or the individual nozzels on a RAPIER engine. The overall thrust produced by the engine, as reported by <see cref="M:SpaceCenter.Engine.AvailableThrust" />, <see cref="M:SpaceCenter.Engine.MaxThrust" /> and others, is the sum of the thrust generated by each thruster. </remarks> </doc>
    pub fn get_thrusters(&self, ) -> CallHandle<Vec<Thruster>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_Thrusters"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The vacuum specific impulse of the engine, in seconds. </summary> </doc>
    pub fn get_vacuum_specific_impulse(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_get_VacuumSpecificImpulse"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the engine is active. Setting this attribute may have no effect, depending on <see cref="M:SpaceCenter.Engine.CanShutdown" /> and <see cref="M:SpaceCenter.Engine.CanRestart" />. </summary> </doc>
    pub fn set_active(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_set_Active"));

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

    /// <doc> <summary> Whether the engine will automatically switch modes. </summary> </doc>
    pub fn set_auto_mode_switch(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_set_AutoModeSwitch"));

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

    /// <doc> <summary> The gimbal limiter of the engine. A value between 0 and 1. Returns 0 if the gimbal is locked. </summary> </doc>
    pub fn set_gimbal_limit(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_set_GimbalLimit"));

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

    /// <doc> <summary> Whether the engines gimbal is locked in place. Setting this attribute has no effect if the engine is not gimballed. </summary> </doc>
    pub fn set_gimbal_locked(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_set_GimbalLocked"));

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

    /// <doc> <summary> The name of the current engine mode. </summary> </doc>
    pub fn set_mode(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_set_Mode"));

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

    /// <doc> <summary> The thrust limiter of the engine. A value between 0 and 1. Setting this attribute may have no effect, for example the thrust limit for a solid rocket booster cannot be changed in flight. </summary> </doc>
    pub fn set_thrust_limit(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Engine_set_ThrustLimit"));

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
impl Experiment {
    /// <doc> <summary> Dump the experimental data contained by the experiment. </summary> </doc>
    pub fn dump(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Experiment_Dump"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Reset the experiment. </summary> </doc>
    pub fn reset(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Experiment_Reset"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Run the experiment. </summary> </doc>
    pub fn run(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Experiment_Run"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Transmit all experimental data contained by this part. </summary> </doc>
    pub fn transmit(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Experiment_Transmit"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Determines if the experiment is available given the current conditions. </summary> </doc>
    pub fn get_available(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Experiment_get_Available"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The name of the biome the experiment is currently in. </summary> </doc>
    pub fn get_biome(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Experiment_get_Biome"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The data contained in this experiment. </summary> </doc>
    pub fn get_data(&self, ) -> CallHandle<Vec<ScienceData>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Experiment_get_Data"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the experiment has been deployed. </summary> </doc>
    pub fn get_deployed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Experiment_get_Deployed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the experiment contains data. </summary> </doc>
    pub fn get_has_data(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Experiment_get_HasData"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the experiment is inoperable. </summary> </doc>
    pub fn get_inoperable(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Experiment_get_Inoperable"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this experiment. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Experiment_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the experiment can be re-run. </summary> </doc>
    pub fn get_rerunnable(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Experiment_get_Rerunnable"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Containing information on the corresponding specific science result for the current conditions. Returns <c>null</c> if the experiment is unavailable. </summary> </doc>
    pub fn get_science_subject(&self, ) -> CallHandle<ScienceSubject> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Experiment_get_ScienceSubject"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl Fairing {
    /// <doc> <summary> Jettison the fairing. Has no effect if it has already been jettisoned. </summary> </doc>
    pub fn jettison(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Fairing_Jettison"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the fairing has been jettisoned. </summary> </doc>
    pub fn get_jettisoned(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Fairing_get_Jettisoned"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this fairing. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Fairing_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl Flight {
    /// <doc> <summary> Simulate and return the total aerodynamic forces acting on the vessel, if it where to be traveling with the given velocity at the given position in the atmosphere of the given celestial body. </summary> <returns>A vector pointing in the direction that the force acts, with its magnitude equal to the strength of the force in Newtons.</returns> </doc>
    pub fn simulate_aerodynamic_force_at(&self, p_body: &CelestialBody, p_position: (f64, f64, f64), p_velocity: (f64, f64, f64)) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_SimulateAerodynamicForceAt"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_body.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_position.encode_to_bytes().unwrap());
        arguments.push(arg2);

        let mut arg3 = krpc::Argument::new();
        arg3.set_position(3);
        arg3.set_value(p_velocity.encode_to_bytes().unwrap());
        arguments.push(arg3);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The total aerodynamic forces acting on the vessel, in reference frame <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> <returns>A vector pointing in the direction that the force acts, with its magnitude equal to the strength of the force in Newtons.</returns> </doc>
    pub fn get_aerodynamic_force(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_AerodynamicForce"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The pitch angle between the orientation of the vessel and its velocity vector, in degrees. </summary> </doc>
    pub fn get_angle_of_attack(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_AngleOfAttack"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The direction opposite to the normal of the vessels orbit, in the reference frame <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> <returns>The direction as a unit vector.</returns> </doc>
    pub fn get_anti_normal(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_AntiNormal"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The direction opposite to the radial direction of the vessels orbit, in the reference frame <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> <returns>The direction as a unit vector.</returns> </doc>
    pub fn get_anti_radial(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_AntiRadial"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current density of the atmosphere around the vessel, in <math>kg/m^3</math>. </summary> </doc>
    pub fn get_atmosphere_density(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_AtmosphereDensity"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Ballistic_coefficient">ballistic coefficient</a>. </summary> <remarks> Requires <a href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>. </remarks> </doc>
    pub fn get_ballistic_coefficient(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_BallisticCoefficient"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The altitude above the surface of the body, in meters. When over water, this is the altitude above the sea floor. Measured from the center of mass of the vessel. </summary> </doc>
    pub fn get_bedrock_altitude(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_BedrockAltitude"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position of the center of mass of the vessel, in the reference frame <see cref="T:SpaceCenter.ReferenceFrame" /></summary> <returns>The position as a vector.</returns> </doc>
    pub fn get_center_of_mass(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_CenterOfMass"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The direction that the vessel is pointing in, in the reference frame <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> <returns>The direction as a unit vector.</returns> </doc>
    pub fn get_direction(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Direction"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Aerodynamic_force">aerodynamic drag</a> currently acting on the vessel. </summary> <returns>A vector pointing in the direction of the force, with its magnitude equal to the strength of the force in Newtons.</returns> </doc>
    pub fn get_drag(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Drag"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The coefficient of drag. This is the amount of drag produced by the vessel. It depends on air speed, air density and wing area. </summary> <remarks> Requires <a href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>. </remarks> </doc>
    pub fn get_drag_coefficient(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_DragCoefficient"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The dynamic pressure acting on the vessel, in Pascals. This is a measure of the strength of the aerodynamic forces. It is equal to <math>\frac{1}{2} . \mbox{air density} . \mbox{velocity}^2</math>. It is commonly denoted <math>Q</math>. </summary> </doc>
    pub fn get_dynamic_pressure(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_DynamicPressure"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The elevation of the terrain under the vessel, in meters. This is the height of the terrain above sea level, and is negative when the vessel is over the sea. </summary> </doc>
    pub fn get_elevation(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Elevation"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Equivalent_airspeed">equivalent air speed</a> of the vessel, in meters per second. </summary> </doc>
    pub fn get_equivalent_air_speed(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_EquivalentAirSpeed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current G force acting on the vessel in <math>g</math>. </summary> </doc>
    pub fn get_g_force(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_GForce"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The heading of the vessel (its angle relative to north), in degrees. A value between 0° and 360°. </summary> </doc>
    pub fn get_heading(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Heading"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The horizontal speed of the vessel in meters per second, in the reference frame <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> </doc>
    pub fn get_horizontal_speed(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_HorizontalSpeed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Latitude">latitude</a> of the vessel for the body being orbited, in degrees. </summary> </doc>
    pub fn get_latitude(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Latitude"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Aerodynamic_force">aerodynamic lift</a> currently acting on the vessel. </summary> <returns>A vector pointing in the direction that the force acts, with its magnitude equal to the strength of the force in Newtons.</returns> </doc>
    pub fn get_lift(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Lift"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The coefficient of lift. This is the amount of lift produced by the vessel, and depends on air speed, air density and wing area. </summary> <remarks> Requires <a href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>. </remarks> </doc>
    pub fn get_lift_coefficient(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_LiftCoefficient"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Longitude">longitude</a> of the vessel for the body being orbited, in degrees. </summary> </doc>
    pub fn get_longitude(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Longitude"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The speed of the vessel, in multiples of the speed of sound. </summary> </doc>
    pub fn get_mach(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Mach"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The altitude above sea level, in meters. Measured from the center of mass of the vessel. </summary> </doc>
    pub fn get_mean_altitude(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_MeanAltitude"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The direction normal to the vessels orbit, in the reference frame <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> <returns>The direction as a unit vector.</returns> </doc>
    pub fn get_normal(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Normal"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The pitch of the vessel relative to the horizon, in degrees. A value between -90° and +90°. </summary> </doc>
    pub fn get_pitch(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Pitch"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The prograde direction of the vessels orbit, in the reference frame <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> <returns>The direction as a unit vector.</returns> </doc>
    pub fn get_prograde(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Prograde"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The radial direction of the vessels orbit, in the reference frame <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> <returns>The direction as a unit vector.</returns> </doc>
    pub fn get_radial(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Radial"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The retrograde direction of the vessels orbit, in the reference frame <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> <returns>The direction as a unit vector.</returns> </doc>
    pub fn get_retrograde(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Retrograde"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The vessels Reynolds number. </summary> <remarks> Requires <a href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>. </remarks> </doc>
    pub fn get_reynolds_number(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_ReynoldsNumber"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The roll of the vessel relative to the horizon, in degrees. A value between -180° and +180°. </summary> </doc>
    pub fn get_roll(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Roll"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rotation of the vessel, in the reference frame <see cref="T:SpaceCenter.ReferenceFrame" /></summary> <returns>The rotation as a quaternion of the form <math>(x, y, z, w)</math>.</returns> </doc>
    pub fn get_rotation(&self, ) -> CallHandle<(f64, f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Rotation"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The yaw angle between the orientation of the vessel and its velocity vector, in degrees. </summary> </doc>
    pub fn get_sideslip_angle(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_SideslipAngle"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The speed of the vessel in meters per second, in the reference frame <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> </doc>
    pub fn get_speed(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Speed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The speed of sound, in the atmosphere around the vessel, in <math>m/s</math>. </summary> </doc>
    pub fn get_speed_of_sound(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_SpeedOfSound"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current amount of stall, between 0 and 1. A value greater than 0.005 indicates a minor stall and a value greater than 0.5 indicates a large-scale stall. </summary> <remarks> Requires <a href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>. </remarks> </doc>
    pub fn get_stall_fraction(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_StallFraction"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Total_air_temperature">static (ambient) temperature</a> of the atmosphere around the vessel, in Kelvin. </summary> </doc>
    pub fn get_static_air_temperature(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_StaticAirTemperature"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The static atmospheric pressure acting on the vessel, in Pascals. </summary> </doc>
    pub fn get_static_pressure(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_StaticPressure"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The static atmospheric pressure at mean sea level, in Pascals. </summary> </doc>
    pub fn get_static_pressure_at_msl(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_StaticPressureAtMSL"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The altitude above the surface of the body or sea level, whichever is closer, in meters. Measured from the center of mass of the vessel. </summary> </doc>
    pub fn get_surface_altitude(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_SurfaceAltitude"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> An estimate of the current terminal velocity of the vessel, in meters per second. This is the speed at which the drag forces cancel out the force of gravity. </summary> </doc>
    pub fn get_terminal_velocity(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_TerminalVelocity"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The thrust specific fuel consumption for the jet engines on the vessel. This is a measure of the efficiency of the engines, with a lower value indicating a more efficient vessel. This value is the number of Newtons of fuel that are burned, per hour, to produce one newton of thrust. </summary> <remarks> Requires <a href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>. </remarks> </doc>
    pub fn get_thrust_specific_fuel_consumption(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_ThrustSpecificFuelConsumption"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Total_air_temperature">total air temperature</a> of the atmosphere around the vessel, in Kelvin. This includes the <see cref="M:SpaceCenter.Flight.StaticAirTemperature" /> and the vessel's kinetic energy. </summary> </doc>
    pub fn get_total_air_temperature(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_TotalAirTemperature"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/True_airspeed">true air speed</a> of the vessel, in meters per second. </summary> </doc>
    pub fn get_true_air_speed(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_TrueAirSpeed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The velocity of the vessel, in the reference frame <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> <returns>The velocity as a vector. The vector points in the direction of travel, and its magnitude is the speed of the vessel in meters per second.</returns> </doc>
    pub fn get_velocity(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_Velocity"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The vertical speed of the vessel in meters per second, in the reference frame <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> </doc>
    pub fn get_vertical_speed(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Flight_get_VerticalSpeed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl Force {
    /// <doc> <summary> Remove the force. </summary> </doc>
    pub fn remove(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Force_Remove"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The force vector, in Newtons. </summary> <returns>A vector pointing in the direction that the force acts, with its magnitude equal to the strength of the force in Newtons.</returns> </doc>
    pub fn get_force_vector(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Force_get_ForceVector"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part that this force is applied to. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Force_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position at which the force acts, in reference frame <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> <returns>The position as a vector.</returns> </doc>
    pub fn get_position(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Force_get_Position"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The reference frame of the force vector and position. </summary> </doc>
    pub fn get_reference_frame(&self, ) -> CallHandle<ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Force_get_ReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The force vector, in Newtons. </summary> <returns>A vector pointing in the direction that the force acts, with its magnitude equal to the strength of the force in Newtons.</returns> </doc>
    pub fn set_force_vector(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Force_set_ForceVector"));

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

    /// <doc> <summary> The position at which the force acts, in reference frame <see cref="T:SpaceCenter.ReferenceFrame" />. </summary> <returns>The position as a vector.</returns> </doc>
    pub fn set_position(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Force_set_Position"));

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

    /// <doc> <summary> The reference frame of the force vector and position. </summary> </doc>
    pub fn set_reference_frame(&self, p_value: &ReferenceFrame) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Force_set_ReferenceFrame"));

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
impl Intake {
    /// <doc> <summary> The area of the intake's opening, in square meters. </summary> </doc>
    pub fn get_area(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Intake_get_Area"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rate of flow into the intake, in units of resource per second. </summary> </doc>
    pub fn get_flow(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Intake_get_Flow"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the intake is open. </summary> </doc>
    pub fn get_open(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Intake_get_Open"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this intake. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Intake_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Speed of the flow into the intake, in <math>m/s</math>. </summary> </doc>
    pub fn get_speed(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Intake_get_Speed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the intake is open. </summary> </doc>
    pub fn set_open(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Intake_set_Open"));

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
impl LaunchClamp {
    /// <doc> <summary> Releases the docking clamp. Has no effect if the clamp has already been released. </summary> </doc>
    pub fn release(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("LaunchClamp_Release"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this launch clamp. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("LaunchClamp_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl Leg {
    /// <doc> <summary> Whether the leg is deployable. </summary> </doc>
    pub fn get_deployable(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Leg_get_Deployable"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the landing leg is deployed. </summary> <remarks> Fixed landing legs are always deployed. Returns an error if you try to deploy fixed landing gear. </remarks> </doc>
    pub fn get_deployed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Leg_get_Deployed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns whether the leg is touching the ground. </summary> </doc>
    pub fn get_is_grounded(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Leg_get_IsGrounded"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this landing leg. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Leg_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current state of the landing leg. </summary> </doc>
    pub fn get_state(&self, ) -> CallHandle<LegState> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Leg_get_State"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the landing leg is deployed. </summary> <remarks> Fixed landing legs are always deployed. Returns an error if you try to deploy fixed landing gear. </remarks> </doc>
    pub fn set_deployed(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Leg_set_Deployed"));

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
impl Light {
    /// <doc> <summary> Whether the light is switched on. </summary> </doc>
    pub fn get_active(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Light_get_Active"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The color of the light, as an RGB triple. </summary> </doc>
    pub fn get_color(&self, ) -> CallHandle<(f32, f32, f32)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Light_get_Color"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this light. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Light_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current power usage, in units of charge per second. </summary> </doc>
    pub fn get_power_usage(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Light_get_PowerUsage"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the light is switched on. </summary> </doc>
    pub fn set_active(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Light_set_Active"));

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

    /// <doc> <summary> The color of the light, as an RGB triple. </summary> </doc>
    pub fn set_color(&self, p_value: (f32, f32, f32)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Light_set_Color"));

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
impl Module {
    /// <doc> <summary> Returns the value of a field. </summary> <param name="name">Name of the field.</param> </doc>
    pub fn get_field(&self, p_name: String) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Module_GetField"));

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

    /// <doc> <summary><c>true</c> if the part has an action with the given name. </summary> <param name="name"></param> </doc>
    pub fn has_action(&self, p_name: String) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Module_HasAction"));

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

    /// <doc> <summary><c>true</c> if the module has an event with the given name. </summary> <param name="name"></param> </doc>
    pub fn has_event(&self, p_name: String) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Module_HasEvent"));

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

    /// <doc> <summary> Returns <c>true</c> if the module has a field with the given name. </summary> <param name="name">Name of the field.</param> </doc>
    pub fn has_field(&self, p_name: String) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Module_HasField"));

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

    /// <doc> <summary> Set the value of a field to its original value. </summary> <param name="name">Name of the field.</param> </doc>
    pub fn reset_field(&self, p_name: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Module_ResetField"));

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

    /// <doc> <summary> Set the value of an action with the given name. </summary> <param name="name"></param> <param name="value"></param> </doc>
    pub fn set_action(&self, p_name: String, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Module_SetAction"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_name.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_value.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Set the value of a field to the given floating point number. </summary> <param name="name">Name of the field.</param> <param name="value">Value to set.</param> </doc>
    pub fn set_field_float(&self, p_name: String, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Module_SetFieldFloat"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_name.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_value.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Set the value of a field to the given integer number. </summary> <param name="name">Name of the field.</param> <param name="value">Value to set.</param> </doc>
    pub fn set_field_int(&self, p_name: String, p_value: i32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Module_SetFieldInt"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_name.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_value.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Set the value of a field to the given string. </summary> <param name="name">Name of the field.</param> <param name="value">Value to set.</param> </doc>
    pub fn set_field_string(&self, p_name: String, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Module_SetFieldString"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_name.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_value.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Trigger the named event. Equivalent to clicking the button in the right-click menu of the part. </summary> <param name="name"></param> </doc>
    pub fn trigger_event(&self, p_name: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Module_TriggerEvent"));

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

    /// <doc> <summary> A list of all the names of the modules actions. These are the parts actions that can be assigned to action groups in the in-game editor. </summary> </doc>
    pub fn get_actions(&self, ) -> CallHandle<Vec<String>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Module_get_Actions"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of the names of all of the modules events. Events are the clickable buttons visible in the right-click menu of the part. </summary> </doc>
    pub fn get_events(&self, ) -> CallHandle<Vec<String>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Module_get_Events"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The modules field names and their associated values, as a dictionary. These are the values visible in the right-click menu of the part. </summary> </doc>
    pub fn get_fields(&self, ) -> CallHandle<HashMap<String, String>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Module_get_Fields"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Name of the PartModule. For example, "ModuleEngines". </summary> </doc>
    pub fn get_name(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Module_get_Name"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part that contains this module. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Module_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl Node {
    /// <doc> <summary> Returns the burn vector for the maneuver node. </summary> <param name="referenceFrame">The reference frame that the returned vector is in. Defaults to <see cref="M:SpaceCenter.Vessel.OrbitalReferenceFrame" />.</param> <returns>A vector whose direction is the direction of the maneuver node burn, and magnitude is the delta-v of the burn in meters per second. </returns> <remarks> Does not change when executing the maneuver node. See <see cref="M:SpaceCenter.Node.RemainingBurnVector" />. </remarks> </doc>
    pub fn burn_vector(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_BurnVector"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The direction of the maneuver nodes burn. </summary> <returns>The direction as a unit vector.</returns> <param name="referenceFrame">The reference frame that the returned direction is in.</param> </doc>
    pub fn direction(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_Direction"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position vector of the maneuver node in the given reference frame. </summary> <returns>The position as a vector.</returns> <param name="referenceFrame">The reference frame that the returned position vector is in.</param> </doc>
    pub fn position(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_Position"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns the remaining burn vector for the maneuver node. </summary> <param name="referenceFrame">The reference frame that the returned vector is in. Defaults to <see cref="M:SpaceCenter.Vessel.OrbitalReferenceFrame" />.</param> <returns>A vector whose direction is the direction of the maneuver node burn, and magnitude is the delta-v of the burn in meters per second. </returns> <remarks> Changes as the maneuver node is executed. See <see cref="M:SpaceCenter.Node.BurnVector" />. </remarks> </doc>
    pub fn remaining_burn_vector(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_RemainingBurnVector"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Removes the maneuver node. </summary> </doc>
    pub fn remove(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_Remove"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The delta-v of the maneuver node, in meters per second. </summary> <remarks> Does not change when executing the maneuver node. See <see cref="M:SpaceCenter.Node.RemainingDeltaV" />. </remarks> </doc>
    pub fn get_delta_v(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_get_DeltaV"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The magnitude of the maneuver nodes delta-v in the normal direction, in meters per second. </summary> </doc>
    pub fn get_normal(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_get_Normal"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The orbit that results from executing the maneuver node. </summary> </doc>
    pub fn get_orbit(&self, ) -> CallHandle<Orbit> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_get_Orbit"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The reference frame that is fixed relative to the maneuver node, and orientated with the orbital prograde/normal/radial directions of the original orbit at the maneuver node's position. <list type="bullet"><item><description>The origin is at the position of the maneuver node.</description></item><item><description>The x-axis points in the orbital anti-radial direction of the original orbit, at the position of the maneuver node.</description></item><item><description>The y-axis points in the orbital prograde direction of the original orbit, at the position of the maneuver node.</description></item><item><description>The z-axis points in the orbital normal direction of the original orbit, at the position of the maneuver node.</description></item></list></summary> </doc>
    pub fn get_orbital_reference_frame(&self, ) -> CallHandle<ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_get_OrbitalReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The magnitude of the maneuver nodes delta-v in the prograde direction, in meters per second. </summary> </doc>
    pub fn get_prograde(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_get_Prograde"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The magnitude of the maneuver nodes delta-v in the radial direction, in meters per second. </summary> </doc>
    pub fn get_radial(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_get_Radial"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The reference frame that is fixed relative to the maneuver node's burn. <list type="bullet"><item><description>The origin is at the position of the maneuver node.</description></item><item><description>The y-axis points in the direction of the burn.</description></item><item><description>The x-axis and z-axis point in arbitrary but fixed directions.</description></item></list></summary> </doc>
    pub fn get_reference_frame(&self, ) -> CallHandle<ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_get_ReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Gets the remaining delta-v of the maneuver node, in meters per second. Changes as the node is executed. This is equivalent to the delta-v reported in-game. </summary> </doc>
    pub fn get_remaining_delta_v(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_get_RemainingDeltaV"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The time until the maneuver node will be encountered, in seconds. </summary> </doc>
    pub fn get_time_to(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_get_TimeTo"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The universal time at which the maneuver will occur, in seconds. </summary> </doc>
    pub fn get_ut(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_get_UT"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The delta-v of the maneuver node, in meters per second. </summary> <remarks> Does not change when executing the maneuver node. See <see cref="M:SpaceCenter.Node.RemainingDeltaV" />. </remarks> </doc>
    pub fn set_delta_v(&self, p_value: f64) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_set_DeltaV"));

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

    /// <doc> <summary> The magnitude of the maneuver nodes delta-v in the normal direction, in meters per second. </summary> </doc>
    pub fn set_normal(&self, p_value: f64) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_set_Normal"));

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

    /// <doc> <summary> The magnitude of the maneuver nodes delta-v in the prograde direction, in meters per second. </summary> </doc>
    pub fn set_prograde(&self, p_value: f64) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_set_Prograde"));

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

    /// <doc> <summary> The magnitude of the maneuver nodes delta-v in the radial direction, in meters per second. </summary> </doc>
    pub fn set_radial(&self, p_value: f64) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_set_Radial"));

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

    /// <doc> <summary> The universal time at which the maneuver will occur, in seconds. </summary> </doc>
    pub fn set_ut(&self, p_value: f64) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Node_set_UT"));

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
impl Orbit {
    /// <doc> <summary> Estimates and returns the distance at closest approach to a target orbit, in meters. </summary> <param name="target">Target orbit.</param> </doc>
    pub fn distance_at_closest_approach(&self, p_target: &Orbit) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_DistanceAtClosestApproach"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_target.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The eccentric anomaly at the given universal time. </summary> <param name="ut">The universal time, in seconds.</param> </doc>
    pub fn eccentric_anomaly_at_ut(&self, p_ut: f64) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_EccentricAnomalyAtUT"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_ut.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns the times at closest approach and corresponding distances, to a target orbit. </summary> <returns> A list of two lists. The first is a list of times at closest approach, as universal times in seconds. The second is a list of corresponding distances at closest approach, in meters. </returns> <param name="target">Target orbit.</param> <param name="orbits">The number of future orbits to search.</param> </doc>
    pub fn list_closest_approaches(&self, p_target: &Orbit, p_orbits: i32) -> CallHandle<Vec<Vec<f64>>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_ListClosestApproaches"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_target.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_orbits.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The mean anomaly at the given time. </summary> <param name="ut">The universal time in seconds.</param> </doc>
    pub fn mean_anomaly_at_ut(&self, p_ut: f64) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_MeanAnomalyAtUT"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_ut.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The orbital speed at the given time, in meters per second. </summary> <param name="time">Time from now, in seconds.</param> </doc>
    pub fn orbital_speed_at(&self, p_time: f64) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_OrbitalSpeedAt"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_time.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position at a given time, in the specified reference frame. </summary> <returns>The position as a vector.</returns> <param name="ut">The universal time to measure the position at.</param> <param name="referenceFrame">The reference frame that the returned position vector is in.</param> </doc>
    pub fn position_at(&self, p_ut: f64, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_PositionAt"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_ut.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The orbital radius at the given time, in meters. </summary> <param name="ut">The universal time to measure the radius at.</param> </doc>
    pub fn radius_at(&self, p_ut: f64) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_RadiusAt"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_ut.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The orbital radius at the point in the orbit given by the true anomaly. </summary> <param name="trueAnomaly">The true anomaly.</param> </doc>
    pub fn radius_at_true_anomaly(&self, p_true_anomaly: f64) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_RadiusAtTrueAnomaly"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_true_anomaly.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Relative inclination of this orbit and the target orbit, in radians. </summary> <param name="target">Target orbit.</param> </doc>
    pub fn relative_inclination(&self, p_target: &Orbit) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_RelativeInclination"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_target.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Estimates and returns the time at closest approach to a target orbit. </summary> <returns>The universal time at closest approach, in seconds.</returns> <param name="target">Target orbit.</param> </doc>
    pub fn time_of_closest_approach(&self, p_target: &Orbit) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_TimeOfClosestApproach"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_target.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The true anomaly of the ascending node with the given target orbit. </summary> <param name="target">Target orbit.</param> </doc>
    pub fn true_anomaly_at_an(&self, p_target: &Orbit) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_TrueAnomalyAtAN"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_target.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The true anomaly of the descending node with the given target orbit. </summary> <param name="target">Target orbit.</param> </doc>
    pub fn true_anomaly_at_dn(&self, p_target: &Orbit) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_TrueAnomalyAtDN"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_target.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The true anomaly at the given orbital radius. </summary> <param name="radius">The orbital radius in meters.</param> </doc>
    pub fn true_anomaly_at_radius(&self, p_radius: f64) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_TrueAnomalyAtRadius"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_radius.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The true anomaly at the given time. </summary> <param name="ut">The universal time in seconds.</param> </doc>
    pub fn true_anomaly_at_ut(&self, p_ut: f64) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_TrueAnomalyAtUT"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_ut.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The universal time, in seconds, corresponding to the given true anomaly. </summary> <param name="trueAnomaly">True anomaly.</param> </doc>
    pub fn ut_at_true_anomaly(&self, p_true_anomaly: f64) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_UTAtTrueAnomaly"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_true_anomaly.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Gets the apoapsis of the orbit, in meters, from the center of mass of the body being orbited. </summary> <remarks> For the apoapsis altitude reported on the in-game map view, use <see cref="M:SpaceCenter.Orbit.ApoapsisAltitude" />. </remarks> </doc>
    pub fn get_apoapsis(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_Apoapsis"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The apoapsis of the orbit, in meters, above the sea level of the body being orbited. </summary> <remarks> This is equal to <see cref="M:SpaceCenter.Orbit.Apoapsis" /> minus the equatorial radius of the body. </remarks> </doc>
    pub fn get_apoapsis_altitude(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_ApoapsisAltitude"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Argument_of_periapsis">argument of periapsis</a>, in radians. </summary> </doc>
    pub fn get_argument_of_periapsis(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_ArgumentOfPeriapsis"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The celestial body (e.g. planet or moon) around which the object is orbiting. </summary> </doc>
    pub fn get_body(&self, ) -> CallHandle<CelestialBody> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_Body"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Eccentric_anomaly">eccentric anomaly</a>. </summary> </doc>
    pub fn get_eccentric_anomaly(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_EccentricAnomaly"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Orbital_eccentricity">eccentricity</a> of the orbit. </summary> </doc>
    pub fn get_eccentricity(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_Eccentricity"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The time since the epoch (the point at which the <a href="https://en.wikipedia.org/wiki/Mean_anomaly">mean anomaly at epoch</a> was measured, in seconds. </summary> </doc>
    pub fn get_epoch(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_Epoch"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Orbital_inclination">inclination</a> of the orbit, in radians. </summary> </doc>
    pub fn get_inclination(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_Inclination"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Longitude_of_the_ascending_node">longitude of the ascending node</a>, in radians. </summary> </doc>
    pub fn get_longitude_of_ascending_node(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_LongitudeOfAscendingNode"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Mean_anomaly">mean anomaly</a>. </summary> </doc>
    pub fn get_mean_anomaly(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_MeanAnomaly"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/Mean_anomaly">mean anomaly at epoch</a>. </summary> </doc>
    pub fn get_mean_anomaly_at_epoch(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_MeanAnomalyAtEpoch"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> If the object is going to change sphere of influence in the future, returns the new orbit after the change. Otherwise returns <c>null</c>. </summary> </doc>
    pub fn get_next_orbit(&self, ) -> CallHandle<Orbit> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_NextOrbit"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current orbital speed in meters per second. </summary> </doc>
    pub fn get_orbital_speed(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_OrbitalSpeed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The periapsis of the orbit, in meters, from the center of mass of the body being orbited. </summary> <remarks> For the periapsis altitude reported on the in-game map view, use <see cref="M:SpaceCenter.Orbit.PeriapsisAltitude" />. </remarks> </doc>
    pub fn get_periapsis(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_Periapsis"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The periapsis of the orbit, in meters, above the sea level of the body being orbited. </summary> <remarks> This is equal to <see cref="M:SpaceCenter.Orbit.Periapsis" /> minus the equatorial radius of the body. </remarks> </doc>
    pub fn get_periapsis_altitude(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_PeriapsisAltitude"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The orbital period, in seconds. </summary> </doc>
    pub fn get_period(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_Period"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current radius of the orbit, in meters. This is the distance between the center of mass of the object in orbit, and the center of mass of the body around which it is orbiting. </summary> <remarks> This value will change over time if the orbit is elliptical. </remarks> </doc>
    pub fn get_radius(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_Radius"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The semi-major axis of the orbit, in meters. </summary> </doc>
    pub fn get_semi_major_axis(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_SemiMajorAxis"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The semi-minor axis of the orbit, in meters. </summary> </doc>
    pub fn get_semi_minor_axis(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_SemiMinorAxis"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current orbital speed of the object in meters per second. </summary> <remarks> This value will change over time if the orbit is elliptical. </remarks> </doc>
    pub fn get_speed(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_Speed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The time until the object reaches apoapsis, in seconds. </summary> </doc>
    pub fn get_time_to_apoapsis(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_TimeToApoapsis"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The time until the object reaches periapsis, in seconds. </summary> </doc>
    pub fn get_time_to_periapsis(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_TimeToPeriapsis"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The time until the object changes sphere of influence, in seconds. Returns <c>NaN</c> if the object is not going to change sphere of influence. </summary> </doc>
    pub fn get_time_to_soi_change(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_TimeToSOIChange"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <a href="https://en.wikipedia.org/wiki/True_anomaly">true anomaly</a>. </summary> </doc>
    pub fn get_true_anomaly(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Orbit_get_TrueAnomaly"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl Parachute {
    /// <doc> <summary> Deploys the parachute. This has no effect if the parachute has already been armed or deployed. Only applicable to RealChutes parachutes. </summary> </doc>
    pub fn arm(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parachute_Arm"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Deploys the parachute. This has no effect if the parachute has already been deployed. </summary> </doc>
    pub fn deploy(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parachute_Deploy"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the parachute has been armed or deployed. Only applicable to RealChutes parachutes. </summary> </doc>
    pub fn get_armed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parachute_get_Armed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The altitude at which the parachute will full deploy, in meters. Only applicable to stock parachutes. </summary> </doc>
    pub fn get_deploy_altitude(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parachute_get_DeployAltitude"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The minimum pressure at which the parachute will semi-deploy, in atmospheres. Only applicable to stock parachutes. </summary> </doc>
    pub fn get_deploy_min_pressure(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parachute_get_DeployMinPressure"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the parachute has been deployed. </summary> </doc>
    pub fn get_deployed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parachute_get_Deployed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this parachute. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parachute_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current state of the parachute. </summary> </doc>
    pub fn get_state(&self, ) -> CallHandle<ParachuteState> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parachute_get_State"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The altitude at which the parachute will full deploy, in meters. Only applicable to stock parachutes. </summary> </doc>
    pub fn set_deploy_altitude(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parachute_set_DeployAltitude"));

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

    /// <doc> <summary> The minimum pressure at which the parachute will semi-deploy, in atmospheres. Only applicable to stock parachutes. </summary> </doc>
    pub fn set_deploy_min_pressure(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parachute_set_DeployMinPressure"));

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
impl Part {
    /// <doc> <summary> Exert a constant force on the part, acting at the given position. </summary> <returns>An object that can be used to remove or modify the force.</returns> <param name="force">A vector pointing in the direction that the force acts, with its magnitude equal to the strength of the force in Newtons.</param> <param name="position">The position at which the force acts, as a vector.</param> <param name="referenceFrame">The reference frame that the force and position are in.</param> </doc>
    pub fn add_force(&self, p_force: (f64, f64, f64), p_position: (f64, f64, f64), p_reference_frame: &ReferenceFrame) -> CallHandle<Force> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_AddForce"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_force.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_position.encode_to_bytes().unwrap());
        arguments.push(arg2);

        let mut arg3 = krpc::Argument::new();
        arg3.set_position(3);
        arg3.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg3);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The axis-aligned bounding box of the part in the given reference frame. </summary> <returns>The positions of the minimum and maximum vertices of the box, as position vectors.</returns> <param name="referenceFrame">The reference frame that the returned position vectors are in.</param> <remarks> This is computed from the collision mesh of the part. If the part is not collidable, the box has zero volume and is centered on the <see cref="M:SpaceCenter.Part.Position" /> of the part. </remarks> </doc>
    pub fn bounding_box(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<((f64, f64, f64), (f64, f64, f64))> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_BoundingBox"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position of the parts center of mass in the given reference frame. If the part is physicsless, this is equivalent to <see cref="M:SpaceCenter.Part.Position" />. </summary> <returns>The position as a vector.</returns> <param name="referenceFrame">The reference frame that the returned position vector is in.</param> </doc>
    pub fn center_of_mass(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_CenterOfMass"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The direction the part points in, in the given reference frame. </summary> <returns>The direction as a unit vector.</returns> <param name="referenceFrame">The reference frame that the returned direction is in.</param> </doc>
    pub fn direction(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_Direction"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Exert an instantaneous force on the part, acting at the given position. </summary> <param name="force">A vector pointing in the direction that the force acts, with its magnitude equal to the strength of the force in Newtons.</param> <param name="position">The position at which the force acts, as a vector.</param> <param name="referenceFrame">The reference frame that the force and position are in.</param> <remarks>The force is applied instantaneously in a single physics update.</remarks> </doc>
    pub fn instantaneous_force(&self, p_force: (f64, f64, f64), p_position: (f64, f64, f64), p_reference_frame: &ReferenceFrame) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_InstantaneousForce"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_force.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_position.encode_to_bytes().unwrap());
        arguments.push(arg2);

        let mut arg3 = krpc::Argument::new();
        arg3.set_position(3);
        arg3.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg3);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position of the part in the given reference frame. </summary> <returns>The position as a vector.</returns> <param name="referenceFrame">The reference frame that the returned position vector is in.</param> <remarks> This is a fixed position in the part, defined by the parts model. It s not necessarily the same as the parts center of mass. Use <see cref="M:SpaceCenter.Part.CenterOfMass" /> to get the parts center of mass. </remarks> </doc>
    pub fn position(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_Position"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rotation of the part, in the given reference frame. </summary> <returns>The rotation as a quaternion of the form <math>(x, y, z, w)</math>.</returns> <param name="referenceFrame">The reference frame that the returned rotation is in.</param> </doc>
    pub fn rotation(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_Rotation"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The linear velocity of the part in the given reference frame. </summary> <returns>The velocity as a vector. The vector points in the direction of travel, and its magnitude is the speed of the body in meters per second.</returns> <param name="referenceFrame">The reference frame that the returned velocity vector is in.</param> </doc>
    pub fn velocity(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_Velocity"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.Antenna" /> if the part is an antenna, otherwise <c>null</c>. </summary> </doc>
    pub fn get_antenna(&self, ) -> CallHandle<Antenna> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Antenna"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the part is axially attached to its parent, i.e. on the top or bottom of its parent. If the part has no parent, returns <c>false</c>. </summary> </doc>
    pub fn get_axially_attached(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_AxiallyAttached"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.CargoBay" /> if the part is a cargo bay, otherwise <c>null</c>. </summary> </doc>
    pub fn get_cargo_bay(&self, ) -> CallHandle<CargoBay> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_CargoBay"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The reference frame that is fixed relative to this part, and centered on its center of mass. <list type="bullet"><item><description>The origin is at the center of mass of the part, as returned by <see cref="M:SpaceCenter.Part.CenterOfMass" />.</description></item><item><description>The axes rotate with the part.</description></item><item><description>The x, y and z axis directions depend on the design of the part. </description></item></list></summary> <remarks> For docking port parts, this reference frame is not necessarily equivalent to the reference frame for the docking port, returned by <see cref="M:SpaceCenter.DockingPort.ReferenceFrame" />. </remarks> </doc>
    pub fn get_center_of_mass_reference_frame(&self, ) -> CallHandle<ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_CenterOfMassReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The parts children. Returns an empty list if the part has no children. This, in combination with <see cref="M:SpaceCenter.Part.Parent" />, can be used to traverse the vessels parts tree. </summary> </doc>
    pub fn get_children(&self, ) -> CallHandle<Vec<Part>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Children"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.ControlSurface" /> if the part is an aerodynamic control surface, otherwise <c>null</c>. </summary> </doc>
    pub fn get_control_surface(&self, ) -> CallHandle<ControlSurface> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_ControlSurface"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The cost of the part, in units of funds. </summary> </doc>
    pub fn get_cost(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Cost"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether this part is crossfeed capable. </summary> </doc>
    pub fn get_crossfeed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Crossfeed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The stage in which this part will be decoupled. Returns -1 if the part is never decoupled from the vessel. </summary> </doc>
    pub fn get_decouple_stage(&self, ) -> CallHandle<i32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_DecoupleStage"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.Decoupler" /> if the part is a decoupler, otherwise <c>null</c>. </summary> </doc>
    pub fn get_decoupler(&self, ) -> CallHandle<Decoupler> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Decoupler"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.DockingPort" /> if the part is a docking port, otherwise <c>null</c>. </summary> </doc>
    pub fn get_docking_port(&self, ) -> CallHandle<DockingPort> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_DockingPort"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The mass of the part, not including any resources it contains, in kilograms. Returns zero if the part is massless. </summary> </doc>
    pub fn get_dry_mass(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_DryMass"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The dynamic pressure acting on the part, in Pascals. </summary> </doc>
    pub fn get_dynamic_pressure(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_DynamicPressure"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> An <see cref="T:SpaceCenter.Engine" /> if the part is an engine, otherwise <c>null</c>. </summary> </doc>
    pub fn get_engine(&self, ) -> CallHandle<Engine> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Engine"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> An <see cref="T:SpaceCenter.Experiment" /> if the part is a science experiment, otherwise <c>null</c>. </summary> </doc>
    pub fn get_experiment(&self, ) -> CallHandle<Experiment> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Experiment"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.Fairing" /> if the part is a fairing, otherwise <c>null</c>. </summary> </doc>
    pub fn get_fairing(&self, ) -> CallHandle<Fairing> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Fairing"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The parts that are connected to this part via fuel lines, where the direction of the fuel line is into this part. </summary> </doc>
    pub fn get_fuel_lines_from(&self, ) -> CallHandle<Vec<Part>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_FuelLinesFrom"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The parts that are connected to this part via fuel lines, where the direction of the fuel line is out of this part. </summary> </doc>
    pub fn get_fuel_lines_to(&self, ) -> CallHandle<Vec<Part>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_FuelLinesTo"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The color used to highlight the part, as an RGB triple. </summary> </doc>
    pub fn get_highlight_color(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_HighlightColor"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the part is highlighted. </summary> </doc>
    pub fn get_highlighted(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Highlighted"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The impact tolerance of the part, in meters per second. </summary> </doc>
    pub fn get_impact_tolerance(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_ImpactTolerance"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The inertia tensor of the part in the parts reference frame (<see cref="T:SpaceCenter.ReferenceFrame" />). Returns the 3x3 matrix as a list of elements, in row-major order. </summary> </doc>
    pub fn get_inertia_tensor(&self, ) -> CallHandle<Vec<f64>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_InertiaTensor"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> An <see cref="T:SpaceCenter.Intake" /> if the part is an intake, otherwise <c>null</c>. </summary> <remarks> This includes any part that generates thrust. This covers many different types of engine, including liquid fuel rockets, solid rocket boosters and jet engines. For RCS thrusters see <see cref="T:SpaceCenter.RCS" />. </remarks> </doc>
    pub fn get_intake(&self, ) -> CallHandle<Intake> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Intake"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether this part is a fuel line. </summary> </doc>
    pub fn get_is_fuel_line(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_IsFuelLine"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.LaunchClamp" /> if the part is a launch clamp, otherwise <c>null</c>. </summary> </doc>
    pub fn get_launch_clamp(&self, ) -> CallHandle<LaunchClamp> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_LaunchClamp"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.Leg" /> if the part is a landing leg, otherwise <c>null</c>. </summary> </doc>
    pub fn get_leg(&self, ) -> CallHandle<Leg> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Leg"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.Light" /> if the part is a light, otherwise <c>null</c>. </summary> </doc>
    pub fn get_light(&self, ) -> CallHandle<Light> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Light"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current mass of the part, including resources it contains, in kilograms. Returns zero if the part is massless. </summary> </doc>
    pub fn get_mass(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Mass"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the part is <a href="https://wiki.kerbalspaceprogram.com/wiki/Massless_part">massless</a>. </summary> </doc>
    pub fn get_massless(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Massless"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Maximum temperature that the skin of the part can survive, in Kelvin. </summary> </doc>
    pub fn get_max_skin_temperature(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_MaxSkinTemperature"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Maximum temperature that the part can survive, in Kelvin. </summary> </doc>
    pub fn get_max_temperature(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_MaxTemperature"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The modules for this part. </summary> </doc>
    pub fn get_modules(&self, ) -> CallHandle<Vec<Module>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Modules"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The moment of inertia of the part in <math>kg.m^2</math> around its center of mass in the parts reference frame (<see cref="T:SpaceCenter.ReferenceFrame" />). </summary> </doc>
    pub fn get_moment_of_inertia(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_MomentOfInertia"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Internal name of the part, as used in <a href="https://wiki.kerbalspaceprogram.com/wiki/CFG_File_Documentation">part cfg files</a>. For example "Mark1-2Pod". </summary> </doc>
    pub fn get_name(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Name"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.Parachute" /> if the part is a parachute, otherwise <c>null</c>. </summary> </doc>
    pub fn get_parachute(&self, ) -> CallHandle<Parachute> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Parachute"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The parts parent. Returns <c>null</c> if the part does not have a parent. This, in combination with <see cref="M:SpaceCenter.Part.Children" />, can be used to traverse the vessels parts tree. </summary> </doc>
    pub fn get_parent(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Parent"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.RCS" /> if the part is an RCS block/thruster, otherwise <c>null</c>. </summary> </doc>
    pub fn get_rcs(&self, ) -> CallHandle<RCS> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_RCS"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the part is radially attached to its parent, i.e. on the side of its parent. If the part has no parent, returns <c>false</c>. </summary> </doc>
    pub fn get_radially_attached(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_RadiallyAttached"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.Radiator" /> if the part is a radiator, otherwise <c>null</c>. </summary> </doc>
    pub fn get_radiator(&self, ) -> CallHandle<Radiator> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Radiator"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.ReactionWheel" /> if the part is a reaction wheel, otherwise <c>null</c>. </summary> </doc>
    pub fn get_reaction_wheel(&self, ) -> CallHandle<ReactionWheel> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_ReactionWheel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The reference frame that is fixed relative to this part, and centered on a fixed position within the part, defined by the parts model. <list type="bullet"><item><description>The origin is at the position of the part, as returned by <see cref="M:SpaceCenter.Part.Position" />.</description></item><item><description>The axes rotate with the part.</description></item><item><description>The x, y and z axis directions depend on the design of the part. </description></item></list></summary> <remarks> For docking port parts, this reference frame is not necessarily equivalent to the reference frame for the docking port, returned by <see cref="M:SpaceCenter.DockingPort.ReferenceFrame" />. </remarks> </doc>
    pub fn get_reference_frame(&self, ) -> CallHandle<ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_ReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.ResourceConverter" /> if the part is a resource converter, otherwise <c>null</c>. </summary> </doc>
    pub fn get_resource_converter(&self, ) -> CallHandle<ResourceConverter> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_ResourceConverter"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.ResourceHarvester" /> if the part is a resource harvester, otherwise <c>null</c>. </summary> </doc>
    pub fn get_resource_harvester(&self, ) -> CallHandle<ResourceHarvester> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_ResourceHarvester"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.Resources" /> object for the part. </summary> </doc>
    pub fn get_resources(&self, ) -> CallHandle<Resources> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Resources"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.Sensor" /> if the part is a sensor, otherwise <c>null</c>. </summary> </doc>
    pub fn get_sensor(&self, ) -> CallHandle<Sensor> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Sensor"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the part is shielded from the exterior of the vessel, for example by a fairing. </summary> </doc>
    pub fn get_shielded(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Shielded"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Temperature of the skin of the part, in Kelvin. </summary> </doc>
    pub fn get_skin_temperature(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_SkinTemperature"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.SolarPanel" /> if the part is a solar panel, otherwise <c>null</c>. </summary> </doc>
    pub fn get_solar_panel(&self, ) -> CallHandle<SolarPanel> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_SolarPanel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The stage in which this part will be activated. Returns -1 if the part is not activated by staging. </summary> </doc>
    pub fn get_stage(&self, ) -> CallHandle<i32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Stage"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The name tag for the part. Can be set to a custom string using the in-game user interface. </summary> <remarks> This string is shared with <a href="https://forum.kerbalspaceprogram.com/index.php?/topic/61827-/">kOS</a> if it is installed. </remarks> </doc>
    pub fn get_tag(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Tag"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Temperature of the part, in Kelvin. </summary> </doc>
    pub fn get_temperature(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Temperature"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rate at which heat energy is conducting into or out of the part via contact with other parts. Measured in energy per unit time, or power, in Watts. A positive value means the part is gaining heat energy, and negative means it is losing heat energy. </summary> </doc>
    pub fn get_thermal_conduction_flux(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_ThermalConductionFlux"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rate at which heat energy is convecting into or out of the part from the surrounding atmosphere. Measured in energy per unit time, or power, in Watts. A positive value means the part is gaining heat energy, and negative means it is losing heat energy. </summary> </doc>
    pub fn get_thermal_convection_flux(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_ThermalConvectionFlux"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rate at which heat energy is begin generated by the part. For example, some engines generate heat by combusting fuel. Measured in energy per unit time, or power, in Watts. A positive value means the part is gaining heat energy, and negative means it is losing heat energy. </summary> </doc>
    pub fn get_thermal_internal_flux(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_ThermalInternalFlux"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A measure of how much energy it takes to increase the internal temperature of the part, in Joules per Kelvin. </summary> </doc>
    pub fn get_thermal_mass(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_ThermalMass"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rate at which heat energy is radiating into or out of the part from the surrounding environment. Measured in energy per unit time, or power, in Watts. A positive value means the part is gaining heat energy, and negative means it is losing heat energy. </summary> </doc>
    pub fn get_thermal_radiation_flux(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_ThermalRadiationFlux"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A measure of how much energy it takes to increase the temperature of the resources contained in the part, in Joules per Kelvin. </summary> </doc>
    pub fn get_thermal_resource_mass(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_ThermalResourceMass"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A measure of how much energy it takes to increase the skin temperature of the part, in Joules per Kelvin. </summary> </doc>
    pub fn get_thermal_skin_mass(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_ThermalSkinMass"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rate at which heat energy is transferring between the part's skin and its internals. Measured in energy per unit time, or power, in Watts. A positive value means the part's internals are gaining heat energy, and negative means its skin is gaining heat energy. </summary> </doc>
    pub fn get_thermal_skin_to_internal_flux(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_ThermalSkinToInternalFlux"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Title of the part, as shown when the part is right clicked in-game. For example "Mk1-2 Command Pod". </summary> </doc>
    pub fn get_title(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Title"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The vessel that contains this part. </summary> </doc>
    pub fn get_vessel(&self, ) -> CallHandle<Vessel> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Vessel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.Wheel" /> if the part is a wheel, otherwise <c>null</c>. </summary> </doc>
    pub fn get_wheel(&self, ) -> CallHandle<Wheel> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_get_Wheel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The color used to highlight the part, as an RGB triple. </summary> </doc>
    pub fn set_highlight_color(&self, p_value: (f64, f64, f64)) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_set_HighlightColor"));

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

    /// <doc> <summary> Whether the part is highlighted. </summary> </doc>
    pub fn set_highlighted(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_set_Highlighted"));

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

    /// <doc> <summary> The name tag for the part. Can be set to a custom string using the in-game user interface. </summary> <remarks> This string is shared with <a href="https://forum.kerbalspaceprogram.com/index.php?/topic/61827-/">kOS</a> if it is installed. </remarks> </doc>
    pub fn set_tag(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Part_set_Tag"));

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
impl Parts {
    /// <doc> <summary> A list of all parts that are decoupled in the given <paramref name="stage" />. </summary> <param name="stage"></param> </doc>
    pub fn in_decouple_stage(&self, p_stage: i32) -> CallHandle<Vec<Part>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_InDecoupleStage"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_stage.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all parts that are activated in the given <paramref name="stage" />. </summary> <param name="stage"></param> </doc>
    pub fn in_stage(&self, p_stage: i32) -> CallHandle<Vec<Part>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_InStage"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_stage.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of modules (combined across all parts in the vessel) whose <see cref="M:SpaceCenter.Module.Name" /> is <paramref name="moduleName" />. </summary> <param name="moduleName"></param> </doc>
    pub fn modules_with_name(&self, p_module_name: String) -> CallHandle<Vec<Module>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_ModulesWithName"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_module_name.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all parts that contain a <see cref="T:SpaceCenter.Module" /> whose <see cref="M:SpaceCenter.Module.Name" /> is <paramref name="moduleName" />. </summary> <param name="moduleName"></param> </doc>
    pub fn with_module(&self, p_module_name: String) -> CallHandle<Vec<Part>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_WithModule"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_module_name.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of parts whose <see cref="M:SpaceCenter.Part.Name" /> is <paramref name="name" />. </summary> <param name="name"></param> </doc>
    pub fn with_name(&self, p_name: String) -> CallHandle<Vec<Part>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_WithName"));

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

    /// <doc> <summary> A list of all parts whose <see cref="M:SpaceCenter.Part.Tag" /> is <paramref name="tag" />. </summary> <param name="tag"></param> </doc>
    pub fn with_tag(&self, p_tag: String) -> CallHandle<Vec<Part>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_WithTag"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_tag.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all parts whose <see cref="M:SpaceCenter.Part.Title" /> is <paramref name="title" />. </summary> <param name="title"></param> </doc>
    pub fn with_title(&self, p_title: String) -> CallHandle<Vec<Part>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_WithTitle"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_title.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all of the vessels parts. </summary> </doc>
    pub fn get_all(&self, ) -> CallHandle<Vec<Part>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_All"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all antennas in the vessel. </summary> </doc>
    pub fn get_antennas(&self, ) -> CallHandle<Vec<Antenna>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_Antennas"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all cargo bays in the vessel. </summary> </doc>
    pub fn get_cargo_bays(&self, ) -> CallHandle<Vec<CargoBay>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_CargoBays"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all control surfaces in the vessel. </summary> </doc>
    pub fn get_control_surfaces(&self, ) -> CallHandle<Vec<ControlSurface>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_ControlSurfaces"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part from which the vessel is controlled. </summary> </doc>
    pub fn get_controlling(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_Controlling"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all decouplers in the vessel. </summary> </doc>
    pub fn get_decouplers(&self, ) -> CallHandle<Vec<Decoupler>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_Decouplers"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all docking ports in the vessel. </summary> </doc>
    pub fn get_docking_ports(&self, ) -> CallHandle<Vec<DockingPort>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_DockingPorts"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all engines in the vessel. </summary> <remarks> This includes any part that generates thrust. This covers many different types of engine, including liquid fuel rockets, solid rocket boosters, jet engines and RCS thrusters. </remarks> </doc>
    pub fn get_engines(&self, ) -> CallHandle<Vec<Engine>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_Engines"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all science experiments in the vessel. </summary> </doc>
    pub fn get_experiments(&self, ) -> CallHandle<Vec<Experiment>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_Experiments"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all fairings in the vessel. </summary> </doc>
    pub fn get_fairings(&self, ) -> CallHandle<Vec<Fairing>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_Fairings"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all intakes in the vessel. </summary> </doc>
    pub fn get_intakes(&self, ) -> CallHandle<Vec<Intake>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_Intakes"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all launch clamps attached to the vessel. </summary> </doc>
    pub fn get_launch_clamps(&self, ) -> CallHandle<Vec<LaunchClamp>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_LaunchClamps"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all landing legs attached to the vessel. </summary> </doc>
    pub fn get_legs(&self, ) -> CallHandle<Vec<Leg>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_Legs"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all lights in the vessel. </summary> </doc>
    pub fn get_lights(&self, ) -> CallHandle<Vec<Light>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_Lights"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all parachutes in the vessel. </summary> </doc>
    pub fn get_parachutes(&self, ) -> CallHandle<Vec<Parachute>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_Parachutes"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all RCS blocks/thrusters in the vessel. </summary> </doc>
    pub fn get_rcs(&self, ) -> CallHandle<Vec<RCS>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_RCS"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all radiators in the vessel. </summary> </doc>
    pub fn get_radiators(&self, ) -> CallHandle<Vec<Radiator>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_Radiators"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all reaction wheels in the vessel. </summary> </doc>
    pub fn get_reaction_wheels(&self, ) -> CallHandle<Vec<ReactionWheel>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_ReactionWheels"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all resource converters in the vessel. </summary> </doc>
    pub fn get_resource_converters(&self, ) -> CallHandle<Vec<ResourceConverter>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_ResourceConverters"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all resource harvesters in the vessel. </summary> </doc>
    pub fn get_resource_harvesters(&self, ) -> CallHandle<Vec<ResourceHarvester>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_ResourceHarvesters"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The vessels root part. </summary> </doc>
    pub fn get_root(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_Root"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all sensors in the vessel. </summary> </doc>
    pub fn get_sensors(&self, ) -> CallHandle<Vec<Sensor>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_Sensors"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all solar panels in the vessel. </summary> </doc>
    pub fn get_solar_panels(&self, ) -> CallHandle<Vec<SolarPanel>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_SolarPanels"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all wheels in the vessel. </summary> </doc>
    pub fn get_wheels(&self, ) -> CallHandle<Vec<Wheel>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_get_Wheels"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part from which the vessel is controlled. </summary> </doc>
    pub fn set_controlling(&self, p_value: &Part) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Parts_set_Controlling"));

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
impl Propellant {
    /// <doc> <summary> The current amount of propellant. </summary> </doc>
    pub fn get_current_amount(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Propellant_get_CurrentAmount"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The required amount of propellant. </summary> </doc>
    pub fn get_current_requirement(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Propellant_get_CurrentRequirement"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> If this propellant has a stack gauge or not. </summary> </doc>
    pub fn get_draw_stack_gauge(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Propellant_get_DrawStackGauge"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> If this propellant should be ignored when calculating required mass flow given specific impulse. </summary> </doc>
    pub fn get_ignore_for_isp(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Propellant_get_IgnoreForIsp"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> If this propellant should be ignored for thrust curve calculations. </summary> </doc>
    pub fn get_ignore_for_thrust_curve(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Propellant_get_IgnoreForThrustCurve"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> If this propellant is deprived. </summary> </doc>
    pub fn get_is_deprived(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Propellant_get_IsDeprived"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The name of the propellant. </summary> </doc>
    pub fn get_name(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Propellant_get_Name"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The propellant ratio. </summary> </doc>
    pub fn get_ratio(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Propellant_get_Ratio"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The total amount of the underlying resource currently reachable given resource flow rules. </summary> </doc>
    pub fn get_total_resource_available(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Propellant_get_TotalResourceAvailable"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The total vehicle capacity for the underlying propellant resource, restricted by resource flow rules. </summary> </doc>
    pub fn get_total_resource_capacity(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Propellant_get_TotalResourceCapacity"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl RCS {
    /// <doc> <summary> Whether the RCS thrusters are active. An RCS thruster is inactive if the RCS action group is disabled (<see cref="M:SpaceCenter.Control.RCS" />), the RCS thruster itself is not enabled (<see cref="M:SpaceCenter.RCS.Enabled" />) or it is covered by a fairing (<see cref="M:SpaceCenter.Part.Shielded" />). </summary> </doc>
    pub fn get_active(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_Active"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The available torque, in Newton meters, that can be produced by this RCS, in the positive and negative pitch, roll and yaw axes of the vessel. These axes correspond to the coordinate axes of the <see cref="M:SpaceCenter.Vessel.ReferenceFrame" />. Returns zero if RCS is disable. </summary> </doc>
    pub fn get_available_torque(&self, ) -> CallHandle<((f64, f64, f64), (f64, f64, f64))> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_AvailableTorque"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the RCS thrusters are enabled. </summary> </doc>
    pub fn get_enabled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_Enabled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the RCS thruster will fire when pitch control input is given. </summary> </doc>
    pub fn get_forward_enabled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_ForwardEnabled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the RCS has fuel available. </summary> <remarks> The RCS thruster must be activated for this property to update correctly. </remarks> </doc>
    pub fn get_has_fuel(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_HasFuel"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The specific impulse of the RCS at sea level on Kerbin, in seconds. </summary> </doc>
    pub fn get_kerbin_sea_level_specific_impulse(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_KerbinSeaLevelSpecificImpulse"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The maximum amount of thrust that can be produced by the RCS thrusters when active, in Newtons. </summary> </doc>
    pub fn get_max_thrust(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_MaxThrust"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The maximum amount of thrust that can be produced by the RCS thrusters when active in a vacuum, in Newtons. </summary> </doc>
    pub fn get_max_vacuum_thrust(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_MaxVacuumThrust"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this RCS. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the RCS thruster will fire when pitch control input is given. </summary> </doc>
    pub fn get_pitch_enabled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_PitchEnabled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The ratios of resources that the RCS consumes. A dictionary mapping resource names to the ratios at which they are consumed by the RCS. </summary> </doc>
    pub fn get_propellant_ratios(&self, ) -> CallHandle<HashMap<String, f32>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_PropellantRatios"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The names of resources that the RCS consumes. </summary> </doc>
    pub fn get_propellants(&self, ) -> CallHandle<Vec<String>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_Propellants"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the RCS thruster will fire when roll control input is given. </summary> </doc>
    pub fn get_right_enabled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_RightEnabled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the RCS thruster will fire when roll control input is given. </summary> </doc>
    pub fn get_roll_enabled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_RollEnabled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current specific impulse of the RCS, in seconds. Returns zero if the RCS is not active. </summary> </doc>
    pub fn get_specific_impulse(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_SpecificImpulse"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of thrusters, one of each nozzel in the RCS part. </summary> </doc>
    pub fn get_thrusters(&self, ) -> CallHandle<Vec<Thruster>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_Thrusters"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the RCS thruster will fire when yaw control input is given. </summary> </doc>
    pub fn get_up_enabled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_UpEnabled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The vacuum specific impulse of the RCS, in seconds. </summary> </doc>
    pub fn get_vacuum_specific_impulse(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_VacuumSpecificImpulse"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the RCS thruster will fire when yaw control input is given. </summary> </doc>
    pub fn get_yaw_enabled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_get_YawEnabled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the RCS thrusters are enabled. </summary> </doc>
    pub fn set_enabled(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_set_Enabled"));

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

    /// <doc> <summary> Whether the RCS thruster will fire when pitch control input is given. </summary> </doc>
    pub fn set_forward_enabled(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_set_ForwardEnabled"));

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

    /// <doc> <summary> Whether the RCS thruster will fire when pitch control input is given. </summary> </doc>
    pub fn set_pitch_enabled(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_set_PitchEnabled"));

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

    /// <doc> <summary> Whether the RCS thruster will fire when roll control input is given. </summary> </doc>
    pub fn set_right_enabled(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_set_RightEnabled"));

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

    /// <doc> <summary> Whether the RCS thruster will fire when roll control input is given. </summary> </doc>
    pub fn set_roll_enabled(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_set_RollEnabled"));

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

    /// <doc> <summary> Whether the RCS thruster will fire when yaw control input is given. </summary> </doc>
    pub fn set_up_enabled(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_set_UpEnabled"));

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

    /// <doc> <summary> Whether the RCS thruster will fire when yaw control input is given. </summary> </doc>
    pub fn set_yaw_enabled(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("RCS_set_YawEnabled"));

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
impl Radiator {
    /// <doc> <summary> Whether the radiator is deployable. </summary> </doc>
    pub fn get_deployable(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Radiator_get_Deployable"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> For a deployable radiator, <c>true</c> if the radiator is extended. If the radiator is not deployable, this is always <c>true</c>. </summary> </doc>
    pub fn get_deployed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Radiator_get_Deployed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this radiator. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Radiator_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current state of the radiator. </summary> <remarks> A fixed radiator is always <see cref="M:SpaceCenter.RadiatorState.Extended" />. </remarks> </doc>
    pub fn get_state(&self, ) -> CallHandle<RadiatorState> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Radiator_get_State"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> For a deployable radiator, <c>true</c> if the radiator is extended. If the radiator is not deployable, this is always <c>true</c>. </summary> </doc>
    pub fn set_deployed(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Radiator_set_Deployed"));

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
impl ReactionWheel {
    /// <doc> <summary> Whether the reaction wheel is active. </summary> </doc>
    pub fn get_active(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ReactionWheel_get_Active"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The available torque, in Newton meters, that can be produced by this reaction wheel, in the positive and negative pitch, roll and yaw axes of the vessel. These axes correspond to the coordinate axes of the <see cref="M:SpaceCenter.Vessel.ReferenceFrame" />. Returns zero if the reaction wheel is inactive or broken. </summary> </doc>
    pub fn get_available_torque(&self, ) -> CallHandle<((f64, f64, f64), (f64, f64, f64))> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ReactionWheel_get_AvailableTorque"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the reaction wheel is broken. </summary> </doc>
    pub fn get_broken(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ReactionWheel_get_Broken"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The maximum torque, in Newton meters, that can be produced by this reaction wheel, when it is active, in the positive and negative pitch, roll and yaw axes of the vessel. These axes correspond to the coordinate axes of the <see cref="M:SpaceCenter.Vessel.ReferenceFrame" />. </summary> </doc>
    pub fn get_max_torque(&self, ) -> CallHandle<((f64, f64, f64), (f64, f64, f64))> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ReactionWheel_get_MaxTorque"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this reaction wheel. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ReactionWheel_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the reaction wheel is active. </summary> </doc>
    pub fn set_active(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ReactionWheel_set_Active"));

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
impl Resource {
    /// <doc> <summary> The amount of the resource that is currently stored in the part. </summary> </doc>
    pub fn get_amount(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resource_get_Amount"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The density of the resource, in <math>kg/l</math>. </summary> </doc>
    pub fn get_density(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resource_get_Density"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether use of this resource is enabled. </summary> </doc>
    pub fn get_enabled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resource_get_Enabled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The flow mode of the resource. </summary> </doc>
    pub fn get_flow_mode(&self, ) -> CallHandle<ResourceFlowMode> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resource_get_FlowMode"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The total amount of the resource that can be stored in the part. </summary> </doc>
    pub fn get_max(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resource_get_Max"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The name of the resource. </summary> </doc>
    pub fn get_name(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resource_get_Name"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part containing the resource. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resource_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether use of this resource is enabled. </summary> </doc>
    pub fn set_enabled(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resource_set_Enabled"));

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
impl ResourceConverter {
    /// <doc> <summary> True if the specified converter is active. </summary> <param name="index">Index of the converter.</param> </doc>
    pub fn active(&self, p_index: i32) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceConverter_Active"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_index.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> List of the names of resources consumed by the specified converter. </summary> <param name="index">Index of the converter.</param> </doc>
    pub fn inputs(&self, p_index: i32) -> CallHandle<Vec<String>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceConverter_Inputs"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_index.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The name of the specified converter. </summary> <param name="index">Index of the converter.</param> </doc>
    pub fn name(&self, p_index: i32) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceConverter_Name"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_index.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> List of the names of resources produced by the specified converter. </summary> <param name="index">Index of the converter.</param> </doc>
    pub fn outputs(&self, p_index: i32) -> CallHandle<Vec<String>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceConverter_Outputs"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_index.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Start the specified converter. </summary> <param name="index">Index of the converter.</param> </doc>
    pub fn start(&self, p_index: i32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceConverter_Start"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_index.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the specified converter. </summary> <param name="index">Index of the converter.</param> </doc>
    pub fn state(&self, p_index: i32) -> CallHandle<ResourceConverterState> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceConverter_State"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_index.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Status information for the specified converter. This is the full status message shown in the in-game UI. </summary> <param name="index">Index of the converter.</param> </doc>
    pub fn status_info(&self, p_index: i32) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceConverter_StatusInfo"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_index.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Stop the specified converter. </summary> <param name="index">Index of the converter.</param> </doc>
    pub fn stop(&self, p_index: i32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceConverter_Stop"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_index.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The core temperature of the converter, in Kelvin. </summary> </doc>
    pub fn get_core_temperature(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceConverter_get_CoreTemperature"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The number of converters in the part. </summary> </doc>
    pub fn get_count(&self, ) -> CallHandle<i32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceConverter_get_Count"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The core temperature at which the converter will operate with peak efficiency, in Kelvin. </summary> </doc>
    pub fn get_optimum_core_temperature(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceConverter_get_OptimumCoreTemperature"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this converter. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceConverter_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The thermal efficiency of the converter, as a percentage of its maximum. </summary> </doc>
    pub fn get_thermal_efficiency(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceConverter_get_ThermalEfficiency"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl ResourceHarvester {
    /// <doc> <summary> Whether the harvester is actively drilling. </summary> </doc>
    pub fn get_active(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceHarvester_get_Active"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The core temperature of the drill, in Kelvin. </summary> </doc>
    pub fn get_core_temperature(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceHarvester_get_CoreTemperature"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the harvester is deployed. </summary> </doc>
    pub fn get_deployed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceHarvester_get_Deployed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rate at which the drill is extracting ore, in units per second. </summary> </doc>
    pub fn get_extraction_rate(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceHarvester_get_ExtractionRate"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The core temperature at which the drill will operate with peak efficiency, in Kelvin. </summary> </doc>
    pub fn get_optimum_core_temperature(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceHarvester_get_OptimumCoreTemperature"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this harvester. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceHarvester_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The state of the harvester. </summary> </doc>
    pub fn get_state(&self, ) -> CallHandle<ResourceHarvesterState> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceHarvester_get_State"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The thermal efficiency of the drill, as a percentage of its maximum. </summary> </doc>
    pub fn get_thermal_efficiency(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceHarvester_get_ThermalEfficiency"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the harvester is actively drilling. </summary> </doc>
    pub fn set_active(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceHarvester_set_Active"));

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

    /// <doc> <summary> Whether the harvester is deployed. </summary> </doc>
    pub fn set_deployed(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceHarvester_set_Deployed"));

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
impl ResourceTransfer {
    /// <doc> <summary> The amount of the resource that has been transferred. </summary> </doc>
    pub fn get_amount(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceTransfer_get_Amount"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the transfer has completed. </summary> </doc>
    pub fn get_complete(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ResourceTransfer_get_Complete"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl Resources {
    /// <doc> <summary> Returns the amount of a resource that is currently stored. </summary> <param name="name">The name of the resource.</param> </doc>
    pub fn amount(&self, p_name: String) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resources_Amount"));

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

    /// <doc> <summary> Check whether the named resource can be stored. </summary> <param name="name">The name of the resource.</param> </doc>
    pub fn has_resource(&self, p_name: String) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resources_HasResource"));

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

    /// <doc> <summary> Returns the amount of a resource that can be stored. </summary> <param name="name">The name of the resource.</param> </doc>
    pub fn max(&self, p_name: String) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resources_Max"));

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

    /// <doc> <summary> All the individual resources with the given name that can be stored. </summary> </doc>
    pub fn with_resource(&self, p_name: String) -> CallHandle<Vec<Resource>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resources_WithResource"));

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

    /// <doc> <summary> All the individual resources that can be stored. </summary> </doc>
    pub fn get_all(&self, ) -> CallHandle<Vec<Resource>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resources_get_All"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether use of all the resources are enabled. </summary> <remarks> This is <c>true</c> if all of the resources are enabled. If any of the resources are not enabled, this is <c>false</c>. </remarks> </doc>
    pub fn get_enabled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resources_get_Enabled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of resource names that can be stored. </summary> </doc>
    pub fn get_names(&self, ) -> CallHandle<Vec<String>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resources_get_Names"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether use of all the resources are enabled. </summary> <remarks> This is <c>true</c> if all of the resources are enabled. If any of the resources are not enabled, this is <c>false</c>. </remarks> </doc>
    pub fn set_enabled(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Resources_set_Enabled"));

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
impl ScienceData {
    /// <doc> <summary> Data amount. </summary> </doc>
    pub fn get_data_amount(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ScienceData_get_DataAmount"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Science value. </summary> </doc>
    pub fn get_science_value(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ScienceData_get_ScienceValue"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Transmit value. </summary> </doc>
    pub fn get_transmit_value(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ScienceData_get_TransmitValue"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl ScienceSubject {
    /// <doc> <summary> Multiply science value by this to determine data amount in mits. </summary> </doc>
    pub fn get_data_scale(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ScienceSubject_get_DataScale"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the experiment has been completed. </summary> </doc>
    pub fn get_is_complete(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ScienceSubject_get_IsComplete"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Amount of science already earned from this subject, not updated until after transmission/recovery. </summary> </doc>
    pub fn get_science(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ScienceSubject_get_Science"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Total science allowable for this subject. </summary> </doc>
    pub fn get_science_cap(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ScienceSubject_get_ScienceCap"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Diminishing value multiplier for decreasing the science value returned from repeated experiments. </summary> </doc>
    pub fn get_scientific_value(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ScienceSubject_get_ScientificValue"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Multiplier for specific Celestial Body/Experiment Situation combination. </summary> </doc>
    pub fn get_subject_value(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ScienceSubject_get_SubjectValue"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Title of science subject, displayed in science archives </summary> </doc>
    pub fn get_title(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("ScienceSubject_get_Title"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl Sensor {
    /// <doc> <summary> Whether the sensor is active. </summary> </doc>
    pub fn get_active(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Sensor_get_Active"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this sensor. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Sensor_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current value of the sensor. </summary> </doc>
    pub fn get_value(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Sensor_get_Value"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the sensor is active. </summary> </doc>
    pub fn set_active(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Sensor_set_Active"));

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
impl SolarPanel {
    /// <doc> <summary> Whether the solar panel is deployable. </summary> </doc>
    pub fn get_deployable(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("SolarPanel_get_Deployable"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the solar panel is extended. </summary> </doc>
    pub fn get_deployed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("SolarPanel_get_Deployed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current amount of energy being generated by the solar panel, in units of charge per second. </summary> </doc>
    pub fn get_energy_flow(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("SolarPanel_get_EnergyFlow"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this solar panel. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("SolarPanel_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current state of the solar panel. </summary> </doc>
    pub fn get_state(&self, ) -> CallHandle<SolarPanelState> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("SolarPanel_get_State"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current amount of sunlight that is incident on the solar panel, as a percentage. A value between 0 and 1. </summary> </doc>
    pub fn get_sun_exposure(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("SolarPanel_get_SunExposure"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the solar panel is extended. </summary> </doc>
    pub fn set_deployed(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("SolarPanel_set_Deployed"));

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
impl Thruster {
    /// <doc> <summary> Position around which the gimbal pivots. </summary> <returns>The position as a vector.</returns> <param name="referenceFrame">The reference frame that the returned position vector is in.</param> </doc>
    pub fn gimbal_position(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Thruster_GimbalPosition"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The direction of the force generated by the thruster, when the engine is in its initial position (no gimballing), in the given reference frame. This is opposite to the direction in which the thruster expels propellant. </summary> <returns>The direction as a unit vector.</returns> <param name="referenceFrame">The reference frame that the returned direction is in.</param> </doc>
    pub fn initial_thrust_direction(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Thruster_InitialThrustDirection"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position at which the thruster generates thrust, when the engine is in its initial position (no gimballing), in the given reference frame. </summary> <returns>The position as a vector.</returns> <param name="referenceFrame">The reference frame that the returned position vector is in.</param> <remarks> This position can move when the gimbal rotates. This is because the thrust position and gimbal position are not necessarily the same. </remarks> </doc>
    pub fn initial_thrust_position(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Thruster_InitialThrustPosition"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The direction of the force generated by the thruster, in the given reference frame. This is opposite to the direction in which the thruster expels propellant. For gimballed engines, this takes into account the current rotation of the gimbal. </summary> <returns>The direction as a unit vector.</returns> <param name="referenceFrame">The reference frame that the returned direction is in.</param> </doc>
    pub fn thrust_direction(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Thruster_ThrustDirection"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position at which the thruster generates thrust, in the given reference frame. For gimballed engines, this takes into account the current rotation of the gimbal. </summary> <returns>The position as a vector.</returns> <param name="referenceFrame">The reference frame that the returned position vector is in.</param> </doc>
    pub fn thrust_position(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Thruster_ThrustPosition"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current gimbal angle in the pitch, roll and yaw axes, in degrees. </summary> </doc>
    pub fn get_gimbal_angle(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Thruster_get_GimbalAngle"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the thruster is gimballed. </summary> </doc>
    pub fn get_gimballed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Thruster_get_Gimballed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The <see cref="T:SpaceCenter.Part" /> that contains this thruster. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Thruster_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A reference frame that is fixed relative to the thruster and orientated with its thrust direction (<see cref="M:SpaceCenter.Thruster.ThrustDirection" />). For gimballed engines, this takes into account the current rotation of the gimbal. <list type="bullet"><item><description> The origin is at the position of thrust for this thruster (<see cref="M:SpaceCenter.Thruster.ThrustPosition" />).</description></item><item><description> The axes rotate with the thrust direction. This is the direction in which the thruster expels propellant, including any gimballing. </description></item><item><description>The y-axis points along the thrust direction.</description></item><item><description>The x-axis and z-axis are perpendicular to the thrust direction. </description></item></list></summary> </doc>
    pub fn get_thrust_reference_frame(&self, ) -> CallHandle<ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Thruster_get_ThrustReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl Vessel {
    /// <doc> <summary> The angular velocity of the vessel, in the given reference frame. </summary> <returns>The angular velocity as a vector. The magnitude of the vector is the rotational speed of the vessel, in radians per second. The direction of the vector indicates the axis of rotation, using the right-hand rule.</returns> <param name="referenceFrame">The reference frame the returned angular velocity is in.</param> </doc>
    pub fn angular_velocity(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_AngularVelocity"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The axis-aligned bounding box of the vessel in the given reference frame. </summary> <returns>The positions of the minimum and maximum vertices of the box, as position vectors.</returns> <param name="referenceFrame">The reference frame that the returned position vectors are in.</param> </doc>
    pub fn bounding_box(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<((f64, f64, f64), (f64, f64, f64))> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_BoundingBox"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The direction in which the vessel is pointing, in the given reference frame. </summary> <returns>The direction as a unit vector.</returns> <param name="referenceFrame">The reference frame that the returned direction is in.</param> </doc>
    pub fn direction(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_Direction"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns a <see cref="T:SpaceCenter.Flight" /> object that can be used to get flight telemetry for the vessel, in the specified reference frame. </summary> <param name="referenceFrame"> Reference frame. Defaults to the vessel's surface reference frame (<see cref="M:SpaceCenter.Vessel.SurfaceReferenceFrame" />). </param> </doc>
    pub fn flight(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<Flight> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_Flight"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The position of the center of mass of the vessel, in the given reference frame. </summary> <returns>The position as a vector.</returns> <param name="referenceFrame">The reference frame that the returned position vector is in.</param> </doc>
    pub fn position(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_Position"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Recover the vessel. </summary> </doc>
    pub fn recover(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_Recover"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns a <see cref="T:SpaceCenter.Resources" /> object, that can used to get information about resources stored in a given <paramref name="stage" />. </summary> <param name="stage">Get resources for parts that are decoupled in this stage.</param> <param name="cumulative">When <c>false</c>, returns the resources for parts decoupled in just the given stage. When <c>true</c> returns the resources decoupled in the given stage and all subsequent stages combined.</param> </doc>
    pub fn resources_in_decouple_stage(&self, p_stage: i32, p_cumulative: bool) -> CallHandle<Resources> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_ResourcesInDecoupleStage"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_stage.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_cumulative.encode_to_bytes().unwrap());
        arguments.push(arg2);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The rotation of the vessel, in the given reference frame. </summary> <returns>The rotation as a quaternion of the form <math>(x, y, z, w)</math>.</returns> <param name="referenceFrame">The reference frame that the returned rotation is in.</param> </doc>
    pub fn rotation(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_Rotation"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The velocity of the center of mass of the vessel, in the given reference frame. </summary> <returns>The velocity as a vector. The vector points in the direction of travel, and its magnitude is the speed of the body in meters per second.</returns> <param name="referenceFrame">The reference frame that the returned velocity vector is in.</param> </doc>
    pub fn velocity(&self, p_reference_frame: &ReferenceFrame) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_Velocity"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_reference_frame.encode_to_bytes().unwrap());
        arguments.push(arg1);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> An <see cref="T:SpaceCenter.AutoPilot" /> object, that can be used to perform simple auto-piloting of the vessel. </summary> </doc>
    pub fn get_auto_pilot(&self, ) -> CallHandle<AutoPilot> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_AutoPilot"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The maximum torque that the aerodynamic control surfaces can generate. Returns the torques in <math>N.m</math> around each of the coordinate axes of the vessels reference frame (<see cref="T:SpaceCenter.ReferenceFrame" />). These axes are equivalent to the pitch, roll and yaw axes of the vessel. </summary> </doc>
    pub fn get_available_control_surface_torque(&self, ) -> CallHandle<((f64, f64, f64), (f64, f64, f64))> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_AvailableControlSurfaceTorque"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The maximum torque that the currently active and gimballed engines can generate. Returns the torques in <math>N.m</math> around each of the coordinate axes of the vessels reference frame (<see cref="T:SpaceCenter.ReferenceFrame" />). These axes are equivalent to the pitch, roll and yaw axes of the vessel. </summary> </doc>
    pub fn get_available_engine_torque(&self, ) -> CallHandle<((f64, f64, f64), (f64, f64, f64))> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_AvailableEngineTorque"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The maximum torque that parts (excluding reaction wheels, gimballed engines, RCS and control surfaces) can generate. Returns the torques in <math>N.m</math> around each of the coordinate axes of the vessels reference frame (<see cref="T:SpaceCenter.ReferenceFrame" />). These axes are equivalent to the pitch, roll and yaw axes of the vessel. </summary> </doc>
    pub fn get_available_other_torque(&self, ) -> CallHandle<((f64, f64, f64), (f64, f64, f64))> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_AvailableOtherTorque"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The maximum torque that the currently active RCS thrusters can generate. Returns the torques in <math>N.m</math> around each of the coordinate axes of the vessels reference frame (<see cref="T:SpaceCenter.ReferenceFrame" />). These axes are equivalent to the pitch, roll and yaw axes of the vessel. </summary> </doc>
    pub fn get_available_rcs_torque(&self, ) -> CallHandle<((f64, f64, f64), (f64, f64, f64))> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_AvailableRCSTorque"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The maximum torque that the currently active and powered reaction wheels can generate. Returns the torques in <math>N.m</math> around each of the coordinate axes of the vessels reference frame (<see cref="T:SpaceCenter.ReferenceFrame" />). These axes are equivalent to the pitch, roll and yaw axes of the vessel. </summary> </doc>
    pub fn get_available_reaction_wheel_torque(&self, ) -> CallHandle<((f64, f64, f64), (f64, f64, f64))> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_AvailableReactionWheelTorque"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Gets the total available thrust that can be produced by the vessel's active engines, in Newtons. This is computed by summing <see cref="M:SpaceCenter.Engine.AvailableThrust" /> for every active engine in the vessel. </summary> </doc>
    pub fn get_available_thrust(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_AvailableThrust"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The maximum torque that the vessel generates. Includes contributions from reaction wheels, RCS, gimballed engines and aerodynamic control surfaces. Returns the torques in <math>N.m</math> around each of the coordinate axes of the vessels reference frame (<see cref="T:SpaceCenter.ReferenceFrame" />). These axes are equivalent to the pitch, roll and yaw axes of the vessel. </summary> </doc>
    pub fn get_available_torque(&self, ) -> CallHandle<((f64, f64, f64), (f64, f64, f64))> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_AvailableTorque"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The name of the biome the vessel is currently in. </summary> </doc>
    pub fn get_biome(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_Biome"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns a <see cref="T:SpaceCenter.Comms" /> object that can be used to interact with CommNet for this vessel. </summary> </doc>
    pub fn get_comms(&self, ) -> CallHandle<Comms> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_Comms"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns a <see cref="T:SpaceCenter.Control" /> object that can be used to manipulate the vessel's control inputs. For example, its pitch/yaw/roll controls, RCS and thrust. </summary> </doc>
    pub fn get_control(&self, ) -> CallHandle<Control> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_Control"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The crew in the vessel. </summary> </doc>
    pub fn get_crew(&self, ) -> CallHandle<Vec<CrewMember>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_Crew"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The number of crew that can occupy the vessel. </summary> </doc>
    pub fn get_crew_capacity(&self, ) -> CallHandle<i32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_CrewCapacity"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The number of crew that are occupying the vessel. </summary> </doc>
    pub fn get_crew_count(&self, ) -> CallHandle<i32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_CrewCount"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The total mass of the vessel, excluding resources, in kg. </summary> </doc>
    pub fn get_dry_mass(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_DryMass"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The inertia tensor of the vessel around its center of mass, in the vessels reference frame (<see cref="T:SpaceCenter.ReferenceFrame" />). Returns the 3x3 matrix as a list of elements, in row-major order. </summary> </doc>
    pub fn get_inertia_tensor(&self, ) -> CallHandle<Vec<f64>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_InertiaTensor"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The combined specific impulse of all active engines at sea level on Kerbin, in seconds. This is computed using the formula <a href="https://wiki.kerbalspaceprogram.com/wiki/Specific_impulse#Multiple_engines">described here</a>. </summary> </doc>
    pub fn get_kerbin_sea_level_specific_impulse(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_KerbinSeaLevelSpecificImpulse"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The mission elapsed time in seconds. </summary> </doc>
    pub fn get_met(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_MET"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The total mass of the vessel, including resources, in kg. </summary> </doc>
    pub fn get_mass(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_Mass"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The total maximum thrust that can be produced by the vessel's active engines, in Newtons. This is computed by summing <see cref="M:SpaceCenter.Engine.MaxThrust" /> for every active engine. </summary> </doc>
    pub fn get_max_thrust(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_MaxThrust"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The total maximum thrust that can be produced by the vessel's active engines when the vessel is in a vacuum, in Newtons. This is computed by summing <see cref="M:SpaceCenter.Engine.MaxVacuumThrust" /> for every active engine. </summary> </doc>
    pub fn get_max_vacuum_thrust(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_MaxVacuumThrust"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The moment of inertia of the vessel around its center of mass in <math>kg.m^2</math>. The inertia values in the returned 3-tuple are around the pitch, roll and yaw directions respectively. This corresponds to the vessels reference frame (<see cref="T:SpaceCenter.ReferenceFrame" />). </summary> </doc>
    pub fn get_moment_of_inertia(&self, ) -> CallHandle<(f64, f64, f64)> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_MomentOfInertia"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The name of the vessel. </summary> </doc>
    pub fn get_name(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_Name"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current orbit of the vessel. </summary> </doc>
    pub fn get_orbit(&self, ) -> CallHandle<Orbit> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_Orbit"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The reference frame that is fixed relative to the vessel, and orientated with the vessels orbital prograde/normal/radial directions. <list type="bullet"><item><description>The origin is at the center of mass of the vessel.</description></item><item><description>The axes rotate with the orbital prograde/normal/radial directions.</description></item><item><description>The x-axis points in the orbital anti-radial direction.</description></item><item><description>The y-axis points in the orbital prograde direction.</description></item><item><description>The z-axis points in the orbital normal direction.</description></item></list></summary> <remarks> Be careful not to confuse this with 'orbit' mode on the navball. </remarks> </doc>
    pub fn get_orbital_reference_frame(&self, ) -> CallHandle<ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_OrbitalReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.Parts" /> object, that can used to interact with the parts that make up this vessel. </summary> </doc>
    pub fn get_parts(&self, ) -> CallHandle<Parts> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_Parts"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the vessel is recoverable. </summary> </doc>
    pub fn get_recoverable(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_Recoverable"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The reference frame that is fixed relative to the vessel, and orientated with the vessel. <list type="bullet"><item><description>The origin is at the center of mass of the vessel.</description></item><item><description>The axes rotate with the vessel.</description></item><item><description>The x-axis points out to the right of the vessel.</description></item><item><description>The y-axis points in the forward direction of the vessel.</description></item><item><description>The z-axis points out of the bottom off the vessel.</description></item></list></summary> </doc>
    pub fn get_reference_frame(&self, ) -> CallHandle<ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_ReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A <see cref="T:SpaceCenter.Resources" /> object, that can used to get information about resources stored in the vessel. </summary> </doc>
    pub fn get_resources(&self, ) -> CallHandle<Resources> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_Resources"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The situation the vessel is in. </summary> </doc>
    pub fn get_situation(&self, ) -> CallHandle<VesselSituation> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_Situation"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The combined specific impulse of all active engines, in seconds. This is computed using the formula <a href="https://wiki.kerbalspaceprogram.com/wiki/Specific_impulse#Multiple_engines">described here</a>. </summary> </doc>
    pub fn get_specific_impulse(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_SpecificImpulse"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The reference frame that is fixed relative to the vessel, and orientated with the surface of the body being orbited. <list type="bullet"><item><description>The origin is at the center of mass of the vessel.</description></item><item><description>The axes rotate with the north and up directions on the surface of the body.</description></item><item><description>The x-axis points in the <a href="https://en.wikipedia.org/wiki/Zenith">zenith</a> direction (upwards, normal to the body being orbited, from the center of the body towards the center of mass of the vessel).</description></item><item><description>The y-axis points northwards towards the <a href="https://en.wikipedia.org/wiki/Horizon">astronomical horizon</a> (north, and tangential to the surface of the body -- the direction in which a compass would point when on the surface).</description></item><item><description>The z-axis points eastwards towards the <a href="https://en.wikipedia.org/wiki/Horizon">astronomical horizon</a> (east, and tangential to the surface of the body -- east on a compass when on the surface).</description></item></list></summary> <remarks> Be careful not to confuse this with 'surface' mode on the navball. </remarks> </doc>
    pub fn get_surface_reference_frame(&self, ) -> CallHandle<ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_SurfaceReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The reference frame that is fixed relative to the vessel, and orientated with the velocity vector of the vessel relative to the surface of the body being orbited. <list type="bullet"><item><description>The origin is at the center of mass of the vessel.</description></item><item><description>The axes rotate with the vessel's velocity vector.</description></item><item><description>The y-axis points in the direction of the vessel's velocity vector, relative to the surface of the body being orbited.</description></item><item><description>The z-axis is in the plane of the <a href="https://en.wikipedia.org/wiki/Horizon">astronomical horizon</a>.</description></item><item><description>The x-axis is orthogonal to the other two axes.</description></item></list></summary> </doc>
    pub fn get_surface_velocity_reference_frame(&self, ) -> CallHandle<ReferenceFrame> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_SurfaceVelocityReferenceFrame"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The total thrust currently being produced by the vessel's engines, in Newtons. This is computed by summing <see cref="M:SpaceCenter.Engine.Thrust" /> for every engine in the vessel. </summary> </doc>
    pub fn get_thrust(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_Thrust"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The type of the vessel. </summary> </doc>
    pub fn get_type(&self, ) -> CallHandle<VesselType> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_Type"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The combined vacuum specific impulse of all active engines, in seconds. This is computed using the formula <a href="https://wiki.kerbalspaceprogram.com/wiki/Specific_impulse#Multiple_engines">described here</a>. </summary> </doc>
    pub fn get_vacuum_specific_impulse(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_get_VacuumSpecificImpulse"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The name of the vessel. </summary> </doc>
    pub fn set_name(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_set_Name"));

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

    /// <doc> <summary> The type of the vessel. </summary> </doc>
    pub fn set_type(&self, p_value: VesselType) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Vessel_set_Type"));

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
impl Waypoint {
    /// <doc> <summary> Removes the waypoint. </summary> </doc>
    pub fn remove(&self, ) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_Remove"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The altitude of the waypoint above the surface of the body, in meters. When over water, this is the altitude above the sea floor. </summary> </doc>
    pub fn get_bedrock_altitude(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_get_BedrockAltitude"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The celestial body the waypoint is attached to. </summary> </doc>
    pub fn get_body(&self, ) -> CallHandle<CelestialBody> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_get_Body"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary><c>true</c> if this waypoint is part of a set of clustered waypoints with greek letter names appended (Alpha, Beta, Gamma, etc). If <c>true</c>, there is a one-to-one correspondence with the greek letter name and the <see cref="M:SpaceCenter.Waypoint.Index" />. </summary> </doc>
    pub fn get_clustered(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_get_Clustered"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The seed of the icon color. See <see cref="M:SpaceCenter.WaypointManager.Colors" /> for example colors. </summary> </doc>
    pub fn get_color(&self, ) -> CallHandle<i32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_get_Color"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The associated contract. </summary> </doc>
    pub fn get_contract(&self, ) -> CallHandle<Contract> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_get_Contract"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary><c>true</c> if the waypoint is attached to the ground. </summary> </doc>
    pub fn get_grounded(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_get_Grounded"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the waypoint belongs to a contract. </summary> </doc>
    pub fn get_has_contract(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_get_HasContract"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The icon of the waypoint. </summary> </doc>
    pub fn get_icon(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_get_Icon"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The integer index of this waypoint within its cluster of sibling waypoints. In other words, when you have a cluster of waypoints called "Somewhere Alpha", "Somewhere Beta" and "Somewhere Gamma", the alpha site has index 0, the beta site has index 1 and the gamma site has index 2. When <see cref="M:SpaceCenter.Waypoint.Clustered" /> is <c>false</c>, this is zero. </summary> </doc>
    pub fn get_index(&self, ) -> CallHandle<i32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_get_Index"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The latitude of the waypoint. </summary> </doc>
    pub fn get_latitude(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_get_Latitude"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The longitude of the waypoint. </summary> </doc>
    pub fn get_longitude(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_get_Longitude"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The altitude of the waypoint above sea level, in meters. </summary> </doc>
    pub fn get_mean_altitude(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_get_MeanAltitude"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The name of the waypoint as it appears on the map and the contract. </summary> </doc>
    pub fn get_name(&self, ) -> CallHandle<String> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_get_Name"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary><c>true</c> if the waypoint is near to the surface of a body. </summary> </doc>
    pub fn get_near_surface(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_get_NearSurface"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The altitude of the waypoint above the surface of the body or sea level, whichever is closer, in meters. </summary> </doc>
    pub fn get_surface_altitude(&self, ) -> CallHandle<f64> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_get_SurfaceAltitude"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The altitude of the waypoint above the surface of the body, in meters. When over water, this is the altitude above the sea floor. </summary> </doc>
    pub fn set_bedrock_altitude(&self, p_value: f64) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_set_BedrockAltitude"));

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

    /// <doc> <summary> The celestial body the waypoint is attached to. </summary> </doc>
    pub fn set_body(&self, p_value: &CelestialBody) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_set_Body"));

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

    /// <doc> <summary> The seed of the icon color. See <see cref="M:SpaceCenter.WaypointManager.Colors" /> for example colors. </summary> </doc>
    pub fn set_color(&self, p_value: i32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_set_Color"));

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

    /// <doc> <summary> The icon of the waypoint. </summary> </doc>
    pub fn set_icon(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_set_Icon"));

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

    /// <doc> <summary> The latitude of the waypoint. </summary> </doc>
    pub fn set_latitude(&self, p_value: f64) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_set_Latitude"));

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

    /// <doc> <summary> The longitude of the waypoint. </summary> </doc>
    pub fn set_longitude(&self, p_value: f64) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_set_Longitude"));

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

    /// <doc> <summary> The altitude of the waypoint above sea level, in meters. </summary> </doc>
    pub fn set_mean_altitude(&self, p_value: f64) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_set_MeanAltitude"));

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

    /// <doc> <summary> The name of the waypoint as it appears on the map and the contract. </summary> </doc>
    pub fn set_name(&self, p_value: String) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_set_Name"));

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

    /// <doc> <summary> The altitude of the waypoint above the surface of the body or sea level, whichever is closer, in meters. </summary> </doc>
    pub fn set_surface_altitude(&self, p_value: f64) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Waypoint_set_SurfaceAltitude"));

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
impl WaypointManager {
    /// <doc> <summary> Creates a waypoint at the given position at ground level, and returns a <see cref="T:SpaceCenter.Waypoint" /> object that can be used to modify it. </summary> <param name="latitude">Latitude of the waypoint.</param> <param name="longitude">Longitude of the waypoint.</param> <param name="body">Celestial body the waypoint is attached to.</param> <param name="name">Name of the waypoint.</param> <returns></returns> </doc>
    pub fn add_waypoint(&self, p_latitude: f64, p_longitude: f64, p_body: &CelestialBody, p_name: String) -> CallHandle<Waypoint> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("WaypointManager_AddWaypoint"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_latitude.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_longitude.encode_to_bytes().unwrap());
        arguments.push(arg2);

        let mut arg3 = krpc::Argument::new();
        arg3.set_position(3);
        arg3.set_value(p_body.encode_to_bytes().unwrap());
        arguments.push(arg3);

        let mut arg4 = krpc::Argument::new();
        arg4.set_position(4);
        arg4.set_value(p_name.encode_to_bytes().unwrap());
        arguments.push(arg4);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Creates a waypoint at the given position and altitude, and returns a <see cref="T:SpaceCenter.Waypoint" /> object that can be used to modify it. </summary> <param name="latitude">Latitude of the waypoint.</param> <param name="longitude">Longitude of the waypoint.</param> <param name="altitude">Altitude (above sea level) of the waypoint.</param> <param name="body">Celestial body the waypoint is attached to.</param> <param name="name">Name of the waypoint.</param> <returns></returns> </doc>
    pub fn add_waypoint_at_altitude(&self, p_latitude: f64, p_longitude: f64, p_altitude: f64, p_body: &CelestialBody, p_name: String) -> CallHandle<Waypoint> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("WaypointManager_AddWaypointAtAltitude"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);

        let mut arg1 = krpc::Argument::new();
        arg1.set_position(1);
        arg1.set_value(p_latitude.encode_to_bytes().unwrap());
        arguments.push(arg1);

        let mut arg2 = krpc::Argument::new();
        arg2.set_position(2);
        arg2.set_value(p_longitude.encode_to_bytes().unwrap());
        arguments.push(arg2);

        let mut arg3 = krpc::Argument::new();
        arg3.set_position(3);
        arg3.set_value(p_altitude.encode_to_bytes().unwrap());
        arguments.push(arg3);

        let mut arg4 = krpc::Argument::new();
        arg4.set_position(4);
        arg4.set_value(p_body.encode_to_bytes().unwrap());
        arguments.push(arg4);

        let mut arg5 = krpc::Argument::new();
        arg5.set_position(5);
        arg5.set_value(p_name.encode_to_bytes().unwrap());
        arguments.push(arg5);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> An example map of known color - seed pairs. Any other integers may be used as seed. </summary> </doc>
    pub fn get_colors(&self, ) -> CallHandle<HashMap<String, i32>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("WaypointManager_get_Colors"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Returns all available icons (from "GameData/Squad/Contracts/Icons/"). </summary> </doc>
    pub fn get_icons(&self, ) -> CallHandle<Vec<String>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("WaypointManager_get_Icons"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> A list of all existing waypoints. </summary> </doc>
    pub fn get_waypoints(&self, ) -> CallHandle<Vec<Waypoint>> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("WaypointManager_get_Waypoints"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }
}
impl Wheel {
    /// <doc> <summary> Whether automatic friction control is enabled. </summary> </doc>
    pub fn get_auto_friction_control(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_AutoFrictionControl"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The braking force, as a percentage of maximum, when the brakes are applied. </summary> </doc>
    pub fn get_brakes(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_Brakes"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the wheel is broken. </summary> </doc>
    pub fn get_broken(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_Broken"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Current deflection of the wheel. </summary> </doc>
    pub fn get_deflection(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_Deflection"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the wheel is deployable. </summary> </doc>
    pub fn get_deployable(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_Deployable"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the wheel is deployed. </summary> </doc>
    pub fn get_deployed(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_Deployed"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Manual setting for the motor limiter. Only takes effect if the wheel has automatic traction control disabled. A value between 0 and 100 inclusive. </summary> </doc>
    pub fn get_drive_limiter(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_DriveLimiter"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the wheel is touching the ground. </summary> </doc>
    pub fn get_grounded(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_Grounded"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the wheel has brakes. </summary> </doc>
    pub fn get_has_brakes(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_HasBrakes"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the wheel has suspension. </summary> </doc>
    pub fn get_has_suspension(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_HasSuspension"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Manual friction control value. Only has an effect if automatic friction control is disabled. A value between 0 and 5 inclusive. </summary> </doc>
    pub fn get_manual_friction_control(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_ManualFrictionControl"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the motor is enabled. </summary> </doc>
    pub fn get_motor_enabled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_MotorEnabled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the direction of the motor is inverted. </summary> </doc>
    pub fn get_motor_inverted(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_MotorInverted"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The output of the motor. This is the torque currently being generated, in Newton meters. </summary> </doc>
    pub fn get_motor_output(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_MotorOutput"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the direction of the motor is inverted. </summary> </doc>
    pub fn get_motor_state(&self, ) -> CallHandle<MotorState> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_MotorState"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The part object for this wheel. </summary> </doc>
    pub fn get_part(&self, ) -> CallHandle<Part> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_Part"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the wheel is powered by a motor. </summary> </doc>
    pub fn get_powered(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_Powered"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Radius of the wheel, in meters. </summary> </doc>
    pub fn get_radius(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_Radius"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the wheel is repairable. </summary> </doc>
    pub fn get_repairable(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_Repairable"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Current slip of the wheel. </summary> </doc>
    pub fn get_slip(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_Slip"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> The current state of the wheel. </summary> </doc>
    pub fn get_state(&self, ) -> CallHandle<WheelState> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_State"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the wheel has steering. </summary> </doc>
    pub fn get_steerable(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_Steerable"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the wheel steering is enabled. </summary> </doc>
    pub fn get_steering_enabled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_SteeringEnabled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether the wheel steering is inverted. </summary> </doc>
    pub fn get_steering_inverted(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_SteeringInverted"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Current stress on the wheel. </summary> </doc>
    pub fn get_stress(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_Stress"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Current stress on the wheel as a percentage of its stress tolerance. </summary> </doc>
    pub fn get_stress_percentage(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_StressPercentage"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Stress tolerance of the wheel. </summary> </doc>
    pub fn get_stress_tolerance(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_StressTolerance"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Suspension damper strength, as set in the editor. </summary> </doc>
    pub fn get_suspension_damper_strength(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_SuspensionDamperStrength"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Suspension spring strength, as set in the editor. </summary> </doc>
    pub fn get_suspension_spring_strength(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_SuspensionSpringStrength"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Setting for the traction control. Only takes effect if the wheel has automatic traction control enabled. A value between 0 and 5 inclusive. </summary> </doc>
    pub fn get_traction_control(&self, ) -> CallHandle<f32> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_TractionControl"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether automatic traction control is enabled. A wheel only has traction control if it is powered. </summary> </doc>
    pub fn get_traction_control_enabled(&self, ) -> CallHandle<bool> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_get_TractionControlEnabled"));

        let mut arguments = protobuf::RepeatedField::<krpc::Argument>::new();

        let mut arg0 = krpc::Argument::new();
        arg0.set_position(0);
        arg0.set_value(self.id.encode_to_bytes().unwrap());
        arguments.push(arg0);
        proc_call.set_arguments(arguments);

        CallHandle::new(proc_call)
    }

    /// <doc> <summary> Whether automatic friction control is enabled. </summary> </doc>
    pub fn set_auto_friction_control(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_set_AutoFrictionControl"));

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

    /// <doc> <summary> The braking force, as a percentage of maximum, when the brakes are applied. </summary> </doc>
    pub fn set_brakes(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_set_Brakes"));

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

    /// <doc> <summary> Whether the wheel is deployed. </summary> </doc>
    pub fn set_deployed(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_set_Deployed"));

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

    /// <doc> <summary> Manual setting for the motor limiter. Only takes effect if the wheel has automatic traction control disabled. A value between 0 and 100 inclusive. </summary> </doc>
    pub fn set_drive_limiter(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_set_DriveLimiter"));

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

    /// <doc> <summary> Manual friction control value. Only has an effect if automatic friction control is disabled. A value between 0 and 5 inclusive. </summary> </doc>
    pub fn set_manual_friction_control(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_set_ManualFrictionControl"));

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

    /// <doc> <summary> Whether the motor is enabled. </summary> </doc>
    pub fn set_motor_enabled(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_set_MotorEnabled"));

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

    /// <doc> <summary> Whether the direction of the motor is inverted. </summary> </doc>
    pub fn set_motor_inverted(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_set_MotorInverted"));

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

    /// <doc> <summary> Whether the wheel steering is enabled. </summary> </doc>
    pub fn set_steering_enabled(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_set_SteeringEnabled"));

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

    /// <doc> <summary> Whether the wheel steering is inverted. </summary> </doc>
    pub fn set_steering_inverted(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_set_SteeringInverted"));

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

    /// <doc> <summary> Setting for the traction control. Only takes effect if the wheel has automatic traction control enabled. A value between 0 and 5 inclusive. </summary> </doc>
    pub fn set_traction_control(&self, p_value: f32) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_set_TractionControl"));

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

    /// <doc> <summary> Whether automatic traction control is enabled. A wheel only has traction control if it is powered. </summary> </doc>
    pub fn set_traction_control_enabled(&self, p_value: bool) -> CallHandle<()> {
        let mut proc_call = krpc::ProcedureCall::new();
        proc_call.set_service(String::from("SpaceCenter"));
        proc_call.set_procedure(String::from("Wheel_set_TractionControlEnabled"));

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

