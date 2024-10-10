// src/models.rs
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Vec<u8>,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn create(conn: &mut MysqlConnection, name: &str, email: &str) -> QueryResult<User> {
        let new_user = User {
            id: Uuid::new_v4().as_bytes().to_vec(),
            name: name.to_string(),
            email: email.to_string(),
        };
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;
        Ok(new_user)
    }

    pub fn read(conn: &mut MysqlConnection) -> QueryResult<Vec<User>> {
        users::table.load::<User>(conn)
    }

    pub fn update(conn: &mut MysqlConnection, user_id: &[u8], name: &str, email: &str) -> QueryResult<User> {
        diesel::update(users::table.find(user_id))
            .set((
                users::name.eq(name),
                users::email.eq(email),
            ))
            .execute(conn)?;
        users::table.find(user_id).first(conn)
    }

    pub fn delete(conn: &mut MysqlConnection, user_id: &[u8]) -> QueryResult<usize> {
        diesel::delete(users::table.find(user_id)).execute(conn)
    }
}