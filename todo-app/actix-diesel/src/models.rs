use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::todos;
use crate::schema::todos::dsl::*;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub done: bool,
}

impl Todo {
    pub fn find_all(conn: &PgConnection) -> Result<Vec<Todo>, diesel::result::Error> {
        todos.load::<Todo>(conn)
    }

    pub fn find_by_id(todo_id: i32, conn: &PgConnection) -> Result<Todo, diesel::result::Error> {
        todos.find(todo_id).first::<Todo>(conn)
    }

    pub fn create(todo_text: String, conn: &PgConnection) -> Result<Todo, diesel::result::Error> {
        diesel::insert_into(todos)
            .values(&text.eq(todo_text))
            .get_result(conn)
    }

    pub fn mark_as_done(todo_id: i32, conn: &PgConnection) -> Result<Todo, diesel::result::Error> {
        diesel::update(todos.find(todo_id))
            .set(done.eq(true))
            .get_result(conn)
    }

    pub fn delete(todo_id: i32, conn: &PgConnection) -> Result<Todo, diesel::result::Error> {
        diesel::delete(todos.find(todo_id)).get_result(conn)
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewTodo {
    pub text: String,
}
