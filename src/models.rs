use super::flextimestamp::FlexTimestamp;
use super::flexuuid::FlexUuid;
use crate::schema;
use chrono;
use diesel;
use diesel::connection::TransactionManager;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use schema::*;
use serde_derive;
use std;
#[serde(default)]
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "Comments"]
pub struct Comment {
    pub RecordUUID: FlexUuid,
    pub Deleted: bool,
    pub ChangesetID: i32,
    pub CommentID: i32,
}

#[serde(default)]
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "FloorMaps"]
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

#[serde(default)]
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "FloorPlans"]
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

#[serde(default)]
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "Jobs"]
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

#[serde(default)]
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "Logs"]
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

#[serde(default)]
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "MapObjects"]
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

#[serde(default)]
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "Services"]
pub struct Service {
    pub ServiceUUID: FlexUuid,
    pub Deleted: bool,
    pub MenuOrder: i32,
    pub ServiceName: String,
    pub ServiceLabel: String,
}

#[serde(default)]
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table_name = "Uploads"]
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
