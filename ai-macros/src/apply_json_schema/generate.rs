use proc_macro::TokenStream;
use types_reader::StructProperty;
pub fn generate(input: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    //    let input_token_stream: proc_macro2::TokenStream = input.clone().into();

    let struct_name = &input.ident;

    let fields = StructProperty::read(input)?;

    let mut fields_to_render = Vec::new();

    let mut required_fields = Vec::new();

    for prop in fields {
        if !prop.ty.is_option() {
            required_fields.push(prop.name.to_string());
        }

        let property = super::generate_property(prop)?;

        fields_to_render.push(property);
    }

    let required = if required_fields.len() > 0 {
        quote::quote! {  .write_iter("required", [#(#required_fields,)*].into_iter())}
    } else {
        quote::quote! {  .write("required", my_ai_agent::my_json::json_writer::EmptyJsonArray)}
    };

    let result = quote::quote! {

        #[async_trait::async_trait]
        impl my_ai_agent::json_schema::JsonSchemaDescription  for #struct_name{

        async fn get_description() -> my_ai_agent::my_json::json_writer::JsonObjectWriter {
        use  my_ai_agent::json_schema::JsonTypeDescription;

        let props = my_ai_agent::my_json::json_writer::JsonObjectWriter::new()

        #(#fields_to_render)*;

        my_ai_agent::my_json::json_writer::JsonObjectWriter::new().write("type", "object")
        .write("properties", props )
        #required
        .write("additionalProperties", false)

       }

      }

      #[async_trait::async_trait]
      impl my_ai_agent::json_schema::GetJsonTypeName for #struct_name{
            const TYPE_NAME: &'static str = "object";
            const OPTIONAL:bool = false;
      }

    };
    Ok(result.into())
}
