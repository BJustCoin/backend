CREATE TABLE users (
    id         SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name  VARCHAR(100) NOT NULL,
    email      VARCHAR(100) NOT NULL,
    phone      VARCHAR(100),
    password   VARCHAR(1000) NOT NULL,
    perm       SMALLINT NOT NULL,
    image      VARCHAR(500),
    created    TIMESTAMP NOT NULL,
    uuid       VARCHAR(100) NOT NULL,
    UNIQUE(email)
);

CREATE TABLE email_verification_token (
    id         BYTEA PRIMARY KEY,
    email      TEXT UNIQUE NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
); 

CREATE TABLE wallets (
    id       SERIAL PRIMARY KEY,
    user_id  INT NOT NULL,
    link     VARCHAR(100) NOT NULL
); 

CREATE TABLE white_lists (
    id         SERIAL PRIMARY KEY,
    user_id    INT NOT NULL,
    token_type SMALLINT NOT NULL
);

CREATE TABLE logs (
    id       SERIAL PRIMARY KEY,
    user_id  INT NOT NULL,
    text     VARCHAR(100) NOT NULL,
    created  TIMESTAMP NOT NULL DEFAULT current_timestamp
); 

CREATE TABLE suggest_items (
    id          SERIAL PRIMARY KEY,
    first_name  VARCHAR(100) NOT NULL,
    middle_name VARCHAR(100) NOT NULL,
    last_name   VARCHAR(100) NOT NULL,
    email       VARCHAR(100) NOT NULL,
    phone       VARCHAR(20) NOT NULL,
    mobile      VARCHAR(20) NOT NULL,
    is_agree    BOOLEAN NOT NULL DEFAULT TRUE,
    address     VARCHAR(100) NOT NULL,
    created     TIMESTAMP NOT NULL
);