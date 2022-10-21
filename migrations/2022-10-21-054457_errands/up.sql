-- Your SQL goes here
CREATE TABLE errands (
	id          SERIAL PRIMARY KEY,
	task        VARCHAR(200) NOT NULL,
	result      VARCHAR(200) NOT NULL,
	time_stamp  VARCHAR(20) NOT NULL
);