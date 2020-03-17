use std::string::ToString;
use std::f64;
use std::convert::From;
use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
pub enum StreamUpdate {
    Altitude(f64),
    UniversalTime(f64),
    Pitch(f64),
    Apoapsis(f64),
    TimeToApoapsis(f64),
    EnginesDepleted(i16),
}

// TODO: If we don't want to keep maintaining this list, could drop strum back in
impl ToString for StreamUpdate {

    fn to_string(&self) -> String {
        match self {
            StreamUpdate::Altitude(_) => "Altitude",
            StreamUpdate::UniversalTime(_) => "UniversalTime",
            StreamUpdate::Pitch(_) => "Pitch",
            StreamUpdate::Apoapsis(_) => "Apoapsis",
            StreamUpdate::TimeToApoapsis(_) => "TimeToApoapsis",
            StreamUpdate::EnginesDepleted(_) => "EnginesDepleted",
        }.to_owned()
    }

}

// TODO: Maybe a better place for this to live
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {

    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3{ x, y, z }
    }

    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }

    // TODO: Will need this later ...
    // pub fn cross(&self, v: &Vector3) -> Vector3 {
    //     return Vector3::new(
    //         self.y * v.z - self.z * v.y,
    //         self.z * v.x - self.x * v.z,
    //         self.x * v.y - self.y * v.x
    //     )
    // }

    pub fn dot(&self, v: &Vector3) -> f64 {
        return self.x * v.x + self.y * v.y + self.z * v.z;
    }

    pub fn magnitude(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn angle_between(&self, v: &Vector3) -> f64 {
        let dot = self.dot(v);
        if dot == 0.0 {
            return 0.0
        }

        let mag_self = self.magnitude();
        let mag_v = v.magnitude();
        (dot / (mag_self * mag_v)).acos() * (180.0 / f64::consts::PI)
    }

}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.x() - other.x(), 
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl From<(f64, f64, f64)> for Vector3 {

    fn from(v: (f64, f64, f64)) -> Self {
        Vector3::new(v.0, v.1, v.2)
    }

}