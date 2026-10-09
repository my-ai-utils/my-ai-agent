use std::sync::Arc;

use flurl::FlUrlBodyReader;
use tokio::sync::Mutex;

pub enum OpenAiNetworkStream {
    Http(FlUrlBodyReader),
    Mock(Arc<Mutex<Vec<String>>>),
}

impl OpenAiNetworkStream {
    pub async fn get_next_chunk(&mut self) -> Result<Option<Vec<u8>>, String> {
        match self {
            OpenAiNetworkStream::Http(body_reader) => body_reader
                .next_item()
                .await
                .map(|chunk| chunk.map(|chunk| chunk.to_vec()))
                .map_err(|err| format!("{:?}", err)),
            OpenAiNetworkStream::Mock(items) => {
                let mut items = items.lock().await;
                if items.len() == 0 {
                    return Ok(None);
                }

                #[cfg(not(test))]
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;

                let result = items.remove(0);

                //       println!("Remains {}. Mock chunk: {}", items.len(), result);

                Ok(Some(result.into_bytes()))
            }
        }
    }
}
