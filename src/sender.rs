use std::time::Instant;

use async_trait::async_trait;

use bytes::BytesMut;
use futures::SinkExt;

use log::info;
use remotia::traits::{BorrowFrameProperties, FrameProcessor};
use srt_tokio::SrtSocket;

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
    F: BorrowFrameProperties<K, BytesMut> + Send + 'static,
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
