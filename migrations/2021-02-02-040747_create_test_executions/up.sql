CREATE TABLE test_execution_status (
id SERIAL PRIMARY KEY,
name VARCHAR NOT NULL
);

INSERT INTO test_execution_status (name) values
('Pending'),
('Passed'),
('Failed'),
('Blocked'),
('Needs Information');

CREATE TABLE test_executions (
id SERIAL PRIMARY KEY,
test_plan_id INTEGER NOT NULL REFERENCES test_plans (id),
test_case_id INTEGER NOT NULL REFERENCES test_cases (id),
execution_status INTEGER NOT NULL REFERENCES test_execution_status (id),
execution_date TIMESTAMP,
executed_by INTEGER REFERENCES users (id)
)