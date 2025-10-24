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

/*
#[async_trait::async_trait]
impl<T: JsonSchemaDescription> JsonSchemaDescription for Vec<T> {
    async fn get_description() -> my_json::json_writer::JsonObjectWriter {
        let description = T::get_description().await;

        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "array")
            .write("items", description)
    }
}

pub fn fill_array_sub_elements<Tp: JsonSchemaDescription>(
    writer: JsonObjectWriter,
    enum_data: &Option<Vec<StrOrString<'static>>>,
) -> JsonObjectWriter {
    let tp = Tp::TYPE_NAME;
    writer.write_json_object("items", move |items| {
        let mut items = items.write("type", tp.as_str());
        if let Some(enum_data) = enum_data {
            let enums = enum_data.iter().map(|itm| itm.as_str());
            items = items.write_iter("enum", enums);
        }

        items
    })
}
 */
