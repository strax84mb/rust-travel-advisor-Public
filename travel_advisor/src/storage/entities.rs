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
