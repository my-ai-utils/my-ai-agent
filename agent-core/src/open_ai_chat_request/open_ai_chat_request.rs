use std::collections::HashMap;

use super::*;
pub struct OpenAiChatRequest {
    pub sys_prompt_id: String,
    pub sys_prompt_version: String,
    pub parallel_tool_calls: bool,
    pub with_tool_calls: Vec<AllowedMcpToolCallsJsonModel>,
    pub history: Vec<HistoryItem>,
    pub request_metadata: HashMap<String, String>,
    pub prompt_cache_key: Option<String>,
}

impl OpenAiChatRequest {
    pub fn new(sys_prompt_id: String, sys_prompt_version: String) -> Self {
        Self {
            sys_prompt_id,
            sys_prompt_version,
            parallel_tool_calls: true,
            with_tool_calls: Default::default(),
            request_metadata: Default::default(),
            history: Default::default(),
            prompt_cache_key: Default::default(),
        }
    }

    pub fn set_prompt_cache_key(mut self, value: String) -> Self {
        self.prompt_cache_key = Some(value);
        self
    }

    pub fn add_user_history_text(mut self, text: String) -> Self {
        self.history.push(HistoryItem {
            role: HistoryItemRole::User,
            text,
        });
        self
    }

    pub fn add_assistant_history_text(mut self, text: String) -> Self {
        self.history.push(HistoryItem {
            role: HistoryItemRole::Assistant,
            text,
        });
        self
    }

    pub fn with_request_metadata(mut self, key: String, value: String) -> Self {
        self.request_metadata.insert(key, value);
        self
    }

    pub fn with_tool_call(
        mut self,
        server_label: String,
        server_url: String,
        server_description: String,
        allowed_tools: Vec<String>,
    ) -> Self {
        self.with_tool_calls.push(AllowedMcpToolCallsJsonModel {
            r#type: "mcp".to_string(),
            server_label,
            server_url,
            server_description,
            allowed_tools,
            require_approval: "never".to_string(),
        });
        self
    }

    pub fn generate_request_model(&self) -> String {
        my_json::json_writer::JsonObjectWriter::new()
            .write_json_object("prompt", |obj| {
                obj.write("id", self.sys_prompt_id.as_str())
                    .write("version", self.sys_prompt_version.as_str())
            })
            .write("parallel_tool_calls", true)
            .write("stream", true)
            .write_json_array("input", |mut arr| {
                for itm in self.history.iter() {
                    arr = arr.write_json_object(|obj| {
                        obj.write("role", itm.role.as_str())
                            .write_json_array("content", |arr| {
                                arr.write_json_object(|obj| {
                                    obj.write("type", "input_text")
                                        .write("text", itm.text.as_str())
                                })
                            })
                    });
                }
                arr
            })
            .write_json_object("metadata", |mut obj| {
                for (key, value) in self.request_metadata.iter() {
                    obj = obj.write(key, value);
                }
                obj
            })
            .write_json_array("tools", |mut tool_array| {
                for tool in self.with_tool_calls.iter() {
                    tool_array = tool_array.write_json_object(|obj| {
                        obj.write("type", tool.r#type.as_str())
                            .write("server_label", tool.server_label.as_str())
                            .write("server_url", tool.server_url.as_str())
                            .write("server_description", tool.server_description.as_str())
                            .write_iter("allowed_tools", tool.allowed_tools.iter())
                            .write("require_approval", tool.require_approval.as_str())
                    })
                }

                tool_array
            })
            .write_if_some("prompt_cache_key", self.prompt_cache_key.as_ref())
            .write("store", true)
            .build()
    }

    pub async fn execute_request(&self, open_ai_key: String) -> OpenAiChatStreamedResponse {
        OpenAiChatStreamedResponse::new(self.generate_request_model(), open_ai_key)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_array() {
        let req = OpenAiChatRequest::new("sys_id".into(), "15".into())
            .with_request_metadata("key".into(), "value".into())
            .add_user_history_text("User text".into())
            .with_tool_call(
                "label".into(),
                "https://url".into(),
                "Description".into(),
                vec![],
            );

        println!("{}", req.generate_request_model());
    }
}
