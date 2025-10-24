#[derive(Clone)]
pub struct OpenAiChunk(String);

impl OpenAiChunk {
    pub fn parse(src: &str) -> Vec<Self> {
        let mut result = Vec::new();
        let mut new_line = String::new();
        for itm in src.split('\n') {
            if itm.starts_with("event:") {
                if new_line.len() > 0 {
                    result.push(OpenAiChunk(std::mem::take(&mut new_line)));
                }
            } else {
                new_line.push_str(itm);
            }
        }

        result
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn get_json(&self) -> &[u8] {
        &self.0.as_bytes()[6..]
    }

    pub fn try_get_text_delta(&self) -> Option<String> {
        let Ok(value) = my_json::j_path::get_value(self.get_json(), "type") else {
            return None;
        };

        let Some(value) = value else {
            return None;
        };

        let Some(result) = value.as_str() else {
            return None;
        };

        if result.as_str() != "response.output_text.delta" {
            return None;
        }

        let Ok(value) = my_json::j_path::get_value(self.get_json(), "delta") else {
            return None;
        };

        let Some(value) = value else {
            return None;
        };

        let Some(result) = value.as_str() else {
            return None;
        };

        Some(result.as_str().to_string())
    }
}
