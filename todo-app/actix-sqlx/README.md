# Actix + SQLx (PostgreSQL) example
Simple todo web app built with actix and sqlx

## Download SQLx cli

```sh
> cargo install sqlx-cli --no-default-features --features postgres
```

## Setup database

```sh
> sqlx migrate add todo
> edit migrations/<timestamp>_todo.sql
```

```sql
-- <timestamp>_todo.sql
CREATE TABLE IF NOT EXISTS todo (
    id          SERIAL  PRIMARY KEY,
    description TEXT    NOT NULL,
    done        BOOLEAN NOT NULL DEFAULT FALSE
)
```

## Usage

**Server**

```sh
> cargo run
```

**Client**

```
# List all todos
> curl 127.0.0.1:8080/todos

# Add a new todo
> curl 127.0.0.1 -H 'Content-Type: Application/json' -d '{"description": "buy milk", "done": false }'
```

## Tips

### use `sqlx::query_as!` macro

This macro can specify the output type returned by the query. For example,

```rust
struct Account {
    id: i32,
    name: String,
    profile: String,
}

let account = sqlx::query_as!(
    Account,
    "SELECT * FROM accounts WHERE id = $1",
    id
    )
    .fetch_one(&mut conn)
    .await?;
println!("{}: {}[{}]", account.id, account.name, account.profile);
```
