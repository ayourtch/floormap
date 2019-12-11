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

CREATE TABLE FloorPlans (
	FloorPlanUUID VARCHAR(32) NOT NULL PRIMARY KEY,
	Deleted BOOLEAN NOT NULL,
	Name VARCHAR NOT NULL,
	Description VARCHAR NOT NULL,
	ParentFloorPlanUUID VARCHAR(32),
	FloorPlanPath VARCHAR NOT NULL,
	CreatedAt datetime NOT NULL,
	UpdatedAt datetime NOT NULL
);

CREATE TABLE FloorMaps (
	FloorMapUUID VARCHAR(32) NOT NULL PRIMARY KEY,
	Deleted BOOLEAN NOT NULL,
	Name VARCHAR NOT NULL,
	Description VARCHAR NOT NULL,
	FullText VARCHAR NOT NULL,
	ParentFloorPlanUUID VARCHAR(32) NOT NULL,
	FloorPlanFileName VARCHAR NOT NULL,
	Locked  BOOLEAN NOT NULL,
	LockedBy VARCHAR,
	LockedAt datetime,
	ClipLeft INT NOT NULL,
	ClipTop INT NOT NULL,
	ClipWidth INT NOT NULL,
	ClipHeight INT NOT NULL,
	UpdatedAt datetime NOT NULL
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
	LabelSize INT NOT NULL,
	Description VARCHAR NOT NULL,
	ParentMapUUID VARCHAR(32) NOT NULL,
	TypeObjectUUID VARCHAR(32),
	MapX INT NOT NULL,
	MapY INT NOT NULL,
	ArrowX INT NOT NULL,
	ArrowY INT NOT NULL,
	UpdatedAt datetime NOT NULL
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

