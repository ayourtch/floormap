
use super::models::*;
use super::*;

use std::io::Read;
use std::sync::{Arc, Mutex};

use router::Router;

use chrono::{NaiveDate, NaiveDateTime};

use flextimestamp::FlexTimestamp;
use flexuuid::FlexUuid;
use uuid::Uuid;

pub fn db_insert_new_mapobject(
    mapfloor_uuid: &FlexUuid,
    new_name: &str,
    new_description: &str,
    new_x: i32,
    new_y: i32,
) -> FlexUuid {
    use self::diesel::prelude::*;
    use schema::MapObjects::dsl::*;

    let new_item = MapObject {
        Name: new_name.to_string(),
        Description: new_description.to_string(),
        MapX: new_x,
        MapY: new_y,
        ParentMapUUID: mapfloor_uuid.clone(),

        ..Default::default()
    };

    let db = get_db();
    let rows_inserted = diesel::insert_into(MapObjects)
        .values(&new_item)
        .execute(db.conn());
    new_item.MapObjectUUID
}

pub fn db_set_mapobject_xy(
    mapobject_uuid: &FlexUuid,
    new_x: i32,
    new_y: i32,
    user: &str,
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
            let now_ts = FlexTimestamp::now();
            let updated_row_res = diesel::update(
                MapObjects.filter(MapObjectUUID.eq(mapobject_uuid).and(Deleted.eq(false))),
            )
            .set((MapX.eq(new_x), MapY.eq(new_y), UpdatedAt.eq(now_ts)))
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

pub fn db_set_mapobject_name_description(
    mapobject_uuid: &FlexUuid,
    new_name: &str,
    new_description: &str,
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
            let now_ts = FlexTimestamp::now();
            let updated_row_res = diesel::update(
                MapObjects.filter(MapObjectUUID.eq(mapobject_uuid).and(Deleted.eq(false))),
            )
            .set((
                Name.eq(new_name),
                Description.eq(new_description),
                UpdatedAt.eq(now_ts),
            ))
            .execute(db.conn());
            match updated_row_res {
                Err(e) => {
                    let msg = format!(
                        "mapobject {} - error updating name/desc: {:?}",
                        mapobject_uuid, e
                    );
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
