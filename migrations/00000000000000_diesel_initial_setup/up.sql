-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.




-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```

CREATE TABLE users (
    id SERIAL NOT NULL PRIMARY KEY,
    email TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (email, username)
);
;

 INSERT INTO users (id, email, username, password, created_at) VALUES
  (1, 'admin@example.com', 'admin', '$2y$12$yXTjrGePVLBPUH6YVs2f5OsUEGSotZxdL5Uu/70r63I5GtynVVjkK', '2018-06-08 13:00:26.353041'),
  (2, 'john.doe@example.com', 'John Doe', '$2y$12$3lOwd/qun2g.KBQpYz7DQu4HgreLODO4aJgYwFAQNj2AqgS14DAMK', '2018-06-08 13:00:28.353041'),
  (3, 'peter.doe@example.com', 'Peter Doe', '$2y$12$6ofSZ3hpsGtDt6bM0WU0geDgZLLETFUVB6FpMXI61SbAvuQD5RiWK', '2018-06-08 13:00:38.353041');
 SELECT setval('users_id_seq', 3, true);

CREATE TABLE cloudenvironments ( 
    id SERIAL NOT NULL PRIMARY KEY,
    env_name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO cloudenvironments(id, env_name, created_at) VALUES
(1, 'env-staging-1', '2018-07-21 23:41:45.672805609'),
(2, 'env-staging-2', '2018-07-21 23:41:45.672805609'),
(3, 'env-staging-3', '2018-07-21 23:41:45.672805609'),
(4, 'env-production', '2018-07-21 23:41:45.672805609');
SELECT setval('cloudenvironments_id_seq', 4, true);

CREATE TABLE products (
    id SERIAL NOT NULL PRIMARY KEY,
    prod_name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO products(id, prod_name, created_at) VALUES
(1, 'Product 1', '2018-07-21 23:41:45.672805609'),
(2, 'Product 2', '2018-07-21 23:41:45.672805609'),
(3, 'Product 3', '2018-07-21 23:41:45.672805609'),
(4, 'Product 4', '2018-07-21 23:41:45.672805609');
SELECT setval('products_id_seq', 4, true);

CREATE TABLE productowner (
    id SERIAL NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO productowner(id, name, created_at) VALUES
(1, 'Sam Doe', '2018-07-21 23:41:45.672805609'),
(2, 'Peter Doe', '2018-07-21 23:41:45.672805609'),
(3, 'Jane Doe', '2018-07-21 23:41:45.672805609'),
(4, 'Jason Doe', '2018-07-21 23:41:45.672805609'),
(5, 'John Doe', '2018-07-21 23:41:45.672805609'),
(6, 'Steve Doe', '2018-07-21 23:41:45.672805609');
SELECT setval('products_id_seq', 6, true);

CREATE TABLE usersenvs ( 
    id SERIAL NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    env_id INTEGER NOT NULL,
    po_id INTEGER NOT NULL,
    start_date TIMESTAMP NULL, 
    max_duration TEXT NULL,
    lease_period TEXT NULL,
    status INTEGER,
    env_type TEXT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP 
);

INSERT INTO usersenvs(id, user_id, env_id, po_id, start_date, 
max_duration, lease_period, status, env_type, created_at) VALUES
(1, 2, 2, 3, '2018-07-21 23:41:45.672805609', '12hrs', 'from 24th Aug to 25th Aug 2018', 1, 'EB', '2018-07-21 23:41:45.672805609');
SELECT setval('products_id_seq', 1, true);
