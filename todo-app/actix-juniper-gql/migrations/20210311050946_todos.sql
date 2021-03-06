BEGIN;
    CREATE TABLE IF NOT EXISTS todos (
        id         SERIAL       PRIMARY KEY,
        task       VARCHAR(100) NOT NULL,
        done       BOOLEAN      NOT NULL DEFAULT FALSE,
        created_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP
    );
COMMIT;
