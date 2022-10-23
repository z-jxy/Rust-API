-- Your SQL goes here
CREATE TABLE errands (
	id          SERIAL PRIMARY KEY,
	created_at	TIMESTAMP WITH TIME ZONE NOT NULL,
	executed_at	TIMESTAMP WITH TIME ZONE NOT NULL,
	task        TEXT NOT NULL,
	args		JSONB NOT NULL,
	result      TEXT
);

CREATE INDEX index_errands_on_agent_id ON errands (agent_id)