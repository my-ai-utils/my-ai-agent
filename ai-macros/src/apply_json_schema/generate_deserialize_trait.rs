use proc_macro2::Ident;
use proc_macro2::TokenStream;
use types_reader::PropertyType;
use types_reader::StructProperty;

pub fn generate_deserialize_trait(
    struct_name: &Ident,
    fields: &[StructProperty],
) -> Result<TokenStream, syn::Error> {
    let mut init_null_props = vec![];

    let mut match_cases = vec![];

    let mut create_props = vec![];

    let mut null_verifications = vec![];

    for field in fields {
        let prop_name = field.get_field_name_ident();
        let prop_type = field.get_syn_type();
        let prop_name_as_str = prop_name.to_string();

        init_null_props.push(quote::quote! {
            let mut #prop_name = None;
        });

        create_props.push(quote::quote! {
            #prop_name,
        });

        match &field.ty {
            PropertyType::OptionOf(tp) => {
                if let PropertyType::VecOf(items) = tp.as_ref() {
                    let tp = items.get_token_stream();
                    match_cases.push(quote::quote! {
                        #prop_name_as_str =>{
                             if let Some(value) = value.as_raw_str() {
                                 let value: Vec<#tp> =
                                my_ai_agent::my_auto_gen::deserializer::deserialize_array(value)?;
                            #prop_name = Some(value);
                        }
                        }
                    });
                } else {
                    let tp = tp.get_token_stream();
                    match_cases.push(quote::quote! {
                        #prop_name_as_str =>{
                            if let Some(value) = value.as_raw_str() {
                                if !value.eq_ignore_ascii_case("null") {
                                  let value = #tp::from_str(value)?;
                                  #prop_name = Some(value);
                                }
                            }
                        }
                    });
                }
            }

            PropertyType::VecOf(items) => {
                let tp = items.get_token_stream();
                match_cases.push(quote::quote! {
                    #prop_name_as_str =>{
                         if let Some(value) = value.as_raw_str() {
                             let value: Vec<#tp> =
                            my_ai_agent::my_auto_gen::deserializer::deserialize_array(value)?;
                        #prop_name = Some(value);
                    }
                    }
                });

                null_verifications.push(quote::quote! {
                    let Some(#prop_name) = #prop_name else {
                      return Err(format!("Json filed `#prop_name` is missing"));
                    };
                });
            }

            _ => {
                match_cases.push(quote::quote! {
                    #prop_name_as_str =>{
                           let Some(value) = value.as_raw_str() else {
                                return Err("Value of `#prop_name` can not be null".to_string());
                            };

                            let value = #prop_type::from_str(value)?;

                            #prop_name = Some(value);
                    },
                });

                null_verifications.push(quote::quote! {
                    let Some(#prop_name) = #prop_name else {
                      return Err(format!("Json filed `#prop_name` is missing"));
                    };
                });
            }
        }
    }

    let result = quote::quote! {

        impl my_ai_agent::my_auto_gen::deserializer::impl_from_str::DeserializeToolCallParam for #struct_name {

            fn from_str(src: &str) -> Result<Self, String> where Self: Sized,
            {
                #(#init_null_props)*

                let json_iterator = my_ai_agent::my_json::json_reader::JsonFirstLineIterator::new(src.as_bytes());

                while let Some(next_item) = json_iterator.get_next() {
                    let (key, value) = next_item.map_err(|err| format!("{:?}", err))?;
                    let key = key.as_str().map_err(|err| format!("{:?}", err))?;

                     match key.as_str() {
                        #(#match_cases)*
                        _ => {}
                        }

                }

                #(#null_verifications)*


                let result = Self {#(#create_props)* };

                Ok(result)
            }
        }
    };

    Ok(result)
}
