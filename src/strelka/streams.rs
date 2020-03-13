use std::string::ToString;

#[derive(Debug, Copy, Clone)]
pub enum StreamUpdate {
    Altitude(f64),
    UniversalTime(f64),
    Pitch(f32),
}

impl ToString for StreamUpdate {

    fn to_string(&self) -> String {
        match self {
            StreamUpdate::Altitude(_) => "Altitude",
            StreamUpdate::UniversalTime(_) => "UniversalTime",
            StreamUpdate::Pitch(_) => "Pitch",
        }.to_owned()
    }

}