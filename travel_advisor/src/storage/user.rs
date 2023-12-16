pub mod users {
    use std::sync::Arc;

    use diesel::prelude::*;

    use crate::{
        Database,
        util::app_errors::Error,
        model::{
            User,
            UserDB,
        },
        schema::users::dsl as user_dsl,
    };
    use super::super::db_context::db_macros::get_connection;

    pub trait UserRepository {
        fn get_by_id(&self, id: i64) -> Result<Option<User>, Error>;
        fn get_by_username(&self, name: String) -> Result<Option<User>, Error>;
        fn get_by_email_and_pass(&self, email: String, password: String) -> Result<Option<User>, Error>;
    }

    pub fn new_user_repository(db: Arc<Database>) -> Arc<impl UserRepository> {
        Arc::new(UserRepositoryImpl {
            db: db,
        })
    }

    struct UserRepositoryImpl {
        db: Arc<Database>,
    }

    impl UserRepository for UserRepositoryImpl {
    
        fn get_by_id(&self, id: i64) -> Result<Option<User>, Error> {
            let conn = &mut get_connection!(self.db);
            match user_dsl::users
                .find(id)
                .select(UserDB::as_select())
                .first(conn)
                .optional() {
                    Ok(result) => match result {
                        Some(user) => Ok(Some(User::from_db(&user))),
                        None => Ok(None),
                    },
                    Err(err) => Err(Error::underlying(err.to_string())),
                }
        }
    
        fn get_by_username(&self, name: String) -> Result<Option<User>, Error> {
            let conn = &mut get_connection!(self.db);
            match user_dsl::users
                .filter(user_dsl::email.eq(name))
                .select(UserDB::as_select())
                .first(conn)
                .optional() {
                    Ok(result) => match result {
                        Some(user) => Ok(Some(User::from_db(&user))),
                        None => Ok(None),
                    },
                    Err(err) => Err(Error::underlying(err.to_string())),
                }
        }

        fn get_by_email_and_pass(&self, email: String, password: String) -> Result<Option<User>, Error> {
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
            let conn = &mut get_connection!(self.db);
            let result = user_dsl::users
                .filter(user_dsl::email.eq(email))
                .filter(user_dsl::pass.eq(pass))
                .select(UserDB::as_select())
                .first(conn)
                .optional();
            match result {
                Ok(result) => match result {
                    Some(user) => Ok(Some(User::from_db(&user))),
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