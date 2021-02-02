CREATE TABLE test_plans (
id SERIAL PRIMARY KEY,
code VARCHAR,
start_date DATE,
end_date DATE,
title VARCHAR NOT NULL,
description VARCHAR NOT NULL
)