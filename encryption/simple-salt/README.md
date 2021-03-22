# Simple password salt database
Just salt the password

**Server**

```sh
> cargo run --bin server
tide::log Logger started
    level Info
tide::server Server listening on http://127.0.0.1:8080
```

**Client**

```sh
# Registration
> curl 127.0.0.1:8080/register -d '{"username": "foo", "password": "bar"}'
Register successed

# Authentication
> curl 127.0.0.1:8080/auth -d '{"username": "foo", "password": "bar"}'
Authentication successed

# Fail authentication
> curl 127.0.0.1:8080/auth -d '{"username": "foo", "password": "wrongpw"}'
# -> 401 Unauthorized
```
