use super::models::*;
use super::*;

use std::io::Read;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

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
    pub Uploads: Vec<Upload>,
}

pub fn split_meta_from_str(str: &str) -> (String, String) {
    use regex::{Captures, Regex};
    lazy_static! {
        static ref RE_TILDE: Regex = Regex::new(r"~(?P<meta>[^~]*)~").unwrap();
    }
    let meta_maybe = RE_TILDE.captures(str);
    if let Some(meta_yes) = meta_maybe {
        let meta = meta_yes.name("meta").unwrap().as_str();
        let remainder = RE_TILDE.replace_all(str, "");
        return (meta.to_string(), remainder.to_string());
    } else {
        return (format!(""), str.to_string());
    }
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
            let mut d = d.clone();
            if d.SortOrder == 0 {
                let new_sort_order = 1 + {
                    use self::diesel::dsl::count_star;
                    *FloorMaps
                        .filter(ParentFloorPlanUUID.eq(&d.ParentFloorPlanUUID))
                        .select(count_star())
                        .load::<i64>(db.conn())
                        .expect("Error loading floormaps")
                        .first()
                        .unwrap()
                };

                let new_sort_order_i32: i32 = new_sort_order as i32;
                d.SortOrder = new_sort_order_i32;
            }
            let rows_inserted = diesel::insert_into(FloorMaps).values(&d).execute(db.conn());
        }
    }
    {
        use self::diesel::prelude::*;
        use schema::MapObjects::dsl::*;

        for d in &bk.MapObjects {
            let mut d = d.clone();
            if (&d.Meta == "") {
                let (m, desc) = split_meta_from_str(&d.Description);
                d.Meta = m;
                d.Description = desc;
            }
            let rows_inserted = diesel::insert_into(MapObjects)
                .values(&d)
                .execute(db.conn());
        }
    }
    {
        use self::diesel::prelude::*;
        use schema::Uploads::dsl::*;

        for d in &bk.Uploads {
            let rows_inserted = diesel::insert_into(Uploads).values(d).execute(db.conn());
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
    let result_uploads = {
        use super::schema::Uploads::dsl::*;
        Uploads
            .filter(Deleted.eq(false)) // .and(AssetID.is_not_null()))
            // .filter(UpdatedAt.ge(since)) // .and(ParentMapUUID.eq(map_uuid)))
            .limit(20000)
            .load::<Upload>(db.conn())
            .expect("Error loading upload")
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
        Uploads: result_uploads,
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

pub fn db_set_floormap_file(
    floormap_uuid: &FlexUuid,
    floormap_filename: &str,
) -> Result<bool, std::io::Error> {
    let res = db_get_floormap(&floormap_uuid);
    match res {
        Err(e) => {
            let msg = format!(
                "floormap {} - error setting floormap: {:?}",
                floormap_uuid, e
            );
            return Err(Error::new(ErrorKind::NotFound, msg));
        }
        Ok(mo) => {
            if mo.Locked {
                let msg = format!("map object {} - is locked", &floormap_uuid);
                return Err(Error::new(ErrorKind::Other, msg));
            }
            let plan_res = db_get_floorplan(&mo.ParentFloorPlanUUID);
            if let Err(e) = plan_res {
                let msg = format!(
                    "floormap {} - error getting floorplan {}: {:?}",
                    floormap_uuid, &mo.ParentFloorPlanUUID, e
                );
                return Err(Error::new(ErrorKind::NotFound, msg));
            }
            let floorplan = plan_res.unwrap();
            let file_version = mo.FloorMapFileVersion + 1;
            let dst_file_name = format!(
                "{}/{}-{}.png",
                &floorplan.FloorPlanPath, floormap_uuid, file_version
            );
            let copy_res = std::fs::copy(floormap_filename, &dst_file_name);
            if let Err(e) = copy_res {
                let msg = format!(
                    "floormap {} - error copying file {} to {}: {:?}",
                    floormap_uuid, floormap_filename, &dst_file_name, e
                );
                return Err(Error::new(ErrorKind::NotFound, msg));
            }
            let dst_thumb_file_name = format!(
                "{}/{}-{}.png-thumb.png",
                &floorplan.FloorPlanPath, floormap_uuid, file_version
            );
            let floormap_thumb_filename = format!("{}-thumb.png", floormap_filename);
            let copy_res = std::fs::copy(&floormap_thumb_filename, &dst_thumb_file_name);
            if let Err(e) = copy_res {
                let msg = format!(
                    "floormap {} - error copying file {} to {}: {:?}",
                    floormap_uuid, floormap_thumb_filename, &dst_thumb_file_name, e
                );
                return Err(Error::new(ErrorKind::NotFound, msg));
            }

            use schema::FloorMaps::dsl::*;
            let db = get_db();
            let now_ts = FlexTimestamp::now();
            let updated_row_res = diesel::update(FloorMaps.filter(FloorMapUUID.eq(floormap_uuid)))
                .set((
                    FloorMapFileName.eq(dst_file_name),
                    FloorMapFileVersion.eq(file_version),
                    UpdatedAt.eq(&now_ts),
                ))
                .execute(db.conn());
            match updated_row_res {
                Err(e) => {
                    let msg = format!("floormap {} - error updating XY: {:?}", floormap_uuid, e);
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

pub fn db_set_floormap_deleted(
    floormap_uuid: &FlexUuid,
    deleted: bool,
) -> Result<bool, std::io::Error> {
    use std::io::{Error, ErrorKind};
    let res = db_get_floormap(&floormap_uuid);
    match res {
        Err(e) => {
            let msg = format!(
                "port {} - error setting description: {:?}",
                floormap_uuid, e
            );
            return Err(Error::new(ErrorKind::NotFound, msg));
        }
        Ok(mo) => {
            if mo.Locked {
                let msg = format!("map object {} - is locked", &floormap_uuid);
                return Err(Error::new(ErrorKind::Other, msg));
            }

            use schema::FloorMaps::dsl::*;
            let db = get_db();
            let now_ts = FlexTimestamp::now();
            let updated_row_res = diesel::update(FloorMaps.filter(FloorMapUUID.eq(floormap_uuid)))
                .set((
                    Deleted.eq(true),
                    // DeletedAt.eq(&now_ts),
                    UpdatedAt.eq(&now_ts),
                ))
                .execute(db.conn());
            match updated_row_res {
                Err(e) => {
                    let msg = format!("floormap {} - error updating XY: {:?}", floormap_uuid, e);
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

pub fn db_set_floormap_name(floormap_uuid: &FlexUuid, name: &str) -> Result<bool, std::io::Error> {
    use std::io::{Error, ErrorKind};
    let res = db_get_floormap(&floormap_uuid);
    match res {
        Err(e) => {
            let msg = format!(
                "port {} - error setting description: {:?}",
                floormap_uuid, e
            );
            return Err(Error::new(ErrorKind::NotFound, msg));
        }
        Ok(mo) => {
            if mo.Locked {
                let msg = format!("map object {} - is locked", &floormap_uuid);
                return Err(Error::new(ErrorKind::Other, msg));
            }

            use schema::FloorMaps::dsl::*;
            let db = get_db();
            let now_ts = FlexTimestamp::now();
            let updated_row_res = diesel::update(FloorMaps.filter(FloorMapUUID.eq(floormap_uuid)))
                .set((
                    Name.eq(name),
                    // DeletedAt.eq(&now_ts),
                    UpdatedAt.eq(&now_ts),
                ))
                .execute(db.conn());
            match updated_row_res {
                Err(e) => {
                    let msg = format!("floormap {} - error setting name: {:?}", floormap_uuid, e);
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

pub fn db_insert_new_floormap(
    new_name: &str,
    new_description: &str,
    new_full_text: &str,
    new_filename: &str,
    new_parent: &FlexUuid,
    insert_before_order: i32,
) -> FlexUuid {
    use self::diesel::dsl::count_star;
    use self::diesel::prelude::*;
    use schema::FloorMaps::dsl::*;
    let db = get_db();
    let now_ts = FlexTimestamp::now();

    let new_sort_order = 1 + {
        use super::schema::FloorMaps::dsl::*;
        *FloorMaps
            .filter(
                SortOrder
                    .lt(insert_before_order)
                    .and(ParentFloorPlanUUID.eq(new_parent)),
            )
            .select(count_star())
            .load::<i64>(db.conn())
            .expect("Error loading floormaps")
            .first()
            .unwrap()
    };

    let new_sort_order_i32: i32 = new_sort_order as i32;

    let updated_row_res = diesel::update(
        FloorMaps.filter(
            ParentFloorPlanUUID
                .eq(new_parent)
                .and(SortOrder.ge(new_sort_order_i32)),
        ),
    )
    .set((SortOrder.eq(SortOrder + 1), UpdatedAt.eq(now_ts)))
    .execute(db.conn());
    println!("Updated row res: {:?}", updated_row_res);

    let new_item = FloorMap {
        Name: new_name.to_string(),
        Description: new_description.to_string(),
        FullText: new_full_text.to_string(),
        FloorMapFileName: new_filename.to_string(),
        ParentFloorPlanUUID: new_parent.clone(),
        SortOrder: new_sort_order_i32,
        ..Default::default()
    };

    let rows_inserted = diesel::insert_into(FloorMaps)
        .values(&new_item)
        .execute(db.conn());
    new_item.FloorMapUUID
}

pub fn db_set_floormap_clip(
    floormap_uuid: &FlexUuid,
    new_left: i32,
    new_top: i32,
    new_width: i32,
    new_height: i32,
) -> Result<bool, std::io::Error> {
    use std::io::{Error, ErrorKind};
    let res = db_get_floormap(&floormap_uuid);
    match res {
        Err(e) => {
            let msg = format!("floormap {} - error setting clip: {:?}", floormap_uuid, e);
            return Err(Error::new(ErrorKind::NotFound, msg));
        }
        Ok(mo) => {
            if mo.Locked {
                let msg = format!("floormap {} - is locked", &floormap_uuid);
                return Err(Error::new(ErrorKind::Other, msg));
            }

            use schema::FloorMaps::dsl::*;
            let db = get_db();
            let now_ts = FlexTimestamp::now();
            let updated_row_res = diesel::update(
                FloorMaps.filter(FloorMapUUID.eq(floormap_uuid).and(Deleted.eq(false))),
            )
            .set((
                ClipLeft.eq(new_left),
                ClipTop.eq(new_top),
                ClipWidth.eq(new_width),
                ClipHeight.eq(new_height),
                UpdatedAt.eq(now_ts),
            ))
            .execute(db.conn());
            match updated_row_res {
                Err(e) => {
                    let msg = format!(
                        "floormap_uuid {} - error updating XY: {:?}",
                        floormap_uuid, e
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

pub fn db_set_floormap_legend(
    floormap_uuid: &FlexUuid,
    new_left: i32,
    new_top: i32,
    new_fontsize: i32,
) -> Result<bool, std::io::Error> {
    use std::io::{Error, ErrorKind};
    let res = db_get_floormap(&floormap_uuid);
    match res {
        Err(e) => {
            let msg = format!("floormap {} - error setting legend: {:?}", floormap_uuid, e);
            return Err(Error::new(ErrorKind::NotFound, msg));
        }
        Ok(mo) => {
            if mo.Locked {
                let msg = format!("floormap {} - is locked", &floormap_uuid);
                return Err(Error::new(ErrorKind::Other, msg));
            }

            use schema::FloorMaps::dsl::*;
            let db = get_db();
            let now_ts = FlexTimestamp::now();
            let updated_row_res = diesel::update(
                FloorMaps.filter(FloorMapUUID.eq(floormap_uuid).and(Deleted.eq(false))),
            )
            .set((
                LegendLeft.eq(new_left),
                LegendTop.eq(new_top),
                LegendFontSize.eq(new_fontsize),
                UpdatedAt.eq(now_ts),
            ))
            .execute(db.conn());
            match updated_row_res {
                Err(e) => {
                    let msg = format!(
                        "floormap_uuid {} - error updating legend: {:?}",
                        floormap_uuid, e
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

pub fn db_insert_new_upload(
    username: &str,
    upload_for_uuid: &FlexUuid,
    temp_filename: &str,
    original_filename: &str,
    upload_comments: &str,
) -> Option<FlexUuid> {
    use self::diesel::prelude::*;
    use schema::Uploads::dsl::*;

    let upload_uuid = FlexUuid::default();
    let dir_name = format!("{}/{}", upload_for_uuid, &upload_uuid);
    let full_dir_name = format!("/var/a3s/http/uploads/{}/{}", upload_for_uuid, &upload_uuid);
    let dest_name = format!("{}/{}", &dir_name, original_filename);
    let full_dest_name = format!("{}/{}", &full_dir_name, original_filename);

    let mkdir_res = std::fs::create_dir_all(&full_dir_name);
    let copy_res = std::fs::copy(&temp_filename, &full_dest_name);
    println!(
        "Mkdir result: {:?}, copy result: {:?}",
        &mkdir_res, &copy_res
    );

    if mkdir_res.is_ok() && copy_res.is_ok() {
        let new_item = Upload {
            UploadUUID: upload_uuid.clone(),
            RelatedToUUID: upload_for_uuid.clone(),
            CreatedBy: username.to_string(),
            OriginalFileName: original_filename.to_string(),
            ServerFileName: dest_name.clone(),
            Message: upload_comments.to_string(),
            ..Default::default()
        };

        let db = get_db();
        let rows_inserted = diesel::insert_into(Uploads)
            .values(&new_item)
            .execute(db.conn());
        Some(new_item.UploadUUID)
    } else {
        None
    }
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

pub fn db_set_mapobject_arrow_xy(
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
            .set((ArrowX.eq(new_x), ArrowY.eq(new_y), UpdatedAt.eq(now_ts)))
            .execute(db.conn());
            match updated_row_res {
                Err(e) => {
                    let msg = format!(
                        "mapobject {} - error updating ArrowXY: {:?}",
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

pub fn db_set_mapobject_deleted(
    mapobject_uuid: &FlexUuid,
    deleted: bool,
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
            let updated_row_res =
                diesel::update(MapObjects.filter(MapObjectUUID.eq(mapobject_uuid)))
                    .set((
                        Deleted.eq(true),
                        DeletedAt.eq(&now_ts),
                        UpdatedAt.eq(&now_ts),
                    ))
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

pub fn db_set_mapobject_name_description_meta(
    mapobject_uuid: &FlexUuid,
    new_name: &str,
    new_description: &str,
    new_meta: &str,
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
                Meta.eq(new_meta),
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

pub fn db_set_mapobject_typeobjectuuid(
    mapobject_uuid: &FlexUuid,
    typeobject_uuid: Option<&FlexUuid>,
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
                TypeObjectUUID.eq(typeobject_uuid.map(|x| x.clone())),
                UpdatedAt.eq(now_ts),
            ))
            .execute(db.conn());
            match updated_row_res {
                Err(e) => {
                    let msg = format!(
                        "mapobject {} - error updating typeobject {:?}",
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
