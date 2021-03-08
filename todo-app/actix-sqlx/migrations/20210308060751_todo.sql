-- Add migration script here
CREATE TABLE IF NOT EXISTS todo (
    id          SERIAL  PRIMARY KEY,
    description TEXT    NOT NULL,
    done        BOOLEAN NOT NULL DEFAULT FALSE
)
