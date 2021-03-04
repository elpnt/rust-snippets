# SQLx + Postgres todo example

## PostgreSQL server

Run the postgres server with `schema.sql`

```sh
> docker-compose up -d
```

## Run the app

```sh
# Add a new todo
> cargo run -- add "todo description"

# Mark todo as done
> cargo run -- done <todo id>

# List all todos
> cargo run
```

Example:

```sh
> cargo run -- add "Drink coffee"
Adding new todos with description: Drink coffee
Added new todo with id: 1

> cargo run -- add "Fix my laptop"
Adding new todos with description: Fix my laptop
Added new todo with id: 2

> cargo run -- done 1
Marking todo 1 as done
Todo 1 is marked as done

> cargo run
Printing list of all todos
- [x] 1: Drink coffee
- [ ] 2: Fix my laptop

```
