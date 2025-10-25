use my_json::json_writer::JsonObjectWriter;

use super::*;

#[async_trait::async_trait]
impl<Tp: JsonTypeDescription> JsonTypeDescription for Vec<Tp> {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<rust_extensions::StrOrString<'static>>>,
    ) -> JsonObjectWriter {
        let items = Tp::get_description(false, with_enum).await;
        JsonObjectWriter::new()
            .write("type", "array")
            .write("items", items)
    }
}
