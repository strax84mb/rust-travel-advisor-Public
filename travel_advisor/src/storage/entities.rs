use std::{
    time::{
        SystemTime,
        Duration,
        UNIX_EPOCH,
    },
    ops::Add,
};

use chrono::NaiveDateTime;
use diesel::{
    Queryable,
    Selectable,
    Identifiable,
    Insertable,
    AsChangeset,
};

use crate::model::{
    Airport,
    City,
    Comment,
};

#[derive(Queryable, Selectable, Identifiable, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::cities)]
pub struct CityDB {
    pub id: i64,
    pub name: String,
}

impl CityDB {
    pub fn to_city(&self) -> City {
        City {
            id: self.id,
            name: self.name.clone(),
            airports: vec![],
            comments: vec![],
        }
    }
}

#[derive(Selectable, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserDB {
    pub id: i64,
    pub email: String,
    pub pass: String,
    pub roles: String,
}

#[derive(Selectable, Queryable, Identifiable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::airports)]
pub struct AirportDB {
    pub id: i64,
    pub city_id: i64,
    pub name: String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::airports)]
pub struct InsertAirportDB {
    pub city_id: i64,
    pub name: String
}

impl AirportDB {
    pub fn to_model(&self) -> Airport {
        Airport {
            id: self.id,
            name: self.name.clone(),
            city_id: self.city_id,
        }
    }
}

#[derive(Selectable, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::routes)]
pub struct Route {
    pub id: i64,
    pub start: i64,
    pub finish: i64,
    pub price: i64,
}

#[derive(Selectable, Queryable, Identifiable, AsChangeset, Clone)]
#[diesel(table_name = crate::schema::comments)]
pub struct CommentDB {
    pub id: i64,
    pub user_id: i64,
    pub city_id: i64,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub fn naive_to_system(value: NaiveDateTime) -> SystemTime {
    UNIX_EPOCH.add(Duration::from_secs(value.timestamp() as u64))
}

impl CommentDB {
    pub fn to_model(&self) -> Comment {
        Comment {
            id: self.id.clone(),
            user_id: self.user_id.clone(),
            city_id: self.city_id.clone(),
            content: self.text.clone(),
            created_at: naive_to_system(self.created_at.clone()),
            updated_at: naive_to_system(self.updated_at.clone()),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::comments)]
pub struct InsertCommentDB {
    pub user_id: i64,
    pub city_id: i64,
    pub text: String,
}
