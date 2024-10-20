CREATE DOMAIN t_email AS VARCHAR(255)
CHECK (VALUE ~* '^[A-Za-z0-9._%-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$');

CREATE TABLE users (
    address t_address PRIMARY KEY,
    logo_nft t_address NULL,
    username VARCHAR(255) UNIQUE NULL,
    bio TEXT NULL,
    twitter VARCHAR(255) NULL,
    instagram VARCHAR(255) NULL,
    facebook VARCHAR(255) NULL,
    link VARCHAR(255) NULL,
    email t_email NULL
);