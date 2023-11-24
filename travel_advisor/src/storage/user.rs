use sqlx::FromRow;

use crate::{
    util::app_errors::Error,
    model::User
};
use super::db_context::Database;

impl Database {
    pub async fn get_user(&self, id: i64) -> Result<User, Error> {
        let result = sqlx::query("SELECT id, email, pass, roles FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(self.connections.as_ref())
            .await;
        match result {
            Ok(row) => {
                match User::from_row(&row) {
                    Ok(user) => Ok(user),
                    Err(err) => Err(Error::underlying(err.to_string())),
                }
            },
            Err(sqlx::Error::RowNotFound) => Err(Error::not_found()),
            Err(err) => Err(Error::underlying(err.to_string())),
        }
    }

    pub async fn get_user_by_email_and_pass(&self, email: String, password: String) -> Result<User, Error> {
        let pass = md5::compute(password.as_bytes());
        let pass: String = pass.iter().map(|&q| q as char).collect();
        let result = sqlx::query("SELECT id, email, pass, roles FROM users WHERE email = ? AND pass = ?")
            .bind(email)
            .bind(pass)
            .fetch_one(self.connections.as_ref())
            .await;
        match result {
            Ok(row) => {
                match User::from_row(&row) {
                    Ok(user) => Ok(user),
                    Err(err) => Err(Error::underlying(err.to_string())),
                }
            },
            Err(sqlx::Error::RowNotFound) => Err(Error::not_found()),
            Err(err) => Err(Error::underlying(err.to_string())),
        }
    }
}

pub mod users {
    use std::sync::Arc;

    use async_trait::async_trait;
    use sqlx::{
        FromRow,
    };

    use crate::{
        Database,
        util::app_errors::Error,
        model::User
    };

    #[async_trait]
    pub trait UserRepository {
        async fn get_by_id(&self, id: i64) -> Result<User, Error>;
        async fn get_by_username(&self, name: String) -> Result<User, Error>;
        async fn get_by_email_and_pass(&self, email: String, password: String) -> Result<Option<User>, Error>;
    }

    pub fn new_user_repository(db: Arc<Database>) -> Arc<impl UserRepository> {
        Arc::new(UserRepositoryImpl {
            db: db,
        })
    }

    struct UserRepositoryImpl {
        db: Arc<Database>,
    }

    #[async_trait]
    impl UserRepository for UserRepositoryImpl {
    
        async fn get_by_id(&self, id: i64) -> Result<User, Error> {
            let result = sqlx::query("SELECT id, email, pass, roles FROM users WHERE id = ?")
                .bind(id)
                .fetch_optional(self.db.connections.as_ref())
                .await;
            match result {
                Ok(row) => match row {
                    Some(row) => match User::from_row(&row) {
                        Ok(user) => Ok(user),
                        Err(err) => Err(Error::underlying(err.to_string())),
                    },
                    None => Err(Error::not_found()),
                }
                Err(err) => Err(Error::underlying(err.to_string())),
            }
        }
    
        async fn get_by_username(&self, name: String) -> Result<User, Error> {
            let result = sqlx::query("SELECT id, email, pass, roles FROM users WHERE email = ?")
                .bind(name)
                .fetch_optional(self.db.connections.as_ref())
                .await;
            match result {
                Ok(row) => match row {
                    Some(row) => match User::from_row(&row) {
                        Ok(user) => Ok(user),
                        Err(err) => Err(Error::underlying(err.to_string())),
                    },
                    None => Err(Error::not_found()),
                }
                Err(err) => Err(Error::underlying(err.to_string())),
            }
        }

        async fn get_by_email_and_pass(&self, email: String, password: String) -> Result<Option<User>, Error> {
            // calculate hash
            let pass = md5::compute(password.as_bytes());
            let mut chars: Vec<char> = Vec::new();
            pass.to_vec().iter().for_each(|&x| {
                let mut first = (x / 16) + 48;
                if first > 57 {
                    first += 7;
                }
                let mut second = (x % 16) + 48;
                if second > 57 {
                    second += 7;
                }
                chars.push(first as char);
                chars.push(second as char);
            });
            let pass: String = chars.iter().collect();
            let pass = pass.to_lowercase();
            // load user
            let result = sqlx::query("SELECT id, email, pass, roles FROM users WHERE email = ? AND pass = ?")
                .bind(email)
                .bind(pass)
                .fetch_optional(self.db.connections.as_ref())
                .await;
            match result {
                Ok(row) => match row {
                    Some(row) => match User::from_row(&row) {
                        Ok(user) => Ok(Some(user)),
                        Err(err) => Err(Error::underlying(err.to_string())),
                    },
                    None => Ok(None),
                },
                Err(err) => Err(Error::underlying(err.to_string())),
            }
        }
            
    }

}

#[cfg(test)]
mod user_test {
    #[test]
    fn test_md5() {
        let password = "admin_pass".to_string();
        let pass = md5::compute(password.as_bytes());
        let mut chars: Vec<char> = Vec::new();
        pass.to_vec().iter().for_each(|&x| {
            let mut first = (x / 16) + 48;
            if first > 57 {
                first += 7;
            }
            let mut second = (x % 16) + 48;
            if second > 57 {
                second += 7;
            }
            chars.push(first as char);
            chars.push(second as char);
        });
        let pass: String = chars.iter().collect();

        assert_eq!("7adc785be4a31eff6783871ff63e18f1".to_string(), pass.to_lowercase())
    }
}