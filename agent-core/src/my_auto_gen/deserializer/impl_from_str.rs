use std::str::FromStr;

use rust_extensions::str_utils::StrUtils;

pub trait DeserializeToolCallParam {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized;
}

impl DeserializeToolCallParam for u8 {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        number_from_str(src)
    }
}

impl DeserializeToolCallParam for i8 {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        number_from_str(src)
    }
}

impl DeserializeToolCallParam for u16 {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        number_from_str(src)
    }
}

impl DeserializeToolCallParam for i16 {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        number_from_str(src)
    }
}

impl DeserializeToolCallParam for u32 {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        number_from_str(src)
    }
}

impl DeserializeToolCallParam for i32 {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        number_from_str(src)
    }
}

impl DeserializeToolCallParam for u64 {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        number_from_str(src)
    }
}

impl DeserializeToolCallParam for i64 {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        number_from_str(src)
    }
}

impl DeserializeToolCallParam for usize {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        number_from_str(src)
    }
}

impl DeserializeToolCallParam for isize {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        number_from_str(src)
    }
}

impl DeserializeToolCallParam for String {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        if src.starts_with('"') {
            Ok(src[1..src.len() - 1].to_string())
        } else {
            Ok(src.to_string())
        }
    }
}

impl DeserializeToolCallParam for bool {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        if src.eq_case_insensitive("true") {
            return Ok(true);
        }

        if src.eq_case_insensitive("false") {
            return Ok(false);
        }

        return Err(format!("Can not deserialize value `{src}` to boolean type"));
    }
}

fn number_from_str<T: FromStr>(src: &str) -> Result<T, String> {
    match src.parse() {
        Ok(result) => return Ok(result),
        Err(_) => {
            return Err(format!("Can not convert to number from str {src}"));
        }
    }
}
