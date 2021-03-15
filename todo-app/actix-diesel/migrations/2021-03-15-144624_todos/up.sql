-- Your SQL goes here
CREATE TABLE IF NOT EXISTS todos (
    id   SERIAL       PRIMARY KEY,
    text VARCHAR(100) NOT NULL,
    done BOOLEAN      NOT NULL DEFAULT FALSE
)
