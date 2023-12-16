use diesel::{
    Queryable,
    Selectable,
    Identifiable,
};

pub struct User {
    pub id: i64,
    pub email: String,
    pub pass: String,
    pub roles: Vec<String>,
}

#[derive(Selectable, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserDB {
    pub id: i64,
    pub email: String,
    pub pass: String,
    pub roles: String,
}

/*impl<'c> FromRow<'c, MySqlRow> for User {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        let id: i64 = match row.try_get(0) {
            Ok(id) => id,
            Err(err) => return Err(err),
        };
        let email: String = match row.try_get(1) {
            Ok(v) => v,
            Err(err) => return Err(err),
        };
        let pass: String = match row.try_get(2) {
            Ok(v) => v,
            Err(err) => return Err(err),
        };
        let roles_string: String = match row.try_get(3) {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        let roles: Vec<String> = roles_string.split(',').map(|s| s.to_string()).collect();

        Ok(User { 
            id: id, 
            email: email, 
            pass: pass, 
            roles: roles,
        })
    }
}*/

impl User {

    pub fn from_db(user: &UserDB) -> User {
        User {
            id: user.id,
            email: user.email.clone(),
            pass: user.pass.clone(),
            roles: user.roles.split(',').map(|s| s.to_string()).collect(),
        }
    }

    pub fn is_admin(&self) -> bool {
        self.roles.iter().any(|r| r == "admin")
    }

}