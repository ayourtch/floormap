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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1GetMapObjectsResponse {
    pub NextPollHorizon: flextimestamp::FlexTimestamp,
    pub MapObjects: Vec<ApiV1MapObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiV1MapObjectSetXYRecord {
    pub MapObjectUUID: flexuuid::FlexUuid,
    pub MapX: i32,
    pub MapY: i32,
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

pub fn api_get_map_objects_for_map(map_uuid: Uuid) -> ApiV1GetMapObjectsResponse {
    use super::schema::MapObjects::dsl::*;

    let db = get_db();
    let next_ts = flextimestamp::FlexTimestamp::now();
    let results = MapObjects
        // .filter(Deleted.eq(false)) // .and(AssetID.is_not_null()))
        .limit(2000)
        .load::<MapObject>(db.conn())
        .expect("Error loading services");

    let new_results: Vec<ApiV1MapObject> = results
        .into_iter()
        .map(|x| ApiV1MapObject { ..x })
        .collect();
    ApiV1GetMapObjectsResponse {
        NextPollHorizon: next_ts,
        MapObjects: new_results,
    }
}

pub fn db_set_mapobject_xy(
    mapobject_uuid: &FlexUuid,
    new_x: i32,
    new_y: i32,
) -> Result<bool, std::io::Error> {
    use std::io::{Error, ErrorKind};
    let res = db_get_mapobject(&mapobject_uuid);
    match res {
        Err(e) => {
            let msg = format!(
                "port {} - error setting description: {:?}",
                mapobject_uuid, e
            );
            return Err(Error::new(ErrorKind::NotFound, msg));
        }
        Ok(mo) => {
            if mo.Locked {
                let msg = format!("map object {} - is locked", &mapobject_uuid);
                return Err(Error::new(ErrorKind::Other, msg));
            }

            use schema::MapObjects::dsl::*;
            let db = get_db();
            let updated_row_res = diesel::update(
                MapObjects.filter(MapObjectUUID.eq(mapobject_uuid).and(Deleted.eq(false))),
            )
            .set((MapX.eq(new_x), MapY.eq(new_y)))
            .execute(db.conn());
            match updated_row_res {
                Err(e) => {
                    let msg = format!("mapobject {} - error updating XY: {:?}", mapobject_uuid, e);
                    return Err(Error::new(ErrorKind::Other, msg));
                }
                Ok(v) => {
                    // do nothing
                    return Ok(true);
                }
            }
        }
    }
}
