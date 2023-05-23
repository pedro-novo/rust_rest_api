CREATE TABLE users
(
    id          VARCHAR(255) PRIMARY KEY,
    username       VARCHAR(255) NOT NULL,
    age             INT,
    created_at  TIMESTAMP
)