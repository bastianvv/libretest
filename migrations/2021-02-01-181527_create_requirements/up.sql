CREATE TABLE requirement_status (
id SERIAL PRIMARY KEY,
name VARCHAR NOT NULL
);

INSERT INTO requirement_status (name) values
('WIP'),
('Under review'),
('Rejected'),
('Approved'),
('Obsolete');

CREATE TABLE requirement_importance (
id SERIAL PRIMARY KEY,
name VARCHAR NOT NULL
);

INSERT INTO requirement_importance (name) values
('Critical'),
('Major'),
('Minor');

CREATE TABLE requirement_nature (
id SERIAL PRIMARY KEY,
name VARCHAR NOT NULL
);

INSERT INTO requirement_nature (name) values
('Business'),
('UI'),
('Performance'),
('Integration'),
('Technical'),
('Use Case'),
('User Story'),
('Undefined');

CREATE TABLE requirements (
id SERIAL PRIMARY KEY,
creation_date TIMESTAMP NOT NULL DEFAULT now(),
created_by INTEGER NOT NULL REFERENCES users (id),
updated_date TIMESTAMP,
updated_by INTEGER REFERENCES users (id),
code VARCHAR,
status INTEGER NOT NULL DEFAULT 1 REFERENCES requirement_status (id),
importance INTEGER REFERENCES requirement_importance (id),
nature INTEGER REFERENCES requirement_nature (id),
title VARCHAR NOT NULL,
description VARCHAR
);

CREATE TABLE "linked_requirements" (
id SERIAL PRIMARY KEY,
parent_id INTEGER NOT NULL REFERENCES requirements (id),
child_id INTEGER NOT NULL REFERENCES requirements (id),
UNIQUE(parent_id, child_id)
);