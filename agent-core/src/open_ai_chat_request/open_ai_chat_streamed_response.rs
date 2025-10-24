use flurl::{FlResponseAsStream, FlUrl, body::FlUrlBody};

use super::*;

pub struct OpenAiChatStreamedResponse {
    request_body: String,
    open_ai_key: String,
    fl_url: Option<FlResponseAsStream>,
    last_chunks: Vec<OpenAiChunk>,
    full_response: Vec<u8>,
}

impl OpenAiChatStreamedResponse {
    pub fn new(request_body: String, open_ai_key: String) -> Self {
        Self {
            request_body,
            open_ai_key,
            fl_url: Default::default(),
            last_chunks: Default::default(),
            full_response: Vec::new(),
        }
    }

    pub async fn get_next(&mut self) -> Result<Option<Vec<OpenAiChunk>>, String> {
        if self.fl_url.is_none() {
            let fl_url_stream = create_fl_url(&self.open_ai_key, &self.request_body).await?;
            self.fl_url = Some(fl_url_stream);
        }

        let fl_url = self.fl_url.as_mut().unwrap();

        let chunk = fl_url
            .get_next_chunk()
            .await
            .map_err(|err| format!("{:?}", err))?;

        let chunk = match chunk {
            Some(chunk) => chunk,
            None => return Ok(None),
        };

        self.full_response.extend_from_slice(chunk.as_slice());

        let chunk = match std::str::from_utf8(self.full_response.as_slice()) {
            Ok(value) => value,
            Err(_) => return Ok(Some(vec![])),
        };

        let response_chunks = OpenAiChunk::parse(chunk);

        let mut to_yield = Vec::new();

        for (index, itm) in response_chunks.iter().enumerate() {
            if let Some(last_itm) = self.last_chunks.get(index) {
                if last_itm.as_str() != itm.as_str() {
                    to_yield.push(itm.clone());
                }
            } else {
                to_yield.push(itm.clone());
            }
        }

        self.last_chunks = response_chunks;

        Ok(Some(to_yield))
    }
}

async fn create_fl_url(open_ai_key: &str, body: &str) -> Result<FlResponseAsStream, String> {
    let body = FlUrlBody::Json(body.as_bytes().to_vec());
    let mut response = FlUrl::new("https://api.openai.com/v1/responses")
        .with_header("Authorization", format!("Bearer {}", open_ai_key))
        .post(body)
        .await
        .map_err(|err| format!("{:?}", err))?;

    let status_code = response.get_status_code();

    if status_code != 200 {
        let body = response
            .get_body_as_str()
            .await
            .map_err(|err| format!("{:?}", err))?;
        return Err(format!("Status code: {}. Body: {}", status_code, body));
    }

    Ok(response.get_body_as_stream())
}
