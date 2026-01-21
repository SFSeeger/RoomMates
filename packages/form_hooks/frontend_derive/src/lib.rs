use proc_macro::TokenStream;

mod enum_select;
mod field_value;

#[proc_macro_derive(EnumSelect, attributes(enum_select, label))]
pub fn enum_select_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    enum_select::impl_enum_select_macro(&ast)
}

#[proc_macro_derive(FieldValue)]
pub fn field_value_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    field_value::impl_field_value_macro(&ast)
}
