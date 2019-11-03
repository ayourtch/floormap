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

CREATE TABLE FloorMaps (
	FloorMapUUID VARCHAR(32) NOT NULL PRIMARY KEY,
	Deleted BOOLEAN NOT NULL,
	Name VARCHAR NOT NULL,
	Description VARCHAR NOT NULL,
	FloorPlanFileName VARCHAR NOT NULL,
	UpdatedAt datetime NOT NULL
);

INSERT INTO "FloorMaps" VALUES(
	'1e79ba6e-fb3a-11e9-b124-03c84357f69a',
	0,
	'Test Floor',
	'Pre-created basic test floor',
	'/var/a3s/http/floor-plan-images/1572806818/images/page-01.png',
	'2017-02-24 16:20:49.983'
);

CREATE TABLE MapObjects (
	MapObjectUUID VARCHAR(32) NOT NULL PRIMARY KEY,
	Deleted BOOLEAN NOT NULL,
	DeletedBy VARCHAR,
	DeletedAt datetime,
	Locked  BOOLEAN NOT NULL,
	LockedBy VARCHAR,
	LockedAt datetime,
	Name VARCHAR NOT NULL,
	Description VARCHAR NOT NULL,
	ParentMapUUID VARCHAR(32) NOT NULL,
	MapX INT NOT NULL,
	MapY INT NOT NULL,
	UpdatedAt datetime NOT NULL
);

INSERT INTO "MapObjects" VALUES (
	'4b06c4b4-fb3a-11e9-af57-fb611161d50b', 0, NULL, NULL,
	0, NULL, NULL,
	'Test object 1',
	'First test object',
	'1e79ba6e-fb3a-11e9-b124-03c84357f69a',
	10,10, '2017-02-24 16:20:49.983'
);

INSERT INTO "MapObjects" VALUES (
	'7392f6f0-fb3a-11e9-b567-633aa008f004', 0, NULL, NULL,
	0, NULL, NULL,
	'Test object 2',
	'second test object',
	'1e79ba6e-fb3a-11e9-b124-03c84357f69a',
	20,20, '2017-02-24 16:20:49.983'
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

