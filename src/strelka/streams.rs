use strum_macros::Display;

#[derive(Display, Copy, Clone)]
pub enum StreamUpdate {
    Altitude(f64),
}
