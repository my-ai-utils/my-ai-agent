pub fn deserialize_array<
    TResult: crate::my_auto_gen::deserializer::impl_from_str::DeserializeToolCallParam,
>(
    src: &str,
) -> Result<Vec<TResult>, String> {
    let array_iterator = my_json::json_reader::JsonArrayIterator::new(src.as_bytes());

    let Ok(array_iterator) = array_iterator else {
        let item = TResult::from_str(src)?;
        return Ok(vec![item]);
    };

    let mut result = vec![];

    while let Some(item) = array_iterator.get_next() {
        let item = match item {
            Ok(item) => item,
            Err(_) => {
                let item = TResult::from_str(src)?;
                return Ok(vec![item]);
            }
        };

        if let Some(raw_str) = item.as_raw_str() {
            result.push(TResult::from_str(raw_str)?);
        }
    }

    Ok(result)
}

pub struct MyStruct {
    pub prop_a: i16,
    pub prop_b: String,
    pub prop_c: Option<String>,
}

impl crate::my_auto_gen::deserializer::impl_from_str::DeserializeToolCallParam for MyStruct {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        let json_iterator = my_json::json_reader::JsonFirstLineIterator::new(src.as_bytes());

        let mut prop_a = None;
        let mut prop_b = None;
        let mut prop_c = None;

        while let Some(next_item) = json_iterator.get_next() {
            let (key, value) = next_item.map_err(|err| format!("{:?}", err))?;

            let key = key.as_str().map_err(|err| format!("{:?}", err))?;

            match key.as_str() {
                "prop_a" => {
                    let Some(value) = value.as_raw_str() else {
                        return Err("Value of prop_a can not be null".to_string());
                    };

                    let value = i16::from_str(value)?;

                    prop_a = Some(value);
                }

                "prop_b" => {
                    let Some(value) = value.as_raw_str() else {
                        return Err("Value of prop_b can not be null".to_string());
                    };

                    let value = String::from_str(value)?;

                    prop_b = Some(value);
                }

                "prop_x" => {
                    if let Some(value) = value.as_raw_str() {
                        let value = String::from_str(value)?;
                        prop_c = Some(value);
                    }
                }

                _ => {}
            }
        }

        let Some(prop_a) = prop_a else {
            return Err(format!("Json filed `prop_a` is missing"));
        };

        let Some(prop_b) = prop_b else {
            return Err(format!("Json filed `prop_b` is missing"));
        };

        let result = Self {
            prop_a,
            prop_b,
            prop_c,
        };

        Ok(result)
    }
}
