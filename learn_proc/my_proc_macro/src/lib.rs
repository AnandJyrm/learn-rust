use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn derive_db_key(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = syn::parse_macro_input!(input as syn::ItemStruct);
    let input_struct_ident = input_struct.clone().ident;
    let key = attr.to_string();
    let output = quote! {
        #input_struct
        impl GetDbKey for #input_struct_ident {
            fn get_db_key() -> &'static str {
                #key
            }
        }

    };
    output.into()
}
