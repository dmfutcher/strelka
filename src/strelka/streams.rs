use strum_macros::{Display, EnumString};

#[derive(Debug, Display, Copy, Clone, EnumString)]
pub enum StreamUpdate {
    Altitude(f64),
    #[strum(serialize="ut")]
    UniversalTime(f64),
}

