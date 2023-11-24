pub mod services {
    use std::{
        time::{
            SystemTime,
            UNIX_EPOCH,
        },
        sync::Arc,
    };

    use actix_web::http::header::ToStrError;
    use async_trait::async_trait;
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
        util::app_errors::Error,
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
        key: String,
        user_repo: Arc<dyn UserRepository + Send + Sync>,
    }

    pub fn new_auth_service(
        key: String,
        user_repo: Arc<dyn UserRepository + Send + Sync>,
    ) -> Arc<impl AuthService> {
        Arc::new(AuthServiceImpl {
            key: key,
            user_repo: user_repo,
        })
    }

    impl AuthServiceImpl {

        fn decode_jwt(&self, header: Option<Result<&str, ToStrError>>) -> Result<Claims, Error> {
            let mut jwt = match header {
                Some(value) => match value {
                    Ok(s) => s,
                    Err(_err) => return Err(Error::from_str("Authorization header is not a string")),
                },
                None => return Err(Error::from_str("no Authorization header found")),
            };
            if !jwt.starts_with("Bearer ") {
                return Err(Error::from_str("Authorization header is not a JWT token"))
            }
            jwt = &jwt[6..];

            let key = match DecodingKey::from_rsa_pem(self.key.clone().as_bytes()) {
                Ok(key) => key,
                Err(err) => return Err(Error::underlying(
                    format!("failed load key: {}", err.to_string())
                )),
            };
            let claims = match decode::<Claims>(jwt, &key, &Validation::default()) {
                Ok(c) => c.claims,
                Err(err) => return Err(Error::underlying(
                    format!("failed to decode claims: {}", err.to_string())
                )),
            };
        
            let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(v) => v.as_millis() as usize,
                Err(err) => return Err(Error::underlying(
                    format!("failed to get current time: {}", err.to_string())
                )),
            };
        
            if now > claims.exp {
                return Err(Error::from_str("token expired"));
            }
        
            if now < claims.iat {
                return Err(Error::from_str("token not valid yet"));
            }
        
            Ok(claims)
        }

    }

    #[async_trait]
    impl AuthService for AuthServiceImpl {

        async fn create_jwt(&self, user: User) -> Result<UserData, Error> {
            let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(v) => v.as_millis() as usize,
                Err(err) => {
                    error!("failed to get current time: {}", err.to_string());
                    return Err(Error::underlying(err.to_string()));
                },
            };
        
            let claims = Claims{
                sub: user.email.clone(),
                iat: now.clone(),
                exp: now + (3600 * 1000),
                roles: user.roles.clone(),
            };
        
            let headers = Header::new(Algorithm::RS256);
            let key = match EncodingKey::from_rsa_pem(self.key.clone().as_bytes()) {
                Ok(b) => b,
                Err(_err) => {
                    error!("this should never happen: {}", _err.to_string());
                    panic!("this should never happen");
                }
            };
        
            match encode(&headers, &claims, &key) {
                Ok(jwt) => Ok(UserData {
                    jwt: jwt,
                    user_id: user.id,
                    user_email: user.email,
                }),
                Err(err) =>{
                    error!("failed to encode JWT: {}", err.to_string());
                    Err(Error::underlying(err.to_string()))
                },
            }
        }

        async fn get_user(&self, header: Option<Result<&str, ToStrError>>) -> Result<User, Error> {
            let claims = match self.decode_jwt(header) {
                Ok(claims) => claims,
                Err(err) => {
                    error!("failed to decode jwt: {}", err.to_string());
                    return Err(Error::underlying(
                        format!("failed to decode jwt: {}", err.to_string())
                    ));
                },
            };

            let user = match self.user_repo.get_by_username(claims.sub.clone()).await {
                Ok(user) => user,
                Err(err) => {
                    error!("failed to load user: {}", err.to_string());
                    return Err(Error::underlying(
                        format!("failed to load user: {}", err.to_string())
                    ));
                },
            };

            Ok(user)
        }

        async fn has_role(&self, header: Option<Result<&str, ToStrError>>, roles: Vec<&str>) -> Result<bool, Error> {
            let roles: Vec<String> = roles.iter().map(|&s| s.to_string()).collect();
            let user = match self.get_user(header).await {
                Ok(user) => user,
                Err(err) => {
                    error!("failed to load user roles: {}", err.to_string());
                    return Err(Error::underlying(
                        format!("failed to load user roles: {}", err.to_string())
                    ));
                },
            };

            Ok(roles.iter().any(|r| user.roles.contains(r)))
        }
    }
}