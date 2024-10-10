use crate::db;
use crate::models::User;
use actix_web::{web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: hex::encode(&user.id),
            name: user.name,
            email: user.email,
        }
    }
}

pub async fn get_users() -> impl Responder {
    let mut conn = db::establish_connection();
    match User::read(&mut conn) {
        Ok(users) => {
            let user_responses: Vec<UserResponse> =
                users.into_iter().map(UserResponse::from).collect();
            HttpResponse::Ok().json(user_responses)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_user(user: web::Json<CreateUserRequest>) -> impl Responder {
    let mut conn = db::establish_connection();
    match User::create(&mut conn, &user.name, &user.email) {
        Ok(created_user) => HttpResponse::Created().json(UserResponse::from(created_user)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_user(
    path: web::Path<String>,
    user: web::Json<UpdateUserRequest>,
) -> impl Responder {
    let mut conn = db::establish_connection();
    let user_id = match hex::decode(path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match User::update(&mut conn, &user_id, &user.name, &user.email) {
        Ok(updated_user) => HttpResponse::Ok().json(UserResponse::from(updated_user)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_user(path: web::Path<String>) -> impl Responder {
    let mut conn = db::establish_connection();
    let user_id = match hex::decode(path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match User::delete(&mut conn, &user_id) {
        Ok(deleted_count) => {
            if deleted_count > 0 {
                HttpResponse::NoContent().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
