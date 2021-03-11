use anyhow::Result;
use chrono::NaiveDateTime;
use juniper::GraphQLObject;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};

#[derive(GraphQLObject)]
pub struct Todo {
    pub id: i32,
    pub task: String,
    pub done: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Todo {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Todo>> {
        let mut todos = vec![];
        let recs = sqlx::query!(
            r#"
            SELECT * FROM todos
             ORDER BY id
            "#,
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            todos.push(Todo {
                id: rec.id,
                task: rec.task,
                done: rec.done,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
            })
        }
        Ok(todos)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<Todo> {
        let rec = sqlx::query!(
            r#"
            SELECT * FROM todos
             WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Todo {
            id: rec.id,
            task: rec.task,
            done: rec.done,
            created_at: rec.created_at,
            updated_at: rec.updated_at,
        })
    }

    pub async fn create_todo(task: String, pool: &PgPool) -> Result<Todo> {
        let mut tx = pool.begin().await?;
        let todo = sqlx::query(
            r#"
            INSERT INTO todos (task)
            VALUES ($1)
            RETURNING *
            "#,
        )
        .bind(task)
        .map(|row: PgRow| Todo {
            id: row.get(0),
            task: row.get(1),
            done: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
        })
        .fetch_one(&mut tx)
        .await?;
        tx.commit().await?;

        Ok(todo)
    }

    pub async fn mark_todo_as_done(id: i32, pool: &PgPool) -> Result<Todo> {
        let mut tx = pool.begin().await?;
        let todo = sqlx::query(
            r#"
            UPDATE todos
               SET done=TRUE,updated_at=CURRENT_TIMESTAMP
             WHERE id=$1
            RETURNING *
            "#,
        )
        .bind(id)
        .map(|row: PgRow| Todo {
            id: row.get(0),
            task: row.get(1),
            done: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
        })
        .fetch_one(&mut tx)
        .await?;
        tx.commit().await?;

        Ok(todo)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<String> {
        let mut tx = pool.begin().await?;
        sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(id)
            .execute(&mut tx)
            .await?;
        tx.commit().await?;
        Ok("deleted".into())
    }
}
