#[macro_use]
pub mod validation_macros {

    /// Converts object into a number of specific type
    /// # Parameters
    ///   * object - object that supports `to_string()` function
    ///   * type - numeric type to which to convert the object to
    ///   * must_be_positive (default: false) - numeric value must be positive
    /// # Example
    /// ```
    /// use crate::validations::get_number;
    /// 
    /// #[get("/number-parse/{num}")]
    /// async fn number_parse(
    ///     num: web::Path<String>,
    /// ) -> Result<web::Json<NumberParseResponse>, crate::util::Error> {
    ///     let q = get_number!(num.to_string(), i64);
    ///     ...
    /// }
    /// ```
    macro_rules! get_number {
        ($val:expr, $typ:ty) => {
            crate::api::validations::get_number!($val, $typ, false)
        };
        ($val:expr, $typ:ty, $must_be_positive:expr) => {
            match $val.to_string().parse::<$typ>() {
                Ok(v) => {
                    if $must_be_positive && v <= 0 {
                        return Err(crate::util::Error::bad_request("must be a positive number".to_string()));
                    }

                    v
                },
                Err(err) => return Err(crate::util::Error::bad_request(err.to_string())),
            }
        };
    }
}

pub(super) use get_number;