use rust_extensions::StrOrString;

#[async_trait::async_trait]
pub trait JsonTypeDescription {
    async fn get_description(
        has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        output: bool,
    ) -> my_json::json_writer::JsonObjectWriter;
}

#[async_trait::async_trait]
impl JsonTypeDescription for u8 {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "number")
            .write_iter_if_some("enum", with_enum.map(|itm| itm.into_iter()))
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for i8 {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "number")
            .write_iter_if_some("enum", with_enum.map(|itm| itm.into_iter()))
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for u16 {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "number")
            .write_iter_if_some("enum", with_enum.map(|itm| itm.into_iter()))
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for i16 {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "number")
            .write_iter_if_some("enum", with_enum.map(|itm| itm.into_iter()))
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for u32 {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "number")
            .write_iter_if_some("enum", with_enum.map(|itm| itm.into_iter()))
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for i32 {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "number")
            .write_iter_if_some("enum", with_enum.map(|itm| itm.into_iter()))
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for u64 {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "number")
            .write_iter_if_some("enum", with_enum.map(|itm| itm.into_iter()))
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for i64 {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "number")
            .write_iter_if_some("enum", with_enum.map(|itm| itm.into_iter()))
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for f64 {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "number")
            .write_iter_if_some("enum", with_enum.map(|itm| itm.into_iter()))
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for f32 {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "number")
            .write_iter_if_some("enum", with_enum.map(|itm| itm.into_iter()))
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for usize {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "number")
            .write_iter_if_some("enum", with_enum.map(|itm| itm.into_iter()))
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for isize {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "number")
            .write_iter_if_some("enum", with_enum.map(|itm| itm.into_iter()))
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for bool {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "boolean")
            .write_iter_if_some("enum", with_enum.map(|itm| itm.into_iter()))
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for String {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "string")
            .write_iter_if_some("enum", with_enum.map(|itm| itm.into_iter()))
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for &'_ String {
    async fn get_description(
        _has_default: bool,
        with_enum: Option<Vec<StrOrString<'static>>>,
        _output: bool,
    ) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new()
            .write("type", "string")
            .write_iter_if_some("enum", with_enum.map(|itm| itm.into_iter()))
    }
}
