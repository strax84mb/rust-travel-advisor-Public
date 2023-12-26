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