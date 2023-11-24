pub mod errors_mod {
    use std::fmt::{Display, Debug};

    #[derive(Debug, PartialEq)]
    pub enum Reason {
        Wrapped,
        NotFound,
        Forbidden,
        Underlying,
    }

    impl Reason {
        pub fn to_string(&self) -> String {
            let text = match self {
                Self::Forbidden => "FORBIDDEN",
                Self::NotFound => "NOT_FOUND",
                Self::Underlying => "INTERNAL_ERROR",
                _ => "UNSUPPORTED_ERROR",
            };
            text.to_string()
        }
    }

    #[derive(Debug)]
    pub struct Error {
        reason: Reason,
        msg: String,
        cause: Option<Box<Error>>
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = self;
            //let step_one = write!(f, "{}: ", self.msg.clone());
            let cause = s.cause.as_ref();
            if cause.is_some() {
                let step_one = write!(f, "{}: ", self.msg.clone());
                match step_one {
                    Ok(_) => {
                        let e =cause.as_deref().unwrap();
                        return <Error as Display>::fmt(e.as_ref(), f)
                    },
                    Err(err) => return Err(err),
                }
            } else {
                write!(f, "{}", self.msg.clone())
            }
        }
    }

    impl std::error::Error for Error {
        fn cause(&self) -> Option<&dyn std::error::Error> {
            None
        }
    }

    impl Error {
        pub fn not_found() -> Self {
            Error {
                reason: Reason::NotFound,
                msg: "not found".to_string(),
                cause: None,
            }
        }

        pub fn underlying(msg: String) -> Self {
            Error {
                reason: Reason::Underlying,
                msg: msg,
                cause: None,
            }
        }

        pub fn forbidden() -> Self {
            Error {
                reason: Reason::Forbidden,
                msg: "forbidden".to_string(),
                cause: None,
            }
        }

        pub fn from_str(msg: &str) -> Self {
            Error {
                reason: Reason::Underlying,
                msg: msg.to_string(),
                cause: None,
            }
        }

        pub fn wrap(err: Error, msg: String) -> Self {
            Error {
                reason: Reason::Wrapped,
                msg: msg,
                cause: Some(Box::new(err)),
            }
        }

        pub fn wrap_str(err: Error, msg: &str) -> Self {
            Error {
                reason: Reason::Wrapped,
                msg: msg.to_string(),
                cause: Some(Box::new(err)),
            }
        }

        pub fn type_message(&self, reason: Reason) -> Option<String> {
            let mut cause: Option<&Error> = Some(&self);
            loop {
                match cause {
                    Some(e) => {
                        if e.reason == reason {
                            return Some(e.msg.clone());
                        } else if e.cause.is_some() {
                            cause = e.cause.as_deref();
                        } else {
                            cause = None;
                        }
                    },
                    None => break,
                }
            };

            None
        }

        pub fn message(&self) -> String {
            self.msg.clone()
        }
    }

}
