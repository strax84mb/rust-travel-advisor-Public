pub trait Greeter {
    fn say_hello(&self, your_name: &str);
}

#[cfg(test)]
mod tests {

    use serde::Serialize;
    use test_annotations::{
        fn_attr_macro,
        fn_sec_attr_macro,
        Greeter,
    };

    use super::{
        Greeter,
        super::macros::my_macros::ok,
    };

    #[derive(Serialize, Greeter, Clone)]
    struct Human {
        name: String,
        age: u16,
    }

    #[fn_sec_attr_macro]
    #[fn_attr_macro]
    fn test_fn(i: i32, name: &str) {
        println!("Number: {} - Name: {}", i, name)
    }

    #[test]
    fn test_ok() {
        let human = Human {
            name: "Strale".to_string(),
            age: 39,
        };
        ok!(human.clone());
        human.say_hello("Strale");
    }

    #[test]
    fn test_fn_attr_macro() {
        test_fn(1, "Strale");
    }

    #[test]
    fn it_works() {
        let temp = "fn test_fn (i : i32 , name : & str) {println ! (\"Number: {} - Name: {}\" , i , name)}";
        let index = temp.find("{").unwrap();
        let temp_str = temp.to_string();
        let (first, last) = temp_str.split_at(index + 1);
        println!("first >>> {}", first.clone());
        println!("last >>> {}", last.clone());
        let new = format!("{}{}{}", first, "println!(\"Hello from the beggining of function\");", last);
        println!("{}", new);
    }

}
