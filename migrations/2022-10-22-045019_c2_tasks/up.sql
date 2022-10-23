-- Your SQL goes here
CREATE TABLE implants (
	id SERIAL PRIMARY KEY,
	name VARCHAR(50) NOT NULL,
	pid VARCHAR(50) NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    last_seen_at TIMESTAMP WITH TIME ZONE NOT NULL,
	ip VARCHAR(50) NULL
);

CREATE TABLE c2_tasks (
	id          SERIAL PRIMARY KEY,
	created_at	TIMESTAMP WITH TIME ZONE NOT NULL,
	executed_at	TIMESTAMP WITH TIME ZONE NOT NULL,
	task        TEXT NOT NULL,
	args		JSONB NOT NULL,
	result      TEXT,

    implant_id SERIAL NOT NULL REFERENCES implants(id) ON DELETE CASCADE
);

CREATE INDEX index_c2_tasks_on_implant_id ON c2_tasks (implant_id)