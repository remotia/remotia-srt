use std::time::Duration;

use async_trait::async_trait;
use bytes::BytesMut;
use remotia::traits::{BorrowableFrameProperties, FrameProcessor};

use futures::TryStreamExt;
use log::{debug, info};
use srt_tokio::SrtSocket;

pub struct SRTFrameReceiver<K> {
    buffer_key: K,
    socket: SrtSocket,
}

impl<K> SRTFrameReceiver<K> {
    pub async fn new(buffer_key: K, server_address: &str, latency: Duration) -> Self {
        info!("Connecting...");
        let socket = SrtSocket::builder()
            .set(|options| {
                options.connect.timeout = Duration::from_secs(30);
            })
            .latency(latency)
            .call(server_address, None)
            .await
            .unwrap();

        info!("Connected");

        Self { buffer_key, socket }
    }
}

#[async_trait]
impl<F, K> FrameProcessor<F> for SRTFrameReceiver<K>
where
    K: Send,
    F: BorrowableFrameProperties<K, BytesMut> + Send + 'static,
{
    async fn process(&mut self, mut frame_data: F) -> Option<F> {
        debug!("Receiving binarized frame DTO...");

        let (_, binarized_obj) = match self.socket.try_next().await {
            Ok(result) => result.unwrap(),
            Err(err) => {
                log::error!("Reception error: {}", err);
                return None;
            }
        };

        frame_data
            .get_mut_ref(&self.buffer_key)
            .unwrap()
            .copy_from_slice(&binarized_obj);

        Some(frame_data)
    }
}
