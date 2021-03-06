use std::env;

use dotenv::dotenv;
use sqlx::postgres::PgPool;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    Add { description: String },
    Done { id: i32 },
}

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[paw::main]
#[tokio::main]
async fn main(args: Args) -> anyhow::Result<()> {
    dotenv().ok();
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    match args.cmd {
        Some(Command::Add { description }) => {
            println!("Adding new todos with description: {}", description);
            let todo_id = add_todo(&pool, description).await?;
            println!("Added new todo with id: {}", todo_id);
        }
        Some(Command::Done { id }) => {
            println!("Marking todo {} as done", id);
            if complete_todo(&pool, id).await? {
                println!("Todo {} is marked as done", id);
            } else {
                println!("Invalid id: {}", id);
            }
        }
        None => {
            println!("Printing list of all todos");
            list_todos(&pool).await?;
        }
    }

    Ok(())
}

async fn add_todo(pool: &PgPool, description: String) -> anyhow::Result<i32> {
    let rec = sqlx::query!(
        r#"
            INSERT INTO todos (description)
            VALUES ($1)
            RETURNING id
        "#,
        description
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

async fn complete_todo(pool: &PgPool, id: i32) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
            UPDATE todos
               SET done = TRUE
             WHERE id = $1

        "#,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

async fn list_todos(pool: &PgPool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
            SELECT id, description, done
              FROM todos
             ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        println!(
            "- [{}] {}: {}",
            if rec.done { "x" } else { " " },
            rec.id,
            rec.description
        );
    }

    Ok(())
}
