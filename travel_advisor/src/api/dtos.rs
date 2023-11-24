use std::time::SystemTime;

use serde::{
    Serialize,
    Deserialize,
};

use crate::model::{
    Airport,
    City,
    Comment,
};

#[derive(Serialize, Deserialize)]
pub struct CityDto {
    pub id: i64,
    pub name: String,
    pub airports: Vec<AirportDto>,
}

impl CityDto {
    pub fn from_model(c: &City) -> Self {
        let airports: Vec<AirportDto> = c.airports.iter()
            .map(|a| AirportDto::from_model(a))
            .collect();
        
        CityDto {
            id: c.id,
            name: c.name.clone(),
            airports: airports,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserDto {
    pub id: i64,
    pub email: String,
    pub roles: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub pass: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub id: i64,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct AirportDto {
    pub id: i64,
    pub city_id: i64,
    pub name: String,
}

impl AirportDto {
    pub fn from_model(a: &Airport) -> AirportDto {
        AirportDto {
            id: a.id.clone(),
            city_id: a.city_id.clone(),
            name: a.name.clone(),
        }
    }

    pub fn to_model(&self) -> Airport {
        Airport {
            id: self.id.clone(),
            city_id: self.city_id.clone(),
            name: self.name.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CommentDto {
    pub id: i64,
    pub user_id: i64,
    pub city_id: i64,
    pub content: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl CommentDto {
    pub fn from_model(c: &Comment) -> Self {
        CommentDto {
            id: c.id.clone(),
            user_id: c.user_id.clone(),
            city_id: c.city_id.clone(),
            content: c.content.clone(),
            created_at: c.created_at.clone(),
            updated_at: c.updated_at.clone(),
        }
    }

    pub fn to_model(&self) -> Comment {
        Comment {
            id: self.id.clone(),
            user_id: self.user_id.clone(),
            city_id: self.city_id.clone(),
            content: self.content.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}
