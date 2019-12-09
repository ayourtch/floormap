use super::models::*;
use super::*;

use std::io::Read;
use std::sync::{Arc, Mutex};

use router::Router;

use chrono::{NaiveDate, NaiveDateTime};

use flextimestamp::FlexTimestamp;
use flexuuid::FlexUuid;
use uuid::Uuid;

#[serde(default)]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct DbExport {
    pub FloorPlans: Vec<FloorPlan>,
    pub FloorMaps: Vec<FloorMap>,
    pub MapObjects: Vec<MapObject>,
}

pub fn db_put_json(data: &str) {
    let db = get_db();
    let bk: DbExport = serde_json::from_str(data).unwrap();
    {
        use self::diesel::prelude::*;
        use schema::FloorPlans::dsl::*;

        for d in &bk.FloorPlans {
            let rows_inserted = diesel::insert_into(FloorPlans).values(d).execute(db.conn());
        }
    }
    {
        use self::diesel::prelude::*;
        use schema::FloorMaps::dsl::*;

        for d in &bk.FloorMaps {
            let rows_inserted = diesel::insert_into(FloorMaps).values(d).execute(db.conn());
        }
    }
    {
        use self::diesel::prelude::*;
        use schema::MapObjects::dsl::*;

        for d in &bk.MapObjects {
            let rows_inserted = diesel::insert_into(MapObjects).values(d).execute(db.conn());
        }
    }
}

pub fn db_get_json() -> String {
    let db = get_db();
    let result_floor_plans = {
        use super::schema::FloorPlans::dsl::*;
        FloorPlans
            .filter(Deleted.eq(false)) // .and(AssetID.is_not_null()))
            // .filter(UpdatedAt.ge(since)) // .and(ParentMapUUID.eq(map_uuid)))
            .limit(20000)
            .load::<FloorPlan>(db.conn())
            .expect("Error loading floorplan")
    };
    let result_floor_maps = {
        use super::schema::FloorMaps::dsl::*;
        FloorMaps
            .filter(Deleted.eq(false)) // .and(AssetID.is_not_null()))
            // .filter(UpdatedAt.ge(since)) // .and(ParentMapUUID.eq(map_uuid)))
            .limit(20000)
            .load::<FloorMap>(db.conn())
            .expect("Error loading floormaps")
    };
    let result_map_objects = {
        use super::schema::MapObjects::dsl::*;
        MapObjects
            .filter(Deleted.eq(false)) // .and(AssetID.is_not_null()))
            // .filter(UpdatedAt.ge(since)) // .and(ParentMapUUID.eq(map_uuid)))
            .limit(20000)
            .load::<MapObject>(db.conn())
            .expect("Error loading mapobjects")
    };
    let results = DbExport {
        FloorPlans: result_floor_plans,
        FloorMaps: result_floor_maps,
        MapObjects: result_map_objects,
    };

    let j = serde_json::to_string_pretty(&results).unwrap();
    j
}

pub fn db_insert_new_floorplan(
    new_name: &str,
    new_description: &str,
    new_path: &str,
    new_parent: Option<&FlexUuid>,
) -> FlexUuid {
    use self::diesel::prelude::*;
    use schema::FloorPlans::dsl::*;

    let new_item = FloorPlan {
        Name: new_name.to_string(),
        Description: new_description.to_string(),
        FloorPlanPath: new_path.to_string(),
        ParentFloorPlanUUID: new_parent.map(|x| x.clone()),
        CreatedAt: FlexTimestamp::now(),
        ..Default::default()
    };

    let db = get_db();
    let rows_inserted = diesel::insert_into(FloorPlans)
        .values(&new_item)
        .execute(db.conn());
    new_item.FloorPlanUUID
}

pub fn db_insert_new_floormap(
    new_name: &str,
    new_description: &str,
    new_full_text: &str,
    new_filename: &str,
    new_parent: &FlexUuid,
) -> FlexUuid {
    use self::diesel::prelude::*;
    use schema::FloorMaps::dsl::*;

    let new_item = FloorMap {
        Name: new_name.to_string(),
        Description: new_description.to_string(),
        FullText: new_full_text.to_string(),
        FloorPlanFileName: new_filename.to_string(),
        ParentFloorPlanUUID: new_parent.clone(),
        ..Default::default()
    };

    let db = get_db();
    let rows_inserted = diesel::insert_into(FloorMaps)
        .values(&new_item)
        .execute(db.conn());
    new_item.FloorMapUUID
}

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

pub fn db_set_mapobject_labelsize(
    mapobject_uuid: &FlexUuid,
    labelsize: i32,
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
            .set((LabelSize.eq(labelsize), UpdatedAt.eq(now_ts)))
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
