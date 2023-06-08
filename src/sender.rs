use std::time::{Duration, Instant};

use async_trait::async_trait;

use bytes::BytesMut;
use futures::SinkExt;

use log::info;
use remotia::traits::{BorrowableFrameProperties, FrameProcessor};
use srt_tokio::{
    options::{ByteCount, PacketSize},
    SrtSocket,
};

pub struct SRTFrameSender<K> {
    buffer_key: K,
    socket: SrtSocket,
}

impl<K> SRTFrameSender<K> {
    pub async fn new(buffer_key: K, port: u16, latency: Duration) -> Self {
        info!("Listening...");
        let socket = SrtSocket::builder()
            .set(|options| {
                options.sender.buffer_size = ByteCount(1024 * 1024 * 8); // 32 MB for internal buffering
                options.sender.max_payload_size = PacketSize(1024 * 1024 * 8);
                options.connect.timeout = Duration::from_secs(30);
            })
            .latency(latency)
            .listen_on(port)
            .await
            .unwrap();

        info!("Connected");

        Self { buffer_key, socket }
    }
}

#[async_trait]
impl<F, K> FrameProcessor<F> for SRTFrameSender<K>
where
    K: Send,
    F: BorrowableFrameProperties<K, BytesMut> + Send + 'static,
{
    async fn process(&mut self, frame_data: F) -> Option<F> {
        let buffer = frame_data
            .get_ref(&self.buffer_key)
            .unwrap()
            .clone()
            .freeze();

        self.socket.send((Instant::now(), buffer)).await.unwrap();

        Some(frame_data)
    }
}
