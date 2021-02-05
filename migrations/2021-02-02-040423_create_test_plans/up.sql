CREATE TABLE test_plan_status (
id SERIAL PRIMARY KEY,
name VARCHAR NOT NULL
);

INSERT INTO test_plan_status (name) values
('Pending'),
('Scheduled'),
('In Progress'),
('Passed'),
('Failed'),
('Blocked');

CREATE TABLE test_plans (
    id SERIAL PRIMARY KEY,
    creation_date TIMESTAMP NOT NULL,
    created_by INTEGER NOT NULL REFERENCES users (id),
    updated_date TIMESTAMP,
    updated_by INTEGER REFERENCES users (id),
    code VARCHAR,
    status INTEGER NOT NULL REFERENCES test_plan_status (id),
    start_date DATE,
    end_date DATE,
    title VARCHAR NOT NULL,
    description VARCHAR
);