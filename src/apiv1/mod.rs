use iron::prelude::*;
use iron::status;

use super::models::*;
use super::*;

use std::io::Read;
use std::sync::{Arc, Mutex};

use router::Router;

use chrono::{NaiveDate, NaiveDateTime};

use flextimestamp::FlexTimestamp;
use flexuuid::FlexUuid;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1ServiceRecord {
    pub ServiceUUID: flexuuid::FlexUuid,
    pub ServiceName: String,
    pub ServiceLabel: String,
    pub MenuOrder: i32,
}

type ApiV1MapObject = MapObject;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1FloorMap {
    pub FloorMapUUID: FlexUuid,
    pub Name: String,
    pub Description: String,
    pub Deleted: bool,
    pub ParentFloorPlanUUID: FlexUuid,
    pub SortOrder: i32,
    pub FileVersion: i32,
    pub ClipLeft: i32,
    pub ClipTop: i32,
    pub ClipWidth: i32,
    pub ClipHeight: i32,
    pub LegendLeft: i32,
    pub LegendTop: i32,
    pub LegendFontSize: i32,
}

type ApiV1Upload = Upload;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1FloorPlan {
    pub FloorPlanUUID: FlexUuid,
    pub Name: String,
    pub Description: String,
    pub Deleted: bool,
    pub Active: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1FloorMapSetClipRecord {
    pub FloorMapUUID: FlexUuid,
    pub ClipLeft: i32,
    pub ClipTop: i32,
    pub ClipWidth: i32,
    pub ClipHeight: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1FloorMapSetLegendRecord {
    pub FloorMapUUID: FlexUuid,
    pub LegendLeft: i32,
    pub LegendTop: i32,
    pub LegendFontSize: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1GetMapObjectsResponse {
    pub NextPollHorizon: i64,
    pub FloorPlans: Vec<ApiV1FloorPlan>,
    pub FloorMaps: Vec<ApiV1FloorMap>,
    pub MapObjects: Vec<ApiV1MapObject>,
    pub Uploads: Vec<ApiV1Upload>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1MapObjectSetXYRecord {
    pub MapObjectUUID: flexuuid::FlexUuid,
    pub MapX: i32,
    pub MapY: i32,
    pub ArrowX: i32,
    pub ArrowY: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1MapObjectDeleteRecord {
    pub MapObjectUUID: flexuuid::FlexUuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1FloorMapSetNameRecord {
    pub FloorMapUUID: flexuuid::FlexUuid,
    pub Name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1FloorMapDeleteRecord {
    pub FloorMapUUID: flexuuid::FlexUuid,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum ApiV1FloorMapCopyOperation {
    FloorMapOverwrite,
    FloorMapInsertBefore,
    FloorMapInsertAfter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1FloorMapCopyRecord {
    pub DstFloorMapUUID: flexuuid::FlexUuid,
    pub SrcFloorMapUUID: flexuuid::FlexUuid,
    pub Operation: ApiV1FloorMapCopyOperation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1MapObjectSetNameDescriptionRecord {
    pub MapObjectUUID: flexuuid::FlexUuid,
    pub TypeObjectUUID: Option<flexuuid::FlexUuid>,
    pub Name: String,
    pub Description: String,
    pub Meta: String,
    pub LabelSize: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1NewMapObjectRecord {
    pub Name: String,
    pub Description: String,
    pub MapX: i32,
    pub MapY: i32,
    pub LabelSize: i32,
    pub ParentMapUUID: FlexUuid,
    pub TypeObjectUUID: Option<flexuuid::FlexUuid>,
}

pub fn api_get_all_services() -> Vec<ApiV1ServiceRecord> {
    use super::schema::Services::dsl::*;

    let db = get_db();
    let results = Services
        .filter(Deleted.eq(false)) // .and(AssetID.is_not_null()))
        .order(MenuOrder.asc())
        .limit(2000)
        .load::<Service>(db.conn())
        .expect("Error loading services");

    let new_results: Vec<ApiV1ServiceRecord> = results
        .into_iter()
        .map(|x| {
            ApiV1ServiceRecord {
                ServiceUUID: x.ServiceUUID,
                ServiceName: x.ServiceName, // .unwrap_or("".to_owned()).clone(),
                ServiceLabel: x.ServiceLabel,
                MenuOrder: x.MenuOrder, // .unwrap_or(99999),
            }
        })
        .collect();
    new_results
}

pub fn api_get_map_objects(since: &FlexTimestamp) -> ApiV1GetMapObjectsResponse {
    let db = get_db();
    let next_ts = flextimestamp::FlexTimestamp::now().timestamp();

    let results = {
        use super::schema::MapObjects::dsl::*;
        MapObjects
            // .filter(Deleted.eq(false)) // .and(AssetID.is_not_null()))
            .filter(UpdatedAt.ge(since)) // .and(ParentMapUUID.eq(map_uuid)))
            .order(UpdatedAt.asc())
            .limit(20000)
            .load::<MapObject>(db.conn())
            .expect("Error loading mapobjects")
    };
    let floormaps = {
        use super::schema::FloorMaps::dsl::*;
        FloorMaps
            // .filter(Deleted.eq(false)) // .and(AssetID.is_not_null()))
            .filter(UpdatedAt.ge(since)) // .and(ParentMapUUID.eq(map_uuid)))
            // .order(SortOrder.asc())
            .order(UpdatedAt.asc())
            .limit(20000)
            .load::<FloorMap>(db.conn())
            .expect("Error loading floormaps")
    };
    let floorplans = {
        use super::schema::FloorPlans::dsl::*;
        FloorPlans
            // .filter(Deleted.eq(false)) // .and(AssetID.is_not_null()))
            .filter(UpdatedAt.ge(since)) // .and(ParentMapUUID.eq(map_uuid)))
            // .order(SortOrder.asc())
            .order(UpdatedAt.asc())
            .limit(20000)
            .load::<FloorPlan>(db.conn())
            .expect("Error loading floorplans")
    };
    let uploads = {
        use super::schema::Uploads::dsl::*;
        Uploads
            .filter(UpdatedAt.ge(since)) // .and(ParentMapUUID.eq(map_uuid)))
            .order(UpdatedAt.asc())
            .limit(20000)
            .load::<Upload>(db.conn())
            .expect("Error loading uploads")
    };

    let new_results: Vec<ApiV1MapObject> = results
        .into_iter()
        .map(|x| ApiV1MapObject { ..x })
        .collect();

    let new_floormaps = floormaps
        .into_iter()
        .map(|x| ApiV1FloorMap {
            FloorMapUUID: x.FloorMapUUID,
            ParentFloorPlanUUID: x.ParentFloorPlanUUID,
            Name: x.Name,
            Description: x.Description,
            Deleted: x.Deleted,
            SortOrder: x.SortOrder,
            FileVersion: x.FloorMapFileVersion,
            ClipLeft: x.ClipLeft,
            ClipTop: x.ClipTop,
            ClipWidth: x.ClipWidth,
            ClipHeight: x.ClipHeight,
            LegendLeft: x.LegendLeft,
            LegendTop: x.LegendTop,
            LegendFontSize: x.LegendFontSize,
        })
        .collect();
    let new_floorplans = floorplans
        .into_iter()
        .map(|x| ApiV1FloorPlan {
            FloorPlanUUID: x.FloorPlanUUID,
            Name: x.Name,
            Description: x.Description,
            Deleted: x.Deleted,
            Active: x.Active,
        })
        .collect();
    ApiV1GetMapObjectsResponse {
        NextPollHorizon: next_ts,
        FloorPlans: new_floorplans,
        FloorMaps: new_floormaps,
        MapObjects: new_results,
        Uploads: uploads,
    }
}
