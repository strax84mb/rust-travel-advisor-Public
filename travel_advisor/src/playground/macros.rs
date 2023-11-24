#[macro_use]
pub mod my_macros {

    use actix_web::http::StatusCode;
    use serde::Serialize;
    use serde_json;

    #[allow(dead_code)]
    pub fn print_json(payload: impl Serialize) {
        let _q = StatusCode::INTERNAL_SERVER_ERROR;
        match serde_json::to_string(&payload) {
            Ok(payload) => println!("{}", payload),
            Err(err) => println!("{}", err.to_string()),
        };
    }

    #[allow(unused_macros)]
    macro_rules! ok {
        ($payload:expr) => {
            use crate::playground::macros::my_macros::print_json;
            print_json($payload);
        };
    }

    #[allow(unused_imports)]
    pub(crate) use ok;

}
