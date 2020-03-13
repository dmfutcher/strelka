use actix::prelude::*;
use krpc_mars::{RPCClient, StreamClient};

use crate::krpc::space_center;
use crate::strelka::streams::StreamUpdate;

/// Streamer periodically gets polls and fetches values from KRPC streams
pub struct Streamer {
    stream_client: StreamClient,
    
    ut: Option<krpc_mars::StreamHandle<f64>>,
    alt: Option<krpc_mars::StreamHandle<f64>>,
    pitch: Option<krpc_mars::StreamHandle<f32>>,
}

impl Streamer {
    pub fn new(krpc_client: RPCClient, stream_client: StreamClient) -> Self {
        let mut streamer = Streamer{ stream_client, ut: None, alt: None, pitch: None };
        streamer.initialise_streams(krpc_client); // TODO: This is a Result

        streamer
    }

    fn initialise_streams(&mut self, client: RPCClient) -> Result<(), failure::Error> {
        // TODO: Maybe refactor all these calls. Seems very likely to be repeated all over the codebase without some thought.
        let vessel = client.mk_call(&space_center::get_active_vessel())?;
        let reference_frame = client.mk_call(&vessel.get_reference_frame())?;
        let flight = client.mk_call(&vessel.flight(&reference_frame))?;

        let ut_handle = client.mk_call(&space_center::get_ut().to_stream())?;
        let alt_handle = client.mk_call(&flight.get_mean_altitude().to_stream())?;
        let pitch_handle = client.mk_call(&flight.get_pitch().to_stream())?;

        self.ut = Some(ut_handle);
        self.alt = Some(alt_handle);
        self.pitch = Some(pitch_handle);

        Ok(())
    } 
}

impl Actor for Streamer {
    type Context = Context<Self>;
}

pub struct StreamValues;

impl actix::Message for StreamValues {
    type Result = Vec<Box<StreamUpdate>>;
}

impl Handler<StreamValues> for Streamer {
    type Result = MessageResult<StreamValues>;

    fn handle(&mut self, _: StreamValues, _: &mut Context<Self>) -> Self::Result {
        let mut results: Vec<Box<StreamUpdate>> = Vec::new();

        let update_result = self.stream_client.recv_update();
        if let Ok(update) = update_result {
            // TODO: Probably abstract this out soon, will do for now
                // Universal time stream
                if let Some(handle) = self.ut {
                match update.get_result(&handle) {
                    Ok(ut) => { results.push(Box::new(StreamUpdate::UniversalTime(ut))); },
                    Err(_) => {}
                }
            }

            // Altitude stream
            if let Some(handle) = self.alt {
                match update.get_result(&handle) {
                    Ok(altitude) => { results.push(Box::new(StreamUpdate::Altitude(altitude))); },
                    Err(_) => {}
                }
            }

            // Pitch stream
            if let Some(handle) = self.pitch {
                match update.get_result(&handle) {
                    Ok(pitch) => { results.push(Box::new(StreamUpdate::Pitch(pitch))); },
                    Err(_) => {}
                }
            }
        }

        MessageResult(results)
    }
}
