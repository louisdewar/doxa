-- TODO: consider indexes

CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    admin boolean NOT NULL default false,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    token_generation TEXT NOT NULL
);

CREATE TABLE competitions(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
    -- TODO: competition version - this can be used to reschedule matches in the event that the scoring system changes or there was some kind of error
    -- TODO: freeze mechanic to pause new uploads
    -- TODO: end date?
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
    uploaded_at timestamptz NOT NULL default now(),
    uploaded boolean NOT NULL default false,
    deleted boolean NOT NULL default false,
    failed boolean NOT NULL default false,
    active boolean NOT NULL default false
);

CREATE UNIQUE INDEX agents_active_unique ON agents (owner, competition)
    WHERE active;

-- CREATE INDEX ON agents((1)) WHERE active;

CREATE TABLE games(
    id SERIAL PRIMARY KEY,
    start_time timestamptz NOT NULL,
    complete_time timestamptz,
    competition INT references competitions(id) NOT NULL
);

CREATE TABLE game_participants(
    agent TEXT references agents(id) NOT NULL,
    game INT references games(id) NOT NULL,
    PRIMARY KEY (agent, game)
);

-- TODO: index on game_id, also on event_type and game_id
CREATE TABLE game_events(
    game INT references games(id) NOT NULL,
    -- ID within a particular game
    event_id INT NOT NULL,
    event_timestamp timestamptz NOT NULL,
    event_type TEXT NOT NULL,
    payload JSONB DEFAULT '{}'::jsonb NOT NULL,
    PRIMARY KEY (event_id, game)
);

CREATE TABLE leaderboard(
    agent TEXT references agents(id) PRIMARY KEY,
    score INT NOT NULL
);

-- CREATE VIEW active_agents AS
-- SELECT DISTINCT ON(competition, owner) *
-- FROM agents
-- WHERE deleted = false AND uploaded = true AND failed = false
-- ORDER BY competition, owner, uploaded_at DESC;

CREATE VIEW active_agents AS
SELECT *
FROM agents
WHERE active = true;

CREATE TABLE game_results(
    agent TEXT references agents(id) NOT NULL,
    game INT references games(id) NOT NULL,
    result INT NOT NULL,
    PRIMARY KEY (agent, game)
);
