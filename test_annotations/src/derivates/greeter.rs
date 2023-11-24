use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_say_hello(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Greeter for #name {
            fn say_hello(&self, your_name: &str) {
                println! ("Hello {}", your_name);
            }
        }
    };
    gen.into()
}