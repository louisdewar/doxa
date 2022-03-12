-- TODO: merge this into `initial`

ALTER TABLE agents
ADD COLUMN execution_environment TEXT NOT NULL DEFAULT 'basic',
ADD COLUMN file_size INT NOT NULL DEFAULT 0;

DROP VIEW active_agents;

CREATE VIEW active_agents AS
SELECT *
FROM agents
WHERE active = true;