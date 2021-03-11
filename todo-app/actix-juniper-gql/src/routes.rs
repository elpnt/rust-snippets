use crate::{context::Database, gql_schema::Schema};
use actix_web::{web, HttpRequest, Responder};
use dotenv::dotenv;
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

// Create a handler that replies with an HTML page containing GraphiQL
pub async fn graphiql_route() -> impl Responder {
    graphiql_handler("/graphiql", None).await
}

// Create a handler that replies with an HTML page containing GraphQL Playground
pub async fn playground_route() -> impl Responder {
    playground_handler("/graphql", None).await
}

// GraphQL endpoint for `GET` and `POST` requests
pub async fn graphql_route(
    req: HttpRequest,
    payload: web::Payload,
    schema: web::Data<Schema>,
) -> impl Responder {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let pool = sqlx::PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to the database");
    let context = Database { pool };
    graphql_handler(&schema, &context, req, payload).await
}
