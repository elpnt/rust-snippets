# Actix-diesel example

## Database setup

Specify the database URL in `.env`

```
DATABASE_URL="postgres://..."
```

Initial setup by `diesel-cli`

```sh
> diesel setup
> diesel migration generate todos
```

Then edit `migrations/<TIMESTAMP>_todos/{up,down}.sql`

```sql
-- up.sql
CREATE TABLE IF NOT EXISTS todos (
    id   SERIAL       PRIMARY KEY,
    text VARCHAR(100) NOT NULL,
    done BOOLEAN      NOT NULL DEFAULT FALSE
)

-- down.sql
DROP TABLE todos
```

Run migration command

```sh
> diesel migration run
```

This time, `src/schema.rs` is created.

```rust
table! {
    todos (id) {
        id -> Int4,
        text -> Varchar,
        done -> Bool,
    }
}
```
