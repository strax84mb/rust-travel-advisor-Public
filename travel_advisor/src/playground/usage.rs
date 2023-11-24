pub trait Greeter {
    fn say_hello(&self, your_name: &str);
}

#[cfg(test)]
mod tests {

    use serde::Serialize;
    use test_annotations::Greeter;

    use super::{
        Greeter,
        super::macros::my_macros::ok,
    };

    #[derive(Serialize, Greeter, Clone)]
    struct Human {
        name: String,
        age: u16,
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
}
