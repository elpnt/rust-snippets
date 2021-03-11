use juniper::Context;
use sqlx::PgPool;

pub struct Database {
    pub pool: PgPool,
}

impl Context for Database {}
