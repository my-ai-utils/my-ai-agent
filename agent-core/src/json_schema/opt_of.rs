use my_json::json_writer::JsonNullValue;
use rust_extensions::StrOrString;

use super::*;

#[async_trait::async_trait]
impl<T: JsonTypeDescription> JsonTypeDescription for Option<T> {
    async fn get_description(
        has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
    ) -> my_json::json_writer::JsonObjectWriter {
        let tp = T::get_description(false, with_enum).await;

        let mut result =
            my_json::json_writer::JsonObjectWriter::new().write_json_array("any", |arr| {
                arr.write_json_object(|o| o.write("json", "null"))
                    .write_ref(&tp)
            });
        if !has_default {
            result = result.write("default", JsonNullValue);
        }

        result
    }
}

/*
fn generate_description_of_opt_parameter<T: GetJsonTypeName>(
    description: Option<&str>,
    default: Option<&str>,
    enum_data: Option<&[StrOrString<'static>]>,
) -> my_json::json_writer::JsonObjectWriter {
    let tp = T::TYPE_NAME;

    let mut result = JsonObjectWriter::new()
        .write("type", tp.as_str())
        .write_if_some("description", description);

    if let Some(enum_data) = enum_data {
        result = result.write_iter("enum", enum_data.iter().map(|itm| itm.as_str()));
    };

    if let Some(default) = default {
        result = result.write("default", default);
    } else {
        result = result.write("default", JsonNullValue);
    }

    result
}
 */
