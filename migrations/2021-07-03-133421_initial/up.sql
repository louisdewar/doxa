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
    extension TEXT NOT NULL,
    uploaded_at timestamp NOT NULL default now(),
    uploaded boolean NOT NULL default false,
    deleted boolean NOT NULL default false,
    failed boolean NOT NULL default false
);

CREATE TABLE games(
    id SERIAL PRIMARY KEY,
    start_time timestamp NOT NULL,
    complete_time timestamp,
    competition INT references competitions(id) NOT NULL
);

CREATE TABLE game_participants(
    agent TEXT references agents(id) NOT NULL,
    game INT references games(id) NOT NULL,
    PRIMARY KEY (agent, game)
);

CREATE TABLE game_events(
    -- ID within a particular game
    event_id INT NOT NULL,
    game INT references games(id) NOT NULL,
    event_timestamp timestamp NOT NULL,
    event_type TEXT NOT NULL,
    payload JSONB DEFAULT '{}'::jsonb NOT NULL,
    PRIMARY KEY (event_id, game)
);
