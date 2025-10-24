use proc_macro::TokenStream;

extern crate proc_macro;
mod apply_json_schema;

#[proc_macro_derive(ApplyJsonSchema, attributes(property))]
pub fn apply_json_schema(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    match crate::apply_json_schema::generate(&ast) {
        Ok(result) => result.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
