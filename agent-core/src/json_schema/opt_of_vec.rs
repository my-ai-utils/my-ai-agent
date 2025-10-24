/*
#[async_trait::async_trait]
impl<T: JsonTypeDescription> JsonTypeDescription for Option<Vec<T>> {
    async fn fill_type_description(
        src: my_json::json_writer::JsonObjectWriter,
        description: Option<&str>,
        default: Option<&str>,
        enum_data: Option<&[StrOrString<'static>]>,
    ) -> my_json::json_writer::JsonObjectWriter {
        let items = T::fill_type_description(src, None, None, enum_data).await;
        src.write_if_some("description", description)
            .write("items", items)
            .write("default", default)
    }
}
 */
/*
async fn generate_description_of_opt_of_vec_parameter<Tp: JsonSchemaDescription>(
    description: Option<&str>,
    default: Option<&str>,
    enum_data: Option<Vec<StrOrString<'static>>>,
) -> my_json::json_writer::JsonObjectWriter {
    let result = JsonObjectWriter::new()
        .write("type", "array")
        .write_if_some("description", description)
        .write("default", default);

    super::vec_of::fill_array_sub_elements::<Tp>(result, &enum_data).write("uniqueItems", true)
}
 */
