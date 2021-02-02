CREATE TABLE permissions (
id SERIAL PRIMARY KEY,
name VARCHAR NOT NULL
);

INSERT INTO permissions (name) values
('Requirements - Guest'),
('Requirements - Create'),
('Requirements - Modify'),
('Requirements - Delete'),
('Test Cases - Guest'),
('Test Cases - Create'),
('Test Cases - Modify'),
('Test Cases - Delete'),
('Test Plans - Guest'),
('Test Plans - Create'),
('Test Plans - Modify'),
('Test Plans - Delete'),
('Test Executions - Guest'),
('Test Executions - Create'),
('Test Executions - Modify'),
('Test Executions - Delete'),
('Automation - Guest'),
('Automation - Create'),
('Automation - Modify'),
('Automation - Delete'),
('Reports - Guest'),
('Reports - Create'),
('Reports - Modify'),
('Reports - Delete');

CREATE TABLE user_group (
id SERIAL PRIMARY KEY,
name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE user_group_permissions (
id SERIAL PRIMARY KEY,
user_group_id INTEGER NOT NULL REFERENCES user_group (id),
permission_id INTEGER NOT NULL REFERENCES permissions (id),
UNIQUE (user_group_id, permission_id)
);

CREATE TABLE users (
id SERIAL PRIMARY KEY,
username VARCHAR NOT NULL,
name VARCHAR,
email VARCHAR UNIQUE,
admin BOOLEAN DEFAULT FALSE NOT NULL,
user_group_id INTEGER REFERENCES user_group (id)
);

INSERT INTO users (username, email, admin) values ('admin', 'admin@localhost', true)