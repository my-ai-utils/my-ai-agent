#![allow(warnings)]

use my_ai_agent::macros::ApplyJsonSchema;
use rust_extensions::StrOrString;

#[derive(ApplyJsonSchema)]
pub struct MyRequestModel {
    #[property(description: "city")]
    pub city: String,
    #[property(description: "service")]
    pub service: Option<String>,
    #[property(description: "Address of dealer")]
    pub addr: Option<String>,
    #[property(enum:["NEW", "CPO"], description: "Vehicle condition (NEW/CPO). Defaults to None")]
    pub condition: Option<String>,

    #[property(enum: "get_other_condition_enum", description: "Vehicle condition (NEW/CPO). Defaults to None")]
    pub other_condition: String,

    #[property(description: "Minimal condition")]
    pub min_condition: Option<i64>,

    #[property(enum:["Value1", "Value2", "Value3"], description: "Several enum values")]
    pub several_enum: Vec<String>,

    #[property(enum:["Value1", "Value2", "Value3"], description: "Several enum values optional")]
    pub several_enum_opt: Option<Vec<String>>,
}

async fn get_other_condition_enum() -> Option<Vec<StrOrString<'static>>> {
    None
}

impl MyRequestModel {
    async fn get_description() -> my_ai_agent::my_json::json_writer::JsonObjectWriter {
        use my_ai_agent::json_schema::JsonTypeDescription;
        let props = my_ai_agent::my_json::json_writer::JsonObjectWriter::new()
            .write(
                "city",
                String::get_description(false)
                    .await
                    .write("description", "city"),
            )
            .write(
                "service",
                Option::<String>::get_description(false)
                    .await
                    .write("description", "service"),
            )
            .write(
                "addr",
                Option::<String>::get_description(false)
                    .await
                    .write("description", "Address of dealer"),
            )
            .write(
                "condition",
                Option::<String>::get_description(false)
                    .await
                    .write(
                        "description",
                        "Vehicle condition (NEW/CPO). Defaults to None",
                    )
                    .write_iter("enum", ["NEW", "CPO"].into_iter()),
            )
            .write(
                "other_condition",
                String::get_description(false)
                    .await
                    .write(
                        "description",
                        "Vehicle condition (NEW/CPO). Defaults to None",
                    )
                    .write_iter_if_some(
                        "enum",
                        get_other_condition_enum().await.map(|itm| itm.into_iter()),
                    ),
            )
            .write(
                "min_condition",
                Option::<i64>::get_description(false)
                    .await
                    .write("description", "Minimal condition"),
            )
            .write(
                "several_enum",
                Vec::<String>::get_description(false)
                    .await
                    .write("description", "Several enum values")
                    .write_iter("enum", ["Value1", "Value2", "Value3"].into_iter()),
            )
            .write(
                "several_enum_opt",
                Option::<Vec<String>>::get_description(false)
                    .await
                    .write("description", "Several enum values optional")
                    .write_iter("enum", ["Value1", "Value2", "Value3"].into_iter()),
            );
        my_ai_agent::my_json::json_writer::JsonObjectWriter::new()
            .write("type", "object")
            .write("properties", props)
            .write_iter(
                "required",
                ["city", "other_condition", "several_enum"].into_iter(),
            )
            .write("additionalProperties", false)
    }
}

#[cfg(test)]
mod tests {
    use my_ai_agent::models::*;

    use crate::tests::MyRequestModel;
    use my_ai_agent::json_schema::JsonTypeDescription;

    #[tokio::test]
    async fn test_generation() {
        let description = MyRequestModel::get_description().await;

        println!("{}", description.build());
    }

    #[tokio::test]
    async fn test_builder_and_model() {
        let mut builder = my_ai_agent::OpenAiRequestBodyBuilder::new_with_system_prompt(
            "test_system_prompt_data",
            my_ai_agent::models::LlmModel::Gpt4o(my_ai_agent::models::Gpt4Settings::default()),
        );

        let func_json_description = FunctionDescriptionJsonModel {
            name: "filter_showrooms".to_string(),
            description: "Filters company location data from a JSON file based on criteria like city (extracted from address), specific service offered (e.g., Sales, Repairs), and geolocation ranges (latitude/longitude).".to_string(),
            parameters: serde_json::from_str(MyRequestModel::get_description().await.build().as_str()).unwrap(),
            strict: None,
        };

        let func_json_description = serde_json::to_value(&func_json_description).unwrap();

        builder
            .add_tools_call_description(func_json_description)
            .await;

        let model = builder.get_model().await;

        let json_str = serde_json::to_string_pretty(&model).unwrap();

        println!("{}", json_str);
    }
}
