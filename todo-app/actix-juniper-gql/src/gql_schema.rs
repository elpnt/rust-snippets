use crate::{context::Database, model::Todo};
use juniper::{graphql_object, EmptySubscription, FieldError, FieldResult};

pub struct Query;

#[graphql_object(context = Database)]
impl Query {
    pub async fn all_todos(context: &Database) -> FieldResult<Vec<Todo>> {
        Todo::find_all(&context.pool)
            .await
            .map_err(|e| FieldError::from(e))
    }

    pub async fn get_todo_by_id(id: i32, context: &Database) -> FieldResult<Todo> {
        Todo::find_by_id(id, &context.pool)
            .await
            .map_err(|e| FieldError::from(e))
    }
}

pub struct Mutation;

#[graphql_object(context = Database)]
impl Mutation {
    pub async fn create_todo(task: String, context: &Database) -> FieldResult<Todo> {
        Todo::create_todo(task, &context.pool)
            .await
            .map_err(|e| FieldError::from(e))
    }

    pub async fn mark_todo_as_done(id: i32, context: &Database) -> FieldResult<Todo> {
        Todo::mark_todo_as_done(id, &context.pool)
            .await
            .map_err(|e| FieldError::from(e))
    }

    pub async fn delete_todo(id: i32, context: &Database) -> FieldResult<String> {
        Todo::delete(id, &context.pool)
            .await
            .map_err(|e| FieldError::from(e))
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Database>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
