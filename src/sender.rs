use std::time::Instant;

use async_trait::async_trait;

use futures::SinkExt;

use remotia::traits::FrameProcessor;
use srt_tokio::SrtSocket;

use crate::SRTTransmission;

pub struct SRTFrameSender {
    socket: SrtSocket,
}

impl SRTFrameSender {
    pub fn from_socket(socket: SrtSocket) -> Self {
        Self { socket }
    }
}

#[async_trait]
impl<F> FrameProcessor<F> for SRTFrameSender
where
    F: SRTTransmission + Send + 'static,
{
    async fn process(&mut self, frame_data: F) -> Option<F> {
        log::debug!("Sending binarized frame DTO...");

        let binarized_obj = frame_data.serialize_packet();
        let transmission_instant = Instant::now();

        log::debug!(
            "Sending a packet of {} bytes at instant {:?}",
            binarized_obj.len(),
            transmission_instant
        );

        self.socket
            .send((transmission_instant, binarized_obj))
            .await
            .unwrap();

        Some(frame_data)
    }
}
