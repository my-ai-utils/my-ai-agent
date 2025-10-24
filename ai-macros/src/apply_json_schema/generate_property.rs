use std::str::FromStr;

use proc_macro2::TokenStream;
use types_reader::{AnyValueAsStr, PropertyType, StructProperty};

pub fn generate_property(prop: StructProperty) -> Result<TokenStream, syn::Error> {
    let prop_name = prop.name.as_str();
    let attr = prop.attrs.get_attr("property")?;

    let enum_param = attr.try_get_named_param("enum");

    let (write_default, has_default) =
        if let Some(default_value) = attr.try_get_named_param("default") {
            let default_value = default_value.unwrap_as_value()?.as_string()?;
            let default_value = default_value.as_str();

            (
                quote::quote! {
                   .write("default", #default_value)
                },
                true,
            )
        } else {
            (quote::quote! {}, false)
        };

    let enum_to_render = match enum_param {
        Some(enum_value) => {
            if let Ok(array) = enum_value.unwrap_as_vec() {
                let mut array_as_tokens = Vec::new();

                for itm in array {
                    let as_str = itm.unwrap_as_value()?.as_str()?;
                    array_as_tokens.push(quote::quote! {#as_str, });
                }

                quote::quote! {.write_iter("enum", [#(#array_as_tokens)*].into_iter())}
            } else {
                let as_str = enum_value.unwrap_as_value()?.as_str()?;

                let enum_fn_name = TokenStream::from_str(as_str).unwrap();
                quote::quote! {
                    .write_iter_if_some("enum",#enum_fn_name().await.map(|itm| itm.into_iter()))
                }
            }
        }
        None => quote::quote! {},
    };

    let description = attr.get_value_from_single_or_named("description")?;
    let description = description.as_string()?;
    let description = description.as_str();

    let result = if let PropertyType::OptionOf(opt_tp) = prop.ty {
        let as_token = opt_tp.get_token_stream();
        quote::quote! {
          .write(#prop_name, Option::<#as_token>::get_description(#has_default).await.write("description", #description) #enum_to_render  #write_default)
        }
    } else {
        let token = prop.ty.get_token_stream();
        quote::quote! {
            .write(#prop_name, #token::get_description(#has_default).await.write("description", #description) #enum_to_render #write_default)
        }
    };

    Ok(result)
}
