use derivates::impl_say_hello;
use proc_macro::TokenStream;

mod derivates;

#[proc_macro_derive(Greeter)]
pub fn say_hello_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    
    impl_say_hello(&ast)
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }

}
