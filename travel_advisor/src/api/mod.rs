mod airport;
mod city;
mod comment;
mod dtos;
mod hello;
mod users;
mod validations;

pub use hello::hello_world;

pub use hello::init as init_hello;
pub use city::init as init_city;
pub use users::init as init_user;
pub use airport::init as init_airport;
pub use comment::init as init_comments;


#[macro_use]
pub mod auth_macro {

    /// Get user from Authorization header if it exists and has provided roles
    /// # Parameters:
    ///   * req = actix_web::HttpRequest
    ///   * auth_service = Arc<dyn AuthService + Send + Sync>
    ///   * roles = Vec<&str>
    /// # Returns
    ///   If successful, object of type `crate::model::User` shall be returned.
    ///   Operation is successful if all of these conditions are met:
    ///   * http request (`req`) has header Authorization
    ///   * value of header Authorization is `Bearer <jwt>`
    ///   * JWT is a JSON Web Token as described in `RFC-7519` specification
    ///   * user mentioned in JWT exists in database
    ///   * user has at least one role stated in `roles` parameter
    /// # Example
    /// ```
    /// use crate::api::get_user_if_has_roles;
    /// let user = get_user_if_has_roles!(req, auth_service, vec!["admin"]);
    /// ```
    macro_rules! get_user_if_has_roles {
        ($req:expr, $auth_service:expr, $roles:expr) => {
            match $auth_service.get_user_if_has_role(
                match $req.headers().get(actix_web::http::header::AUTHORIZATION) {
                    Some(header) => Some(header.to_str()),
                    None => None,
                },
                $roles
            ) {
                Err(err) => return Err(err),
                Ok(user_option) => match user_option {
                    Some(user) => user,
                    None => return Err(Error::unauthorized_str("user has no rights for this operation")),
                }
            }
        };
    }

}

pub(super) use get_user_if_has_roles;
