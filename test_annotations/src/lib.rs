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

fn get_param_name_by_type(param_type: &str, load: String) -> Option<String> {
    match load.find(param_type) {
        None => None,
        Some(pos) => {
            let mut temp: String = load.chars().skip(0).take(pos).collect();
            let start_pos = match temp.rfind(',') {
                None => temp.rfind('(').expect("not even first param"),
                Some(pos1) => pos1,
            };
            temp = temp.chars().skip(start_pos + 1).take_while(|x| (*x).ne(&':')).collect();
            Some(temp.trim().to_string())
        },
    }
}

#[proc_macro_attribute]
pub fn roles(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func_str = item.to_string();
    let roles = attr.to_string();
    let auth_service_param = get_param_name_by_type("Data<Arc<dyn AuthService + Send + Sync>>", func_str.clone());
    let req_param = get_param_name_by_type("HttpRequest", func_str.clone());
    let (first, other) = func_str.split_at(func_str.find('(').expect("failed to split string") + 1);
    let (second, third) = other.split_at(other.find('{').expect("failed to get split string the second time") + 1);
    let validation_code = format!(
        "match {}.has_role(extract_auth(&{}), vec![{}]) {{
            Ok(found) if !found => return Ok(respond_unauthorized(None)),
            Err(err) => return Ok(resolve_error(err, Some(\"failed to check JWT\"))),
        _   => (),
        }};",
        match auth_service_param.clone() {
            Some(s) => s,
            None => "auth_service".to_string(),
        },
        match req_param.clone() {
            Some(s) => s,
            None => "req".to_string(),
        },
        roles,
    );
    let final_func = format!(
        "{}{}{}{}{}{}",
        first,
        match req_param {
            None => "req: actix_web::HttpRequest,",
            _ => "",
        },
        match auth_service_param {
            None => "auth_service: actix_web::web::Data<Arc<dyn AuthService + Send + Sync>>,",
            _ => "",
        },
        second,
        validation_code,
        third,
    );
    final_func.parse().expect("failed to process the function")
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        println!("{}", "strale");
    }

}
