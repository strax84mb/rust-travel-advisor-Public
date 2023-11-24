#[cfg(test)]
mod errors_tests {
    use super::super::app_errors::{Error, Reason};

    #[test]
    fn test_errors() {
        let nfe = Error::not_found();
        let we = Error::wrap(nfe, "some error".to_string());

        let mut msg = we.type_message(Reason::NotFound);
        assert_eq!(true, msg.is_some());
        assert_eq!("not found", msg.unwrap());

        msg = we.type_message(Reason::Underlying);
        assert_eq!(true, msg.is_none());

        assert_eq!("some error", we.message());
        assert_eq!("some error: not found", we.to_string());
    }
}