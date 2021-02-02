CREATE TABLE test_case_status (
id SERIAL PRIMARY KEY,
name VARCHAR NOT NULL
);

INSERT INTO test_case_status (name) values
('WIP'),
('Under review'),
('Rejected'),
('Approved'),
('Obsolete');

CREATE TABLE test_case_importance (
id SERIAL PRIMARY KEY,
name VARCHAR NOT NULL
);

INSERT INTO test_case_importance (name) values
('Very High'),
('High'),
('Medium'),
('Low');

CREATE TABLE test_case_nature (
id SERIAL PRIMARY KEY,
name VARCHAR NOT NULL
);

INSERT INTO test_case_nature (name) values
('Functional'),
('UI/UX'),
('Integration'),
('Performance'),
('Security'),
('Database'),
('UAT');

CREATE TABLE test_case_type (
id SERIAL PRIMARY KEY,
name VARCHAR NOT NULL
);

INSERT INTO test_case_type (name) values
('Classic'),
('Exploratory'),
('Gherkin');

CREATE TABLE test_cases (
    id SERIAL PRIMARY KEY,
    creation_date TIMESTAMP DEFAULT now(),
    created_by INTEGER REFERENCES users (id),
    updated_date TIMESTAMP,
    updated_by INTEGER REFERENCES users (id),
    code VARCHAR,
    status INTEGER DEFAULT 1 REFERENCES test_case_status (id),
    importance INTEGER REFERENCES test_case_importance (id),
    nature INTEGER REFERENCES test_case_nature (id),
    test_type INTEGER NOT NULL REFERENCES test_case_type (id),
    automated BOOLEAN DEFAULT false,
    title VARCHAR NOT NULL,
    description VARCHAR
);

CREATE TABLE linked_test_cases (
id SERIAL PRIMARY KEY,
parent_id INTEGER NOT NULL REFERENCES test_cases (id),
child_id INTEGER NOT NULL REFERENCES test_cases (id),
UNIQUE(parent_id, child_id)
);

CREATE TABLE traceability (
id SERIAL PRIMARY KEY,
req_id INTEGER NOT NULL REFERENCES requirements (id),
tc_id INTEGER NOT NULL REFERENCES test_cases (id),
UNIQUE(req_id, tc_id)
);

CREATE TABLE test_case_steps (
id SERIAL PRIMARY KEY,
tc_id INTEGER NOT NULL REFERENCES test_cases (id),
step_order INTEGER NOT NULL,
step_description VARCHAR,
step_expected_results VARCHAR
);

CREATE TABLE test_case_gherkin (
id SERIAL PRIMARY KEY,
tc_id INTEGER NOT NULL REFERENCES test_cases (id),
script VARCHAR NOT NULL
);