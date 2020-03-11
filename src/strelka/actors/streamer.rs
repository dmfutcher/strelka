use actix::prelude::*;
use krpc_mars::{RPCClient, StreamClient};
use std::collections::HashMap;
use std::str::FromStr;

use crate::krpc::space_center;
use crate::strelka::streams::StreamUpdate;

enum Handle {
    StreamHandleF64(krpc_mars::StreamHandle<f64>),
}

impl Handle {
    fn new_f64(h: krpc_mars::StreamHandle<f64>) -> Self {
        Handle::StreamHandleF64(h)
    }
}

/// Streamer periodically gets polls and fetches values from KRPC streams
pub struct Streamer {
    stream_client: StreamClient,
    handles: HashMap<String, Handle>,
}

impl Streamer {
    pub fn new(krpc_client: RPCClient, stream_client: StreamClient) -> Self {
        let mut streamer = Streamer{ stream_client, handles: HashMap::new() };
        streamer.initialise_streams(krpc_client); // TODO: This is a Result

        streamer
    }

    fn initialise_streams(&mut self, client: RPCClient) -> Result<(), failure::Error> {
        let ut_stream_handle = client.mk_call(&space_center::get_ut().to_stream())?;
        self.handles.insert("ut".to_owned(), Handle::new_f64(ut_stream_handle));

        let vessel = client.mk_call(&space_center::get_active_vessel())?;
        let reference_frame = client.mk_call(&vessel.get_reference_frame())?;
        let flight = client.mk_call(&vessel.flight(&reference_frame))?;

        let alt_stream_handle = client.mk_call(&flight.get_mean_altitude().to_stream())?;
        self.handles.insert("Altitude".to_owned(), Handle::new_f64(alt_stream_handle));

        Ok(())
    } 
}

impl Actor for Streamer {
    type Context = Context<Self>;
}

pub struct StreamValues;

impl actix::Message for StreamValues {
    type Result = Vec<StreamUpdate>;
}

impl Handler<StreamValues> for Streamer {
    type Result = MessageResult<StreamValues>;

    fn handle(&mut self, _: StreamValues, _: &mut Context<Self>) -> Self::Result {
        let mut results = Vec::new();
        let update_result = self.stream_client.recv_update();

        if let Ok(update) = update_result {
            for (name, handle) in &self.handles {
                let stream_handle = match *handle {
                    Handle::StreamHandleF64(v) => v
                };

                let stream_result = update.get_result(&stream_handle);
                if let Err(_) = stream_result {
                    continue;
                }
                let stream_value = stream_result.unwrap();

                let update_val_create = StreamUpdate::from_str(name);
                if let Ok(mut update_value) = update_val_create {

                    // TODO: This is utter madness, need to find an alternative
                    //       would a tuple of (ValueName, ValueValue) make sense?
                    match update_value {
                        StreamUpdate::Altitude(ref mut wrapped) => {
                            *wrapped = stream_value;
                        },
                        StreamUpdate::UniversalTime(ref mut wrapped) => {
                            *wrapped = stream_value;
                        }
                    };

                    results.push(update_value);
                }
            }    
        }

        MessageResult(results)
    }
}
