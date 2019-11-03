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
pub struct ApiV1GetMapObjectsResponse {
    pub NextPollHorizon: i64,
    pub MapUUID: FlexUuid,
    pub MapObjects: Vec<ApiV1MapObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1MapObjectSetXYRecord {
    pub MapObjectUUID: flexuuid::FlexUuid,
    pub MapX: i32,
    pub MapY: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1MapObjectSetNameDescriptionRecord {
    pub MapObjectUUID: flexuuid::FlexUuid,
    pub Name: String,
    pub Description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1NewMapObjectRecord {
    pub Name: String,
    pub MapX: i32,
    pub MapY: i32,
    pub ParentMapUUID: FlexUuid,
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

pub fn api_get_map_objects_for_map(
    map_uuid: &FlexUuid,
    since: &FlexTimestamp,
) -> ApiV1GetMapObjectsResponse {
    use super::schema::MapObjects::dsl::*;

    let db = get_db();
    let next_ts = flextimestamp::FlexTimestamp::now().timestamp();
    let results = MapObjects
        // .filter(Deleted.eq(false)) // .and(AssetID.is_not_null()))
        .filter(UpdatedAt.ge(since).and(ParentMapUUID.eq(map_uuid)))
        .limit(2000)
        .load::<MapObject>(db.conn())
        .expect("Error loading services");

    let new_results: Vec<ApiV1MapObject> = results
        .into_iter()
        .map(|x| ApiV1MapObject { ..x })
        .collect();
    ApiV1GetMapObjectsResponse {
        NextPollHorizon: next_ts,
        MapUUID: map_uuid.clone(),
        MapObjects: new_results,
    }
}

