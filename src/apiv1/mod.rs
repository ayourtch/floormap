use iron::prelude::*;
use iron::status;

use super::models::*;
use super::*;

use std::io::Read;
use std::sync::{Arc, Mutex};

use router::Router;

use chrono::{NaiveDate, NaiveDateTime};

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1ServiceRecord {
    pub ServiceUUID: flexuuid::FlexUuid,
    pub ServiceName: String,
    pub ServiceLabel: String,
    pub MenuOrder: i32,
}

type ApiV1MapObject = MapObject;

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

pub fn api_get_map_objects_for_map(map_uuid: Uuid) -> Vec<ApiV1MapObject> {
    use super::schema::MapObjects::dsl::*;

    let db = get_db();
    let results = MapObjects
        .filter(Deleted.eq(false)) // .and(AssetID.is_not_null()))
        .limit(2000)
        .load::<MapObject>(db.conn())
        .expect("Error loading services");

    let new_results: Vec<ApiV1MapObject> = results
        .into_iter()
        .map(|x| ApiV1MapObject { ..x })
        .collect();
    new_results
}
