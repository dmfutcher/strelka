use actix::prelude::*;
use krpc_mars::{RPCClient, StreamClient};

use crate::krpc::space_center;
use crate::strelka::streams::{StreamUpdate, Vector3};

/// Streamer periodically gets polls and fetches values from KRPC streams
pub struct Streamer {
    client: RPCClient,
    stream_client: StreamClient,
    
    ut: Option<krpc_mars::StreamHandle<f64>>,
    alt: Option<krpc_mars::StreamHandle<f64>>,
    direction: Option<krpc_mars::StreamHandle<(f64, f64, f64)>>,
}

impl Streamer {
    pub fn new(krpc_client: RPCClient, stream_client: StreamClient) -> Self {
        let mut streamer = Streamer{ 
            client:krpc_client.clone(),
            stream_client,
            ut: None, 
            alt: None,
            direction: None
        };

        streamer.initialise_streams(); // TODO: This is a Result
        streamer
    }

    fn initialise_streams(&mut self) -> Result<(), failure::Error> {
        // TODO: Maybe refactor all these calls. Seems very likely to be repeated all over the codebase without some thought.
        let vessel = self.client.mk_call(&space_center::get_active_vessel())?;
        let bodies = self.client.mk_call(&space_center::get_bodies())?;
        let kerbin_ref = self.client.mk_call(&bodies.get("Kerbin").unwrap().get_reference_frame())?;
        let flight = self.client.mk_call(&vessel.flight(&kerbin_ref))?;

        let ut_handle = self.client.mk_call(&space_center::get_ut().to_stream())?;
        let alt_handle = self.client.mk_call(&flight.get_mean_altitude().to_stream())?;
        let dir_handle = self.client.mk_call(&flight.get_direction().to_stream())?;

        self.ut = Some(ut_handle);
        self.alt = Some(alt_handle);
        self.direction = Some(dir_handle);

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

            // Direction-based streams
            if let Some(handle) = self.direction {
                match update.get_result(&handle) {
                    Ok(raw_dir) => {
                        // Compute the pitch - the angle between the vessels direction and
                        // the direction in the horizon plane
                        let vessel_dir = Vector3::from(raw_dir);
                        let horizon = Vector3::new(0.0, vessel_dir.y(), vessel_dir.z());
                        let angle = vessel_dir.angle_between(&horizon);
                        let pitch = if vessel_dir.x() < 0.0 { -angle } else { angle };
                        results.push(Box::new(StreamUpdate::Pitch(pitch))); 
                    },
                    Err(_) => {}
                }
            }
        }

        MessageResult(results)
    }
}
