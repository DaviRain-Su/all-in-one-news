-- Add migration script here
CREATE TABLE rebase_daily(
    key_id uuid NOT NULL UNIQUE,
    PRIMARY KEY (key_id),
    hash TEXT NOT NULL,
    id INTEGER NOT NULL,
    author TEXT NOT NULL,
    episode TEXT NOT NULL,
    introduce TEXT NOT NULL,
    time timestamptz NOT NULL,
    title TEXT NOT NULL,
    url TEXT NOT NULL
);
