use crate::models::{NewTodo, Todo};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::prelude::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/todos")]
pub async fn find_all(pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let result = web::block(move || Todo::find_all(&conn)).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        _ => HttpResponse::BadRequest().body("cannot load all todos"),
    }
}

#[get("/todos/{id}")]
pub async fn find_by_id(id: web::Path<i32>, pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let result = web::block(move || Todo::find_by_id(id.into_inner(), &conn)).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        _ => HttpResponse::BadRequest().body("Todo not found"),
    }
}

#[post("/todo")]
pub async fn create(form: web::Json<NewTodo>, pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let result = web::block(move || Todo::create(form.text.clone(), &conn)).await;
    match result {
        Ok(created) => HttpResponse::Ok().json(created),
        _ => HttpResponse::BadRequest().body("failed to create a new todo"),
    }
}

#[put("/todos/{id}")]
pub async fn mark_as_done(id: web::Path<i32>, pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let result = web::block(move || Todo::mark_as_done(id.into_inner(), &conn)).await;
    match result {
        Ok(updated) => HttpResponse::Ok().json(updated),
        _ => HttpResponse::BadRequest().body("failed to update the todo"),
    }
}

#[delete("/todos/{id}")]
pub async fn delete(id: web::Path<i32>, pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let result = web::block(move || Todo::delete(id.into_inner(), &conn)).await;
    match result {
        Ok(deleted) => HttpResponse::Ok().json(deleted),
        _ => HttpResponse::BadRequest().body("failed to update the todo"),
    }
}
