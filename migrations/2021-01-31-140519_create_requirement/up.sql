CREATE TABLE "requirements" (
id SERIAL PRIMARY KEY,
creation_date TIMESTAMP NOT NULL,
code VARCHAR,
status INTEGER NOT NULL,
importance INTEGER,
nature INTEGER,
title VARCHAR NOT NULL,
description VARCHAR
)
