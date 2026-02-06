use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Expr, LitStr};

pub(crate) fn impl_enum_select_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut message_value = String::from("Select a Value");
    for attr in &ast.attrs {
        if attr.path().is_ident("enum_select") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("default_label") {
                    let expr: LitStr = meta.value()?.parse()?;
                    message_value = expr.value();
                }
                Ok(())
            })
            .unwrap();
        }
    }

    let variants = if let Data::Enum(data_enum) = &ast.data {
        &data_enum.variants
    } else {
        return syn::Error::new_spanned(name, "EnumSelect can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let entries = variants.iter().map(|v| {
        let variant = &v.ident;
        let mut label = quote! {#variant}.to_string();

        for attr in &v.attrs {
            if attr.path().is_ident("label") {
                let label_expr: Expr = attr.parse_args().unwrap();
                if let Expr::Lit(expr_lit) = label_expr
                    && let syn::Lit::Str(lit_str) = expr_lit.lit
                {
                    label = lit_str.value();
                }
            }
        }
        quote! {
            (#name::#variant.to_input_value(), #label),
        }
    });

    let generated = quote! {
        impl EnumSelect for #name {
            fn select_options() -> Vec<(String, &'static str)> {
                vec![
                    #(#entries)*
                ]
            }
        }
        impl EnumSelectDefault for #name {
            fn default_label() -> &'static str {
                #message_value
            }
        }
    };
    generated.into()
}
