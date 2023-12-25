#[macro_use]
pub mod service_macros {

    /// Runs the expression. If the result is error, it will be logged before passing through.
    macro_rules! log_if_error {
        ($operation:expr) => {
            match $operation {
                Ok(result) => Ok(result),
                Err(err) => {
                    log::error!("failed getting airports: {}", err.to_string());
                    Err(err)
                },
            }
        };

    }
}

pub(super) use log_if_error;