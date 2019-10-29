-- Your SQL goes here


CREATE TABLE Comments (
	RecordUUID VARCHAR(32) NOT NULL PRIMARY KEY,
	Deleted BOOLEAN NOT NULL,
	ChangesetID INT NOT NULL,
	CommentID INT NOT NULL
);

CREATE TABLE Services (
	ServiceUUID VARCHAR(32) NOT NULL PRIMARY KEY,
	Deleted BOOLEAN NOT NULL,
	MenuOrder INT NOT NULL,
	ServiceName VARCHAR NOT NULL,
	ServiceLabel VARCHAR NOT NULL
);

CREATE TABLE Jobs (
	RecordUUID VARCHAR(32) NOT NULL PRIMARY KEY,
	JobGrouName VARCHAR NOT NULL,
	InstanceID INT NOT NULL,
	JobID VARCHAR NOT NULL,
	JobPID INT NOT NULL,
	ParentJobID VARCHAR,
	changeset_id INT NOT NULL,
	patchset_id INT NOT NULL,
	command VARCHAR NOT NULL,
	command_pid INT,
	remote_host VARCHAR,
	status_message VARCHAR NOT NULL,
	status_updated_at datetime,
	started_at datetime,
	finished_at datetime,
	return_success BOOLEAN NOT NULL,
	return_code INT,
	trigger_event_id VARCHAR
);

CREATE TABLE counters (
	name VARCHAR NOT NULL PRIMARY KEY,
	value INT NOT NULL
);

CREATE TABLE timestamps (
	name VARCHAR NOT NULL PRIMARY KEY,
	value datetime
);


