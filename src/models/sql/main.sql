CREATE TABLE users (
    id         SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name  VARCHAR(100) NOT NULL,
    email      VARCHAR(100) NOT NULL,
    phone      VARCHAR(100),
    password   VARCHAR(1000) NOT NULL,
    perm       SMALLINT NOT NULL,
    created    TIMESTAMP NOT NULL,
    image      VARCHAR(500),
    UNIQUE(email)
);

CREATE TABLE email_verification_token (
    id         BYTEA PRIMARY KEY,
    email      TEXT UNIQUE NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);