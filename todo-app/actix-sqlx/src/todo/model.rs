use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::{Done, FromRow, PgPool};

#[derive(Serialize, Deserialize)]
pub struct TodoRequest {
    pub description: String,
    pub done: bool,
}

#[derive(Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

impl Responder for Todo {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

impl Todo {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Todo>> {
        let mut todos = vec![];
        let recs = sqlx::query!(
            r#"
                SELECT id, description, done
                  FROM todo
                 ORDER BY ID
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            todos.push(Todo {
                id: rec.id,
                description: rec.description,
                done: rec.done,
            });
        }

        Ok(todos)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<Todo> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM todo
                 WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Todo {
            id: rec.id,
            description: rec.description,
            done: rec.done,
        })
    }

    pub async fn create(todo: TodoRequest, pool: &PgPool) -> Result<Todo> {
        let mut tx = pool.begin().await?;
        let rec = sqlx::query!(
            "
                INSERT INTO todo (description, done)
                VALUES ($1, $2)
                RETURNING id, description, done
            ",
            &todo.description,
            todo.done
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(Todo {
            id: rec.id,
            description: rec.description,
            done: rec.done,
        })
    }

    pub async fn update(id: i32, todo: TodoRequest, pool: &PgPool) -> Result<Todo> {
        let mut tx = pool.begin().await?;
        let rec = sqlx::query!(
            "UPDATE todo
                SET description=$1, done=$2
              WHERE id=$3
             RETURNING id, description, done",
            &todo.description,
            todo.done,
            id
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(Todo {
            id: rec.id,
            description: rec.description,
            done: rec.done,
        })
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<u64> {
        let mut tx = pool.begin().await?;
        let deleted = sqlx::query("DELETE FROM todo WHERE id=$1")
            .bind(id)
            .execute(&mut tx)
            .await?;
        tx.commit().await?;
        Ok(deleted.rows_affected())
    }
}
