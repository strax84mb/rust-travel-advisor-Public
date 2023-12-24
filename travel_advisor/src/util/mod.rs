mod errors;
mod errors_v2;

pub use errors::errors_mod as app_errors;

pub use errors_v2::ErrorV2 as Error;
pub use errors_v2::ErrorCode as ErrorCode;

mod errors_test;