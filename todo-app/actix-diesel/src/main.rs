#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;

use actix_web::{middleware, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(routes::find_all)
            .service(routes::find_by_id)
            .service(routes::create)
            .service(routes::mark_as_done)
            .service(routes::delete)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
