pub mod services {
    use std::{
        time::{
            SystemTime,
            UNIX_EPOCH,
        },
        sync::Arc,
    };

    use actix_web::http::header::ToStrError;
    use jsonwebtoken::{
        encode,
        decode,
        Header,
        EncodingKey,
        DecodingKey,
        Algorithm,
        Validation,
    };
    use log::error;
    use serde::{
        Deserialize,
        Serialize,
    };

    use crate::{
        UserRepository,
        model::User,
        util::{
            Error,
            ErrorCode,
        },
    };
    use super::super::traits::AuthService;

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        iat: usize,
        exp: usize,
        roles: Vec<String>,
    }

    pub struct UserData {
        pub jwt: String,
        pub user_id: i64,
        pub user_email: String,
    }

    pub struct AuthServiceImpl {
        decoding_key: DecodingKey,
        encoding_key: EncodingKey,
        user_repo: Arc<dyn UserRepository + Send + Sync>,
    }

    pub fn new_auth_service(
        key: String,
        user_repo: Arc<dyn UserRepository + Send + Sync>,
    ) -> Result<Arc<impl AuthService>, String> {
        let decoding_key = match DecodingKey::from_rsa_pem(key.clone().as_bytes()) {
            Ok(decoded_key) => decoded_key,
            Err(err) => return Err(err.to_string()),
        };
        let encoding_key = match EncodingKey::from_rsa_pem(key.clone().as_bytes()) {
            Ok(decoded_key) => decoded_key,
            Err(err) => return Err(err.to_string()),
        };
        Ok(Arc::new(AuthServiceImpl {
            decoding_key: decoding_key,
            encoding_key: encoding_key,
            user_repo: user_repo,
        }))
    }

    impl AuthServiceImpl {

        fn decode_jwt(&self, header: Option<Result<&str, ToStrError>>) -> Result<Claims, Error> {
            let mut jwt = match header {
                Some(value) => match value {
                    Ok(s) => s,
                    Err(err) => return Err(Error::internal_str(
                        ErrorCode::NoAuthorizationHeader,
                        "Authorization header is not a string",
                    )),
                },
                None => return Err(Error::internal_str(
                    ErrorCode::NoAuthorizationHeader,
                    "no Authorization header found",
                )),
            };
            if !jwt.starts_with("Bearer ") {
                return Err(Error::internal_str(ErrorCode::JwtMalformed, "Authorization header is not a JWT token"))
            }
            jwt = &jwt[6..];

            let claims = match decode::<Claims>(jwt, &self.decoding_key, &Validation::default()) {
                Ok(c) => c.claims,
                Err(err) => return Err(Error::unauthorized(
                    format!("failed to decode claims: {}", err.to_string()),
                )),
            };
        
            let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(v) => v.as_millis() as usize,
                Err(err) => return Err(Error::unauthorized(
                    format!("failed to get current time: {}", err.to_string()),
                )),
            };
        
            if now > claims.exp {
                return Err(Error::unauthorized_str("token expired"));
            }
        
            if now < claims.iat {
                return Err(Error::unauthorized_str("token not valid yet"));
            }
        
            Ok(claims)
        }

    }

    impl AuthService for AuthServiceImpl {

        fn create_jwt(&self, user: User) -> Result<UserData, Error> {
            let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(v) => v.as_millis() as usize,
                Err(err) => {
                    error!("failed to get current time: {}", err.to_string());
                    return Err(Error::internal(ErrorCode::InternalError, err.to_string()));
                },
            };
        
            let claims = Claims{
                sub: user.email.clone(),
                iat: now.clone(),
                exp: now + (3600 * 1000),
                roles: user.roles.clone(),
            };
        
            let headers = Header::new(Algorithm::RS256);
        
            match encode(&headers, &claims, &self.encoding_key.clone()) {
                Ok(jwt) => Ok(UserData {
                    jwt: jwt,
                    user_id: user.id,
                    user_email: user.email,
                }),
                Err(err) =>{
                    error!("failed to encode JWT: {}", err.to_string());
                    Err(Error::internal(ErrorCode::InternalError, err.to_string()))
                },
            }
        }

        fn get_user(&self, header: Option<Result<&str, ToStrError>>) -> Result<User, Error> {
            let claims = match self.decode_jwt(header) {
                Ok(claims) => claims,
                Err(err) => {
                    error!("failed to decode jwt: {}", err.to_string());
                    return Err(err.wrap_str("failed to decode jwt"));
                },
            };

            let user = match self.user_repo.get_by_username(claims.sub.clone()) {
                Ok(user) => match user {
                    Some(user) => user,
                    None => return Err(Error::not_found("user not found".to_string())),
                },
                Err(err) => {
                    error!("failed to load user: {}", err.to_string());
                    return Err(err.wrap_str("failed to load user"));
                },
            };

            Ok(user)
        }

        fn has_role(&self, header: Option<Result<&str, ToStrError>>, roles: Vec<&str>) -> Result<bool, Error> {
            let roles: Vec<String> = roles.iter().map(|&s| s.to_string()).collect();
            let user = match self.get_user(header) {
                Ok(user) => user,
                Err(err) => {
                    error!("failed to load user roles: {}", err.to_string());
                    return Err(err.wrap_str("failed to load user roles"));
                },
            };

            Ok(roles.iter().any(|r| user.roles.contains(r)))
        }
    }
}