// tauri-src-1/models
use diesel::prelude::*;
use crate::schema::{ users, wrestlers, belts };
// use diesel::Associations;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = wrestlers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Wrestler {
    pub id: i32,
    pub name: String,
    pub gender: String,
    pub wins: i32,
    pub losses: i32
}

#[derive(Insertable)]
#[diesel(table_name = wrestlers)]
pub struct NewWrestler<'a> {
    pub name: &'a str,
    pub gender: &'a str
}

#[derive(Debug, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = belts)]
#[diesel(belongs_to(Wrestler, foreign_key = current_holder_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Title {
    pub id: i32,
    pub name: String,
    pub current_holder_id: Option<i32>
}

#[derive(Insertable)]
#[diesel(table_name = belts)]
pub struct NewTitle<'a> {
    pub name: &'a str
}