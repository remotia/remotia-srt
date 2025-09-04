use std::time::Instant;

use async_trait::async_trait;
use bytes::Bytes;
use remotia::traits::FrameProcessor;

use futures::TryStreamExt;
use srt_tokio::SrtSocket;

use crate::SRTTransmission;

pub struct SRTFrameReceiver {
    socket: SrtSocket,
}

impl SRTFrameReceiver {
    pub fn from_socket(socket: SrtSocket) -> Self {
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
        log::debug!("Receiving binarized frame DTO...");

        let receive_result = self.receive_binarized().await;

        if let Err(error) = receive_result {
            log::debug!("Error on receive: {:?}", error);
            frame_data.report_receive_error(error);
            return Some(frame_data);
        }

        // TODO: Implement an error handling mechanism
        let (transmission_instant, binarized_obj) = receive_result
            .expect("Unexpected error")
            .expect("Unexpected None receive result");

        log::debug!(
            "Received a packet of {} bytes at instant {:?}",
            binarized_obj.len(),
            transmission_instant
        );

        frame_data.report_reception_delay(transmission_instant.elapsed().as_millis());
        frame_data.deserialize_packet(&binarized_obj);

        Some(frame_data)
    }
}
