-- Your SQL goes here
CREATE TABLE agents(
	id SERIAL PRIMARY KEY,
	agent_id VARCHAR(50) NOT NULL,
	agent_pid VARCHAR(50) NOT NULL,
	agent_ip VARCHAR(50) NOT NULL
);