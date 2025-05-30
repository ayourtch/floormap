use std; use serde::{Serialize, Deserialize}; use chrono; use super::flexuuid::FlexUuid; use super::flextimestamp::FlexTimestamp;  use diesel::connection::TransactionManager; use diesel::pg::PgConnection; use diesel::prelude::*; use diesel::sqlite::SqliteConnection; use crate::schema; use schema::*;
// Generated by diesel_ext



use chrono::NaiveDateTime;
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "Comments"]
#[serde(default)]
pub struct Comment {
    pub RecordUUID: FlexUuid,
    pub Deleted: bool,
    pub ChangesetID: i32,
    pub CommentID: i32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "FloorMaps"]
#[serde(default)]
pub struct FloorMap {
    pub FloorMapUUID: FlexUuid,
    pub Deleted: bool,
    pub Name: String,
    pub Description: String,
    pub FullText: String,
    pub ParentFloorPlanUUID: FlexUuid,
    pub FloorMapFileName: String,
    pub FloorMapFileVersion: i32,
    pub Locked: bool,
    pub LockedBy: Option<String>,
    pub LockedAt: Option<FlexTimestamp>,
    pub SortOrder: i32,
    pub ClipLeft: i32,
    pub ClipTop: i32,
    pub ClipWidth: i32,
    pub ClipHeight: i32,
    pub LegendTop: i32,
    pub LegendLeft: i32,
    pub LegendFontSize: i32,
    pub UpdatedAt: FlexTimestamp,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "FloorPlans"]
#[serde(default)]
pub struct FloorPlan {
    pub FloorPlanUUID: FlexUuid,
    pub Deleted: bool,
    pub Active: bool,
    pub Name: String,
    pub Description: String,
    pub ParentFloorPlanUUID: Option<FlexUuid>,
    pub FloorPlanPath: String,
    pub CreatedAt: FlexTimestamp,
    pub UpdatedAt: FlexTimestamp,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "Jobs"]
#[serde(default)]
pub struct Job {
    pub RecordUUID: FlexUuid,
    pub JobGrouName: String,
    pub InstanceID: i32,
    pub JobID: String,
    pub JobPID: i32,
    pub ParentJobID: Option<String>,
    pub changeset_id: i32,
    pub patchset_id: i32,
    pub command: String,
    pub command_pid: Option<i32>,
    pub remote_host: Option<String>,
    pub status_message: String,
    pub status_updated_at: Option<FlexTimestamp>,
    pub started_at: Option<FlexTimestamp>,
    pub finished_at: Option<FlexTimestamp>,
    pub return_success: bool,
    pub return_code: Option<i32>,
    pub trigger_event_id: Option<String>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "Logs"]
#[serde(default)]
pub struct Log {
    pub LogUUID: FlexUuid,
    pub LogTimestamp: FlexTimestamp,
    pub Key1: i32,
    pub Key2: i32,
    pub Key3UUID: FlexUuid,
    pub Key4UUID: FlexUuid,
    pub Username: String,
    pub Source: String,
    pub Message: String,
    pub Data1: String,
    pub Data2: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "MapObjects"]
#[serde(default)]
pub struct MapObject {
    pub MapObjectUUID: FlexUuid,
    pub Deleted: bool,
    pub DeletedBy: Option<String>,
    pub DeletedAt: Option<FlexTimestamp>,
    pub Locked: bool,
    pub LockedBy: Option<String>,
    pub LockedAt: Option<FlexTimestamp>,
    pub Name: String,
    pub LabelSize: i32,
    pub Description: String,
    pub Meta: String,
    pub ParentMapUUID: FlexUuid,
    pub TypeObjectUUID: Option<FlexUuid>,
    pub MapX: i32,
    pub MapY: i32,
    pub ArrowX: i32,
    pub ArrowY: i32,
    pub UpdatedAt: FlexTimestamp,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "Services"]
#[serde(default)]
pub struct Service {
    pub ServiceUUID: FlexUuid,
    pub Deleted: bool,
    pub MenuOrder: i32,
    pub ServiceName: String,
    pub ServiceLabel: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "Uploads"]
#[serde(default)]
pub struct Upload {
    pub UploadUUID: FlexUuid,
    pub RelatedToUUID: FlexUuid,
    pub Deleted: bool,
    pub CreatedBy: String,
    pub CreatedAt: FlexTimestamp,
    pub UpdatedAt: FlexTimestamp,
    pub OriginalFileName: String,
    pub ServerFileName: String,
    pub ServerFileSize: i32,
    pub MimeType: String,
    pub Checksum: String,
    pub ChecksumType: String,
    pub Key1: i32,
    pub Key2: i32,
    pub Key3UUID: FlexUuid,
    pub Key4UUID: FlexUuid,
    pub Message: String,
    pub Data1: String,
    pub Data2: String,
}

