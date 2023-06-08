use std::time::{Duration, Instant};

use async_trait::async_trait;

use bytes::{BytesMut, Bytes};
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
    pub async fn new(buffer_key: K, socket: SrtSocket) -> Self {
        info!("Listening...");
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
