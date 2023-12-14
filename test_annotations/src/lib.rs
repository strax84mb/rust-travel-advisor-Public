use derivates::impl_say_hello;
use proc_macro::TokenStream;

mod derivates;

#[proc_macro_derive(Greeter)]
pub fn say_hello_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    
    impl_say_hello(&ast)
}

///
/// # Syntax
///  Just use it on a function
#[proc_macro_attribute]
pub fn fn_attr_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = item.to_string();
    let start_index = func.find("{").unwrap();
    let (first, last) = func.split_at(start_index + 1);
    let front_command = "println!(\"Hello from the beggining of function\");";
    format!("{}{}{}", first, front_command, last).parse().expect("failed to process new function")
}

#[proc_macro_attribute]
pub fn fn_sec_attr_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = item.to_string();
    let start_index = func.find("{").unwrap();
    let (first, last) = func.split_at(start_index + 1);
    let front_command = "println!(\"Hello from the second line of function\");";
    format!("{}{}{}", first, front_command, last).parse().expect("failed to process new function")
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }

}
