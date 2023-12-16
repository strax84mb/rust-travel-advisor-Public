use super::{
    Airport,
    Comment,
};

pub struct City {
    pub id: i64,
    pub name: String,
    pub airports: Vec<Airport>,
    pub comments: Vec<Comment>,
}

impl City {
    pub fn new(id: i64, name: String) -> Self {
        City {
            id: id,
            name: name,
            comments: vec![],
            airports: vec![],
        }
    }
}
