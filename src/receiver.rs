use std::time::{Duration, Instant};

use async_trait::async_trait;
use bytes::Bytes;
use remotia::traits::FrameProcessor;

use futures::TryStreamExt;
use log::{debug, info};
use srt_tokio::SrtSocket;

use crate::SRTTransmission;

pub struct SRTFrameReceiver {
    socket: SrtSocket,
}

impl SRTFrameReceiver {
    pub async fn new(server_address: &str, latency: Duration) -> Self {
        info!("Connecting...");
        let socket = SrtSocket::builder()
            .latency(latency)
            .call(server_address, None)
            .await
            .unwrap();

        info!("Connected");

        Self { socket }
    }

    async fn receive_binarized(&mut self) -> Result<Option<(Instant, Bytes)>, std::io::Error> {
        self.socket.try_next().await
    }
}

#[async_trait]
impl<F> FrameProcessor<F> for SRTFrameReceiver
where
    F: SRTTransmission + Send + 'static,
{
    async fn process(&mut self, mut frame_data: F) -> Option<F> {
        debug!("Receiving binarized frame DTO...");

        let receive_result = self.receive_binarized().await;

        if let Err(error) = receive_result {
            frame_data.report_receive_error(error);
            return Some(frame_data);
        }

        // TODO: Implement an error handling mechanism
        let (transmission_instant, binarized_obj) = receive_result
            .expect("Unexpected error")
            .expect("Unexpected None receive result");

        frame_data.report_reception_delay(transmission_instant.elapsed().as_millis());
        frame_data.deserialize_packet(&binarized_obj);

        Some(frame_data)
    }
}
