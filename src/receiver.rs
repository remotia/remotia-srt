use async_trait::async_trait;
use remotia::{traits::{FrameProcessor, BorrowMutFrameProperties}, buffers::{BytesMut, BufMut}};

use tokio_stream::StreamExt;
use srt_tokio::SrtSocket;

pub struct SRTFrameReceiver<K> {
    buffer_key: K,
    socket: SrtSocket,
}

impl<K> SRTFrameReceiver<K> {
    pub fn new(buffer_key: K, socket: SrtSocket) -> Self {
        Self { buffer_key, socket }
    }
}


#[async_trait]
impl<F, K> FrameProcessor<F> for SRTFrameReceiver<K>
where
    K: Send,
    F: BorrowMutFrameProperties<K, BytesMut> + Send + 'static,
{
    async fn process(&mut self, mut frame_data: F) -> Option<F> {
        log::trace!("Receiving...");
        let (_, received_buffer) = match self.socket.try_next().await {
            Ok(result) => {
                let (instant, buffer) = result.unwrap();
                log::trace!("Received buffer: {}", buffer.len());
                (instant, buffer)
            },
            Err(err) => {
                log::error!("Reception error: {}", err);
                return None;
            }
        };

        log::trace!("Received {} bytes...", received_buffer.len());
        frame_data
            .get_mut_ref(&self.buffer_key)
            .unwrap()
            .put(received_buffer);

        Some(frame_data)
    }
}
