mod todo;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::PgPool;

async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        Welcome to Actix-web with SQLx Todo example.
        Available routes:
          GET /todos
          POST /todo
          GET /todo/{id}
          PUT /todo/{id}
          DELETE /todo/{id}
        "#,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone()) // pass database pool
            .route("/", web::get().to(index))
            .configure(todo::init) // init todo routes
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
