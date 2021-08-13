CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    admin boolean NOT NULL default false,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

CREATE TABLE competitions(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE enrollment(
    user_id INT references users(id) NOT NULL,
    competition INT references competitions(id) NOT NULL,
    PRIMARY KEY (user_id, competition)
);

CREATE TABLE agents(
    id TEXT PRIMARY KEY,
    owner INT REFERENCES users(id) NOT NULL,
    competition INT references competitions(id) NOT NULL,
    uploaded boolean NOT NULL default false,
    deleted boolean NOT NULL default false,
    failed boolean NOT NULL default false
);

