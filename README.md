# my-ai-agent

Rust crate for talking to OpenAI-compatible chat-completion APIs (OpenAI, Nebius, Z.ai, Fireworks, Cerebras) and building agentic loops with tool calls — local Rust functions or remote tool servers, with optional streaming.

The crate re-exports three pieces:

- `my_ai_agent::models` — model enum + per-vendor settings (always available).
- `my_ai_agent::macros` — `#[derive(ApplyJsonSchema)]` for tool-call params (always available).
- everything from `agent-core` flat at the crate root — request builder, agent loop, streaming, traits (only with feature `agent`).

## Cargo

```toml
[dependencies]
my-ai-agent = { git = "...", features = ["agent"] }   # full agent + builder
# or:
my-ai-agent = { git = "..." }                         # models + macros only
```

The `agent` feature pulls in `agent-core` and `my-json`. Without it you only get the model enums and the schema-derive macro (useful for crates that just need the JSON-schema generation).

## Quick map of the API

| You want to… | Use |
|---|---|
| Pick a model + vendor settings | [`models::LlmModel`](#1-pick-a-model) |
| Pick HTTP endpoint / API key | [`AutoGenSettings::create_as_*`](#2-pick-an-endpoint) |
| Build a chat request manually | [`LlmRequestBuilder`](#3-llmrequestbuilder) |
| Run an agent loop with tools | [`MyAutoGen`](#4-myautogen-agent-loop) |
| Define a tool's params struct | [`#[derive(ApplyJsonSchema)]`](#5-tool-params-with-derive) |
| Implement a local tool function | [`ToolFunction` + `ToolDefinition`](#6-local-tool-functions) |
| Delegate tools to a remote server | [`RemoteToolFunctions`](#7-remote-tool-functions) |
| Stream tokens incrementally | [`execute_request_as_stream`](#8-streaming) |
| Use the OpenAI Responses API (MCP) | [`OpenAiChatRequest`](#9-openaichatrequest-responses-api) |

---

## 1. Pick a model

`LlmModel` is a `Copy` enum. Each variant carries vendor-specific settings.

```rust
use my_ai_agent::models::*;

let model = LlmModel::Gpt5(Gpt5Settings {
    reasoning_effort: Some(Gpt5ReasoningEffort::Low),
    verbosity: Some(Gpt5VerbosityEffort::Medium),
});

let model = LlmModel::Gpt4o(Gpt4Settings {
    temperature: Some(0.7),
    top_p: None,
    n: None,
    presence_penalty: None,
    frequency_penalty: None,
});

let model = LlmModel::Qwen3_30bA3b(QwenSettings { think: false });
let model = LlmModel::ZaiGlm4_7(ZaiSettings { temperature: Some(0.6) });
```

Available variants: `Gpt4o`, `Gpt4oMini`, `Gpt5`, `Gpt5Mini`, `Gpt5Nano`, `Qwen3_30bA3b`, `ZaiGlm4_5`, `ZaiGlm4_5Air`, `ZaiGlm4_5X`, `ZaiGlm4_6`, `ZaiGlm4_7`, `FireworksZaiGlm4_6`, `FireworksZaiGlm4_7`, `CerebrasZaiGlm4_6`, `CerebrasZaiGlm4_7`.

Settings are vendor-aware: GPT-4 fields go to GPT-4 requests, GPT-5 fields go to GPT-5 requests, etc. — irrelevant fields are skipped.

For Qwen the `think: false` flag automatically prepends `/no_think\n` to the system prompt.

## 2. Pick an endpoint

`AutoGenSettings` describes where the request goes. Use the matching constructor for your vendor:

```rust
use my_ai_agent::my_auto_gen::AutoGenSettings;

let settings = AutoGenSettings::create_as_open_ai(Some(api_key), false);
let settings = AutoGenSettings::create_as_nebius(Some(api_key), false);
let settings = AutoGenSettings::create_as_zai(Some(api_key), false);
let settings = AutoGenSettings::create_as_fireworks(Some(api_key), false);
let settings = AutoGenSettings::create_as_cerebras(Some(api_key), false);
```

Second argument is `do_not_reuse_connection` — set it to `true` if the vendor closes idle connections aggressively.

There is also `AutoGenSettings::Mock(Vec<String>)` which feeds prebaked SSE chunks into the streaming pipeline; useful for tests.

## 3. `LlmRequestBuilder`

Stateful builder for one chat (system prompt + history + tools). All methods are `async` because the inner state is behind a `tokio::sync::Mutex`.

```rust
use my_ai_agent::LlmRequestBuilder;

let rb = LlmRequestBuilder::new_with_system_prompt(
    "You are a helpful assistant.",
    model,
);

rb.add_user_message("Hello!").await;
rb.add_assistant_message("Hi, how can I help?".to_string()).await;
rb.add_user_message("What's 2+2?").await;
```

Other constructors:

```rust
LlmRequestBuilder::new(model);                                // no system prompt
LlmRequestBuilder::from_history(sys_prompt, history, summary, model);
```

Useful methods:

- `add_system_message`, `add_user_message`, `add_assistant_message`
- `add_tools(serde_json::Value)` / `add_tools_call_description(serde_json::Value)`
- `remove_tool_calls()` — strip assistant tool calls + tool responses from history
- `get_model() -> OpenAiRequestModel` — final serialisable request
- `modify(|inner| { … })` — direct access to set `max_tokens`, `temperature`, `top_p`, GPT-5 reasoning, etc.
- `get_tech_log()` — drain the request/response/chunk debug log written during `execute`

You usually don't need to call `get_model()` yourself — `MyAutoGen::execute` does it.

## 4. `MyAutoGen` (agent loop)

Drives a chat→tool-call→tool-response→chat loop until the model returns a plain assistant message.

```rust
use std::sync::Arc;
use my_ai_agent::my_auto_gen::*;
use my_ai_agent::LlmRequestBuilder;

let auto_gen = MyAutoGen::new(logger);          // logger: Arc<dyn rust_extensions::Logger + Send + Sync>

// register one or more tools (see §6) before calling execute:
auto_gen.register_function(Arc::new(MyTool { /* … */ })).await;

let rb = LlmRequestBuilder::new_with_system_prompt("…", model);
rb.add_user_message("do the thing").await;

let tool_results: Vec<ToolCallsResult> =
    auto_gen.execute(&settings, &rb, "ctx-passed-to-tools").await?;

// final assistant text is now the last message in `rb`:
let last = rb.get(|inner| inner.get_last_message().clone()).await;
println!("{}", last.content.unwrap());
```

`execute` returns the list of every tool call that fired during the loop (`fn_name`, `request_data`, `result_data`). The final assistant message is appended to the builder's history.

You can register **either** local functions (`register_function`, can be called many times) **or** a single remote handler (`register_remote_tool_functions`). Mixing the two panics.

## 5. Tool params with `#[derive(...)]`

Tool parameters are described by a struct that derives `ApplyJsonSchema`. This auto-generates:

- `JsonTypeDescription` impl → JSON schema for the OpenAI `tools` field
- `DeserializeToolCallParam` impl → parser from the JSON the model sends back
- An associated `Self::get_json_schema(output: bool)` helper

```rust
use my_ai_agent::macros::ApplyJsonSchema;
use rust_extensions::StrOrString;

#[derive(ApplyJsonSchema)]
pub struct FilterShowrooms {
    #[property(description: "city")]
    pub city: String,

    #[property(description: "service")]
    pub service: Option<String>,

    #[property(enum: ["NEW", "CPO"], description: "Vehicle condition")]
    pub condition: Option<String>,

    // dynamic enum: name a function returning Option<Vec<StrOrString<'static>>>
    #[property(enum: "fetch_brands_enum", description: "Brand")]
    pub brand: String,

    #[property(description: "Min year", default: "2020")]
    pub min_year: Option<i64>,

    #[property(enum: ["A", "B"], description: "Tags")]
    pub tags: Vec<String>,
}

async fn fetch_brands_enum() -> Option<Vec<StrOrString<'static>>> {
    Some(vec!["BMW".into(), "Audi".into()])
}
```

Supported field types: primitive numbers, `String`, `bool`, `Option<T>`, `Vec<T>`, and other `ApplyJsonSchema` structs (nested objects). `Option<T>` becomes nullable (and is excluded from `required`).

`#[property(...)]` attributes:

| key | meaning |
|---|---|
| `description: "…"` | required, becomes the JSON schema `description` |
| `enum: ["a", "b"]` | static enum values |
| `enum: "fn_name"` | dynamic enum: async fn returning `Option<Vec<StrOrString<'static>>>` |
| `default: "value"` | becomes the JSON schema `default` |

## 6. Local tool functions

Implement two traits on a struct:

- `ToolDefinition` — gives the model-facing name + description (constants).
- `ToolFunction<ParamType>` — async callback the agent invokes.

```rust
use std::sync::Arc;
use async_trait::async_trait;
use my_ai_agent::{ToolDefinition, my_auto_gen::ToolFunction};

pub struct FilterShowroomsTool;

impl ToolDefinition for FilterShowroomsTool {
    const FUNC_NAME: &'static str = "filter_showrooms";
    const DESCRIPTION: &'static str =
        "Filters showrooms by city, service and brand.";
}

#[async_trait]
impl ToolFunction<FilterShowrooms> for FilterShowroomsTool {
    async fn callback(
        &self,
        params: FilterShowrooms,
        ctx: &str,            // the `ctx` string passed to MyAutoGen::execute
    ) -> Result<String, String> {
        // do work …
        Ok(serde_json::json!({ "found": 3 }).to_string())
    }
}

auto_gen.register_function(Arc::new(FilterShowroomsTool)).await;
```

The returned `String` is fed back into the chat as the `tool` role content.

## 7. Remote tool functions

When tools live in another process (an MCP server, gateway, etc.), implement `RemoteToolFunctions` instead — you supply both the schema (already-serialised JSON) and the dispatcher.

```rust
use async_trait::async_trait;
use my_ai_agent::my_auto_gen::RemoteToolFunctions;

pub struct MyMcpGateway { /* … */ }

#[async_trait]
impl RemoteToolFunctions for MyMcpGateway {
    async fn get_tools_description(&self) -> String {
        // return a JSON array of {type:"function", function:{...}} objects
        r#"[{"type":"function","function":{"name":"…","parameters":{…}}}]"#.into()
    }

    async fn tool_call(
        &self,
        fn_name: &str,
        params: &str,    // raw JSON the model produced
        ctx: &str,
    ) -> Result<String, String> { /* … */ Ok("…".into()) }
}

auto_gen.register_remote_tool_functions(Arc::new(MyMcpGateway { /* … */ })).await;
```

## 8. Streaming

Same setup as `execute`, but you get a stream of chunks back. The loop still runs end-to-end (text → tool calls → text), and you observe each chunk as it happens.

```rust
use my_ai_agent::my_auto_gen::OpenAiStreamChunk;
use std::sync::Arc;

let rb = Arc::new(LlmRequestBuilder::new_with_system_prompt("…", model));
rb.add_user_message("…").await;

let mut stream = auto_gen
    .execute_request_as_stream(&settings, rb.clone(), "ctx")
    .await?;

while let Some(chunk) = stream.get_next().await? {
    match chunk {
        OpenAiStreamChunk::Text(text) => print!("{text}"),
        OpenAiStreamChunk::ToolCalls(calls) => {
            for c in calls {
                println!("[{}] called {} with {} → {}",
                    c.id, c.fn_name, c.params, c.result);
            }
        }
    }
}
```

`OpenAiResponseStream::get_next` returns `Ok(None)` when the loop has finished; `Err(...)` on a hard failure. Tool calls inside the stream are fully resolved (params + result) by the time you see them.

## 9. `OpenAiChatRequest` (Responses API)

Separate, smaller helper for the OpenAI Responses endpoint (`/v1/responses`) that targets a stored prompt template and optional MCP tool servers. Independent of `MyAutoGen`.

```rust
use my_ai_agent::open_ai_chat_request::*;

let req = OpenAiChatRequest::new("sys_prompt_id".into(), "15".into())
    .with_request_metadata("user_id".into(), "u-1".into())
    .add_user_history_text("Hello".into())
    .add_assistant_history_text("Hi!".into())
    .with_tool_call(
        "my_mcp_server".into(),
        "https://my.mcp/server".into(),
        "Server description".into(),
        vec!["search".into(), "fetch".into()],
    )
    .set_prompt_cache_key("cache-key-123".into());

let mut response = req.execute_request(open_ai_api_key).await;
while let Some(chunks) = response.get_next().await? {
    for chunk in chunks {
        if let Some(delta) = chunk.try_get_text_delta() {
            print!("{delta}");
        }
    }
}
```

---

## Roles & message model

Roles are constants in `models`:

```rust
my_ai_agent::models::SYSTEM_ROLE     // "system"
my_ai_agent::models::USER_ROLE       // "user"
my_ai_agent::models::ASSISTANT_ROLE  // "assistant"
my_ai_agent::models::TOOL_ROLE       // "tool"
```

The trait `MessageRole` (`is_system`, `is_user`, `is_assistant`, `is_tool`) is implemented on the message structs.

## Debug log

Every request, response and stream chunk going through `MyAutoGen` is recorded into the builder's tech log. Drain it with:

```rust
let log: my_ai_agent::my_auto_gen::TechRequestLogger = rb.get_tech_log().await;
for item in log.into_vec() {
    println!("{:?} {:?} {}", item.timestamp, item.tp, item.data);
}
```

`TechLogItemType` is `Request | Response | Chunk`.
