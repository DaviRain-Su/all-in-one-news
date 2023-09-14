-- Add migration script here
-- Create Subscriptions Table
CREATE TABLE rebase_daily(
key_id uuid NOT NULL,
PRIMARY KEY (key_id),
id INTEGER NOT NULL,
author TEXT NOT NULL UNIQUE,
episode TEXT NOT NULL,
introduce TEXT NOT NULL,
time timestamptz NOT NULL,
title TEXT NOT NULL,
url TEXT NOT NULL,
tag TEXT[] NOT NULL
);
