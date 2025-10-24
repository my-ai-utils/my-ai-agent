#[async_trait::async_trait]
pub trait JsonTypeDescription {
    async fn get_description(has_default: bool) -> my_json::json_writer::JsonObjectWriter;
}

#[async_trait::async_trait]
impl JsonTypeDescription for u8 {
    async fn get_description(_has_default: bool) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new().write("type", "number")
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for i8 {
    async fn get_description(_has_default: bool) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new().write("type", "number")
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for u16 {
    async fn get_description(_has_default: bool) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new().write("type", "number")
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for i16 {
    async fn get_description(_has_default: bool) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new().write("type", "number")
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for u32 {
    async fn get_description(_has_default: bool) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new().write("type", "number")
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for i32 {
    async fn get_description(_has_default: bool) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new().write("type", "number")
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for u64 {
    async fn get_description(_has_default: bool) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new().write("type", "number")
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for i64 {
    async fn get_description(_has_default: bool) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new().write("type", "number")
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for f64 {
    async fn get_description(_has_default: bool) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new().write("type", "number")
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for f32 {
    async fn get_description(_has_default: bool) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new().write("type", "number")
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for usize {
    async fn get_description(_has_default: bool) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new().write("type", "number")
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for isize {
    async fn get_description(_has_default: bool) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new().write("type", "number")
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for bool {
    async fn get_description(_has_default: bool) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new().write("type", "boolean")
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for String {
    async fn get_description(_has_default: bool) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new().write("type", "string")
    }
}

#[async_trait::async_trait]
impl JsonTypeDescription for &'_ String {
    async fn get_description(_has_default: bool) -> my_json::json_writer::JsonObjectWriter {
        my_json::json_writer::JsonObjectWriter::new().write("type", "string")
    }
}
