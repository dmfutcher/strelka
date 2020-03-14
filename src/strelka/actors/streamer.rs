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
        let dir_handle = client.mk_call(&flight.get_direction().to_stream())?;

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
                    Ok(raw_vessel_dir) => {
                        // Compute the pitch - the angle between the vessels direction and
                        // the direction in the horizon plane



                        if let Ok(vessel) = self.client.mk_call(&space_center::get_active_vessel()) {
                            if let Ok(bodies) = self.client.mk_call(&space_center::get_bodies()) {
                                if let Some(earth) = bodies.get("Kerbin") {
                                    if let Ok(ref_plane) = self.client.mk_call(&earth.get_reference_frame()) {
                                        if let Ok(flight) = self.client.mk_call(&vessel.flight(&ref_plane)) {
                                            if let Ok(direction) = self.client.mk_call(&flight.get_direction()) {
                                                let vessel_dir = Vector3::from(direction);
                                                let horizon = Vector3::new(0.0, vessel_dir.y(), vessel_dir.z());
                                    
                                                let pitch_magnitude = vessel_dir.angle_between(&horizon);
                                                println!("Pitch: {:?}!", pitch_magnitude);
                                            }
                                            
                                            
                                        }
                                    }

                                }
                                   
                            }
                        }
                        // results.push(Box::new(StreamUpdate::Pitch(pitch))); 



                    },
                    Err(_) => {}
                }
            }
        }

        MessageResult(results)
    }
}
