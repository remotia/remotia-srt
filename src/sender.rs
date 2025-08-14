use std::time::{Duration, Instant};

use async_trait::async_trait;

use futures::SinkExt;

use remotia::traits::FrameProcessor;
use srt_tokio::{
    options::{ByteCount, PacketSize},
    SrtSocket,
};

use crate::SRTTransmission;

pub struct SRTFrameSender {
    socket: SrtSocket,
}

impl SRTFrameSender {
    pub async fn new(port: u16, latency: Duration) -> Self {
        log::info!("Listening...");
        let socket = SrtSocket::builder()
            .set(|options| {
                options.sender.buffer_size = ByteCount(1024 * 1024 * 32); // 32 MB for internal buffering
                options.sender.max_payload_size = PacketSize(1024 * 1024 * 32);
            })
            .latency(latency)
            .listen_on(port)
            .await
            .unwrap();

        log::info!("Connected");

        Self { socket }
    }
}

#[async_trait]
impl<F> FrameProcessor<F> for SRTFrameSender 
where
    F: SRTTransmission + Send + 'static,
{
    async fn process(&mut self, frame_data: F) -> Option<F> {
        let binarized_obj = frame_data.serialize_packet();
        self.socket
            .send((Instant::now(), binarized_obj))
            .await
            .unwrap();
        Some(frame_data)
    }
}
