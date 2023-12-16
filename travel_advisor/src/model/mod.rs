mod airport;
mod city;
mod comment;
mod common;
mod route;
mod user;

pub type Airport = airport::Airport;
pub type User = user::User;
pub type UserDB = user::UserDB;
pub type City = city::City;
pub type Comment = comment::Comment;
pub type Route = route::Route;

mod airports;
mod airports_test;