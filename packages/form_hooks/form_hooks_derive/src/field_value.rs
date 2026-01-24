use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

pub(crate) fn impl_field_value_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let variants = if let Data::Enum(data_enum) = &ast.data {
        &data_enum.variants
    } else {
        return syn::Error::new_spanned(name, "EnumSelect can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let to_input_value_arms = variants.iter().enumerate().map(|(i, v)| {
        let variant = &v.ident;
        let i = (i + 1).to_string();
        quote! {
            #name::#variant => #i.to_string(),
        }
    });

    let from_input_value_arms = variants.iter().enumerate().map(|(i, v)| {
        let variant = &v.ident;
        let i = (i + 1).to_string();
        quote! {
            #i => Ok(#name::#variant),
        }
    });

    let generated = quote! {
        impl FieldValue for #name {
            fn to_input_value(&self) -> String {
                match self {
                    #(#to_input_value_arms)*
                }
            }

            fn from_input_value(value: &str) -> Result<Self, String> {
                match value {
                    #(#from_input_value_arms)*
                    &_ => Err(format!("Unknown variant: {}", value)),
                }
            }
        }
    };
    generated.into()
}
