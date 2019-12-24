#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate chrono;
extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate urlencoded;
extern crate uuid;

use uuid::Uuid;

use iron::prelude::*;
use iron::status;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate floormap;

use self::floormap::apiv1::*;
use self::floormap::models::*;
use self::floormap::*;

use self::diesel::prelude::*;

use std::io::Read;
use std::sync::{Arc, Mutex};

use router::Router;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate hyper;
extern crate params;

use chrono::{NaiveDate, NaiveDateTime};

pub fn build_response(template: mustache::Template, data: mustache::MapBuilder) -> iron::Response {
    use iron::headers::{Connection, ContentType};
    let mut bytes = vec![];
    let data_built = data.build();
    template
        .render_data(&mut bytes, &data_built)
        .expect("Failed to render");
    let payload = std::str::from_utf8(&bytes).unwrap();

    let mut resp = Response::with((status::Ok, payload));
    resp.headers.set(ContentType::html());
    resp.headers.set(Connection::close());
    resp
}

macro_rules! render_response {
    ($template: ident, $data: ident, $redirect_to: ident) => {
        if $redirect_to.is_empty() {
            let resp = build_response($template, $data);
            Ok(resp)
        } else {
            use iron::headers::Location;
            // let mut resp = Response::with((status::TemporaryRedirect, $redirect_to.clone()));
            let mut resp = Response::with((status::Found, $redirect_to.clone()));
            resp.headers.set(ContentType::html());
            resp.headers.set(Location($redirect_to));
            Ok(resp)
        }
    };
}

fn root_page(req: &mut Request) -> IronResult<Response> {
    use floormap::template::get_page_mapbuilder;
    use iron::headers::ContentType;
    use urlencoded::UrlEncodedQuery;

    let template = floormap::template::maybe_compile_template("root");
    if let Err(e) = template {
        return Ok(Response::with((
            status::Unauthorized,
            format!("Error occured: {}", e),
        )));
    };
    let template = template.unwrap();

    let page_title = format!("root");
    let mut data = get_page_mapbuilder(req, &page_title);
    let redirect_to = "".to_string();

    render_response!(template, data, redirect_to)
}

fn arrange_page(req: &mut Request) -> IronResult<Response> {
    use floormap::template::get_page_mapbuilder;
    use iron::headers::ContentType;
    use urlencoded::UrlEncodedQuery;

    let template = floormap::template::maybe_compile_template("arrange");
    if let Err(e) = template {
        return Ok(Response::with((
            status::Unauthorized,
            format!("Error occured: {}", e),
        )));
    };
    let template = template.unwrap();

    let page_title = format!("arrange");
    let mut data = get_page_mapbuilder(req, &page_title);
    let redirect_to = "".to_string();

    render_response!(template, data, redirect_to)
}

fn main() {
    use floormap::flextimestamp::FlexTimestamp;
    let mut router = Router::new();

    use mount::Mount;
    use staticfile::Static;
    use std::path::Path;

    router.get("/services", api_http_get_services_json, "get the services");
    router.get(
        "/api/v1/mapobjects/get/json/:query_timestamp",
        api_http_get_map_objects,
        "mapobjects-for-map with argument",
    );
    router.put(
        "/api/v1/mapobjects/xy/put/json",
        api_http_put_map_object_xy,
        "set mapobjects-for-map",
    );
    router.put(
        "/api/v1/mapobjects/delete/put/json",
        api_http_put_mapobject_delete,
        "set mapobjects-for-map deleted",
    );
    router.put(
        "/api/v1/floormaps/delete/put/json",
        api_http_put_floormap_delete,
        "set floormap deleted",
    );
    router.put(
        "/api/v1/floormaps/copy/put/json",
        api_http_put_floormap_copy,
        "floormap copy",
    );
    router.put(
        "/api/v1/mapobjects/name_description/put/json",
        api_http_put_mapobject_name_description,
        "set mapobjects-for-map name",
    );
    router.put(
        "/api/v1/mapobjects/new/put/json",
        api_http_put_new_mapobject,
        "make new mapobject",
    );
    router.get(
        "/images/floormaps/:floormap_uuid",
        http_get_floormap_image,
        "floormap image page",
    );
    router.put(
        "/api/v1/floormaps/clip/put/json",
        api_http_put_floormaps_clip,
        "set clip for floormap",
    );
    router.get(
        "/images/floormaps/thumbnails/:floormap_uuid",
        http_get_floormap_thumbnail_image,
        "floormap image thumbnails page",
    );
    router.get("/", root_page, "root_page");
    router.get("/arrange.aspx", arrange_page, "arrange_page");

    fn insert_new_service() {
        use schema::Services::dsl::*;

        let db = get_db();
        use uuid::Uuid;
        let my_uuid = Uuid::new_v4();

        let svc = Service {
            // ServiceUUID: rand_uuid, -- set by default
            MenuOrder: 0,
            Deleted: false,
            ServiceName: "SomeName".to_owned(),
            ServiceLabel: "SomeName".to_owned(),
            ..Default::default()
        };

        // let inserted_id: std::result::Result<_, diesel::result::Error> = {

        diesel::insert_into(schema::Services::dsl::Services)
            .values(&svc)
            .execute(db.conn())
            .unwrap();
    }

    fn api_http_get_services_json(_: &mut Request) -> IronResult<Response> {
        insert_new_service();
        let new_results = api_get_all_services();
        let payload = serde_json::to_string(&new_results).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    fn api_http_get_map_objects(req: &mut Request) -> IronResult<Response> {
        use floormap::flextimestamp::FlexTimestamp;
        use floormap::flexuuid::FlexUuid;
        use iron::headers::{Connection, ContentType};

        use std::str::FromStr;

        let ref query_ts = req
            .extensions
            .get::<Router>()
            .unwrap()
            .find("query_timestamp")
            .map_or(FlexTimestamp::from_timestamp(0), |s| {
                FlexTimestamp::from_timestamp(s.parse::<i64>().unwrap_or(0))
            });
        // println!("from start: {:?}", &query_ts);

        let new_results = api_get_map_objects(&query_ts);
        let payload = serde_json::to_string(&new_results).unwrap();
        let mut resp = Response::with((status::Ok, payload));
        resp.headers.set(Connection::close());
        Ok(resp)
    }
    fn api_http_put_map_object_xy(req: &mut Request) -> IronResult<Response> {
        use floormap::db::db_set_mapobject_arrow_xy;
        use floormap::db::db_set_mapobject_xy;
        use std::str::FromStr;
        let mut payload = String::new();
        req.body.read_to_string(&mut payload).unwrap();
        let cr_res: Result<Vec<ApiV1MapObjectSetXYRecord>, serde_json::Error> =
            serde_json::from_str(&payload);
        match cr_res {
            Ok(cr) => {
                println!("CR: {:?}", &cr);
                for o in cr {
                    db_set_mapobject_xy(&o.MapObjectUUID, o.MapX, o.MapY, "web").unwrap();
                    db_set_mapobject_arrow_xy(&o.MapObjectUUID, o.ArrowX, o.ArrowY, "web").unwrap();
                }
                /*
                let map_uuid = Uuid::from_str("4b06c4b4-fb3a-11e9-af57-fb611161d50b").unwrap();
                let new_results = api_get_map_objects_for_map(&map_uuid, &FlexTimestamp::now());
                let payload = serde_json::to_string(&new_results).unwrap();
                */

                Ok(Response::with((status::Ok, payload)))
            }
            Err(e) => Ok(Response::with((
                status::BadRequest,
                format!("error: {:?}", e),
            ))),
        }
    }
    fn api_http_put_floormaps_clip(req: &mut Request) -> IronResult<Response> {
        use floormap::db::db_set_floormap_clip;
        use std::str::FromStr;
        let mut payload = String::new();
        req.body.read_to_string(&mut payload).unwrap();
        let cr_res: Result<Vec<ApiV1FloorMapSetClipRecord>, serde_json::Error> =
            serde_json::from_str(&payload);
        match cr_res {
            Ok(cr) => {
                println!("CR: {:?}", &cr);
                for o in cr {
                    db_set_floormap_clip(
                        &o.FloorMapUUID,
                        o.ClipLeft,
                        o.ClipTop,
                        o.ClipWidth,
                        o.ClipHeight,
                    );
                }

                Ok(Response::with((status::Ok, payload)))
            }
            Err(e) => Ok(Response::with((
                status::BadRequest,
                format!("error: {:?}", e),
            ))),
        }
    }
    fn api_http_put_mapobject_delete(req: &mut Request) -> IronResult<Response> {
        use floormap::db::db_set_mapobject_deleted;
        use std::str::FromStr;
        let mut payload = String::new();
        req.body.read_to_string(&mut payload).unwrap();
        let cr_res: Result<Vec<ApiV1MapObjectDeleteRecord>, serde_json::Error> =
            serde_json::from_str(&payload);
        match cr_res {
            Ok(cr) => {
                println!("CR: {:?}", &cr);
                for o in cr {
                    db_set_mapobject_deleted(&o.MapObjectUUID, true).unwrap();
                }

                Ok(Response::with((status::Ok, payload)))
            }
            Err(e) => Ok(Response::with((
                status::BadRequest,
                format!("error: {:?}", e),
            ))),
        }
    }

    fn api_http_put_floormap_delete(req: &mut Request) -> IronResult<Response> {
        use floormap::db::db_set_floormap_deleted;
        use std::str::FromStr;
        let mut payload = String::new();
        req.body.read_to_string(&mut payload).unwrap();
        let cr_res: Result<Vec<ApiV1FloorMapDeleteRecord>, serde_json::Error> =
            serde_json::from_str(&payload);
        match cr_res {
            Ok(cr) => {
                println!("CR: {:?}", &cr);
                for o in cr {
                    db_set_floormap_deleted(&o.FloorMapUUID, true).unwrap();
                }

                Ok(Response::with((status::Ok, payload)))
            }
            Err(e) => Ok(Response::with((
                status::BadRequest,
                format!("error: {:?}", e),
            ))),
        }
    }

    fn api_http_put_floormap_copy(req: &mut Request) -> IronResult<Response> {
        use std::str::FromStr;
        let mut payload = String::new();
        req.body.read_to_string(&mut payload).unwrap();
        let cr_res: Result<Vec<ApiV1FloorMapCopyRecord>, serde_json::Error> =
            serde_json::from_str(&payload);
        match cr_res {
            Ok(cr) => {
                println!("CR: {:?}", &cr);
                for o in cr {
                    match o.Operation {
                        ApiV1FloorMapCopyOperation::FloorMapOverwrite => {}
                        ApiV1FloorMapCopyOperation::FloorMapInsertBefore => {
                            let dst = db_get_floormap(&o.DstFloorMapUUID);
                            let src = db_get_floormap(&o.SrcFloorMapUUID);
                            if src.is_ok() && dst.is_ok() {
                                use floormap::db::db_insert_new_floormap;

                                let src = src.unwrap();
                                let dst = dst.unwrap();
                                db_insert_new_floormap(
                                    &src.Name,
                                    &src.Description,
                                    &src.FullText,
                                    &src.FloorMapFileName,
                                    &dst.ParentFloorPlanUUID,
                                    dst.SortOrder,
                                );
                            }
                        }
                        ApiV1FloorMapCopyOperation::FloorMapInsertAfter => {
                            let dst = db_get_floormap(&o.DstFloorMapUUID);
                            let src = db_get_floormap(&o.SrcFloorMapUUID);
                            if src.is_ok() && dst.is_ok() {
                                use floormap::db::db_insert_new_floormap;

                                let src = src.unwrap();
                                let dst = dst.unwrap();
                                db_insert_new_floormap(
                                    &src.Name,
                                    &src.Description,
                                    &src.FullText,
                                    &src.FloorMapFileName,
                                    &dst.ParentFloorPlanUUID,
                                    dst.SortOrder + 1,
                                );
                            }
                        }
                    }
                    // db_set_floormap_deleted(&o.FloorMapUUID, true).unwrap();
                }

                Ok(Response::with((status::Ok, payload)))
            }
            Err(e) => Ok(Response::with((
                status::BadRequest,
                format!("error: {:?}", e),
            ))),
        }
    }

    fn api_http_put_mapobject_name_description(req: &mut Request) -> IronResult<Response> {
        use floormap::db::db_set_mapobject_labelsize;
        use floormap::db::db_set_mapobject_name_description;
        use floormap::db::db_set_mapobject_typeobjectuuid;
        use std::str::FromStr;
        let mut payload = String::new();
        req.body.read_to_string(&mut payload).unwrap();
        let cr_res: Result<Vec<ApiV1MapObjectSetNameDescriptionRecord>, serde_json::Error> =
            serde_json::from_str(&payload);
        match cr_res {
            Ok(cr) => {
                println!("CR: {:?}", &cr);
                for o in cr {
                    db_set_mapobject_name_description(&o.MapObjectUUID, &o.Name, &o.Description)
                        .unwrap();
                    db_set_mapobject_labelsize(&o.MapObjectUUID, o.LabelSize).unwrap();
                    db_set_mapobject_typeobjectuuid(&o.MapObjectUUID, o.TypeObjectUUID.as_ref())
                        .unwrap();
                }
                Ok(Response::with((status::Ok, payload)))
            }
            Err(e) => Ok(Response::with((
                status::BadRequest,
                format!("error: {:?}", e),
            ))),
        }
    }

    fn api_http_put_new_mapobject(req: &mut Request) -> IronResult<Response> {
        use floormap::db::db_insert_new_mapobject;
        use std::str::FromStr;
        let mut payload = String::new();
        req.body.read_to_string(&mut payload).unwrap();
        let cr_res: Result<ApiV1NewMapObjectRecord, serde_json::Error> =
            serde_json::from_str(&payload);
        match cr_res {
            Ok(o) => {
                println!("O: {:?}", &o);
                let uuid = db_insert_new_mapobject(
                    &o.ParentMapUUID,
                    &o.Name,
                    &o.Description,
                    o.MapX,
                    o.MapY,
                );
                floormap::db::db_set_mapobject_labelsize(&uuid, o.LabelSize).unwrap();
                floormap::db::db_set_mapobject_typeobjectuuid(&uuid, o.TypeObjectUUID.as_ref())
                    .unwrap();
                let payload = serde_json::to_string(&uuid).unwrap();

                Ok(Response::with((status::Ok, payload)))
            }
            Err(e) => Ok(Response::with((
                status::BadRequest,
                format!("error: {:?}", e),
            ))),
        }
    }

    fn http_get_floormap_image(req: &mut Request) -> IronResult<Response> {
        use floormap::flextimestamp::FlexTimestamp;
        use floormap::flexuuid::FlexUuid;
        use iron::headers::{Connection, ContentType};
        use std::fs::File;

        use std::str::FromStr;
        let map_str = "1e79ba6e-fb3a-11e9-b124-03c84357f69a";
        let map_uuid = Uuid::from_str(map_str).unwrap();
        let flex_uuid = FlexUuid::from_str(map_str).unwrap();

        let ref query_uuid = req
            .extensions
            .get::<Router>()
            .unwrap()
            .find("floormap_uuid")
            .map_or(flex_uuid.clone(), |s| {
                FlexUuid::from_str(&s).unwrap_or(flex_uuid.clone())
            });

        let floor = db_get_floormap(&query_uuid);
        println!("Got floormap: {:?}", &floor);
        let mut buffer = Vec::new();
        if floor.is_err() {
            let payload = format!(
                "Floor plan with uuid {} not found: {:?}",
                &query_uuid, &floor
            );
            // let mut resp = Response::with((status::NotFound, payload));
            // return Ok(resp);
            let mut f = File::open("staticfiles/grid_page.png").unwrap();
            f.read_to_end(&mut buffer).unwrap();
        } else {
            let floor = floor.unwrap();

            let mut f = File::open(&floor.FloorMapFileName).unwrap();
            f.read_to_end(&mut buffer).unwrap();
        }

        let mut resp = Response::with((status::Ok, buffer));
        resp.headers.set(ContentType::png());
        resp.headers.set(Connection::close());
        Ok(resp)
    }

    fn http_get_floormap_thumbnail_image(req: &mut Request) -> IronResult<Response> {
        use floormap::flextimestamp::FlexTimestamp;
        use floormap::flexuuid::FlexUuid;
        use iron::headers::{Connection, ContentType};
        use std::fs::File;

        use std::str::FromStr;
        let map_str = "1e79ba6e-fb3a-11e9-b124-03c84357f69a";
        let map_uuid = Uuid::from_str(map_str).unwrap();
        let flex_uuid = FlexUuid::from_str(map_str).unwrap();

        let ref query_uuid = req
            .extensions
            .get::<Router>()
            .unwrap()
            .find("floormap_uuid")
            .map_or(flex_uuid.clone(), |s| {
                FlexUuid::from_str(&s).unwrap_or(flex_uuid.clone())
            });

        let floor = db_get_floormap(&query_uuid);
        println!("Got floormap: {:?}", &floor);
        if floor.is_err() {
            let payload = format!(
                "Floor plan with uuid {} not found: {:?}",
                &query_uuid, &floor
            );
            let mut resp = Response::with((status::NotFound, payload));
            return Ok(resp);
        }
        let floor = floor.unwrap();
        let filename = format!("{}-thumb.png", &floor.FloorMapFileName);
        println!("Filename: {}", &filename);

        let mut f = File::open(&filename).unwrap();
        let mut buffer = Vec::new();

        f.read_to_end(&mut buffer).unwrap();

        let mut resp = Response::with((status::Ok, buffer));
        resp.headers.set(ContentType::png());
        resp.headers.set(Connection::close());
        Ok(resp)
    }

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/static/", Static::new(Path::new("staticfiles/")));

    use std::time::Duration;
    // use iron::prelude::*;
    //  use iron::status;
    use iron::Timeouts;

    let mut iron = Iron::new(mount);
    iron.threads = 1;
    iron.timeouts = Timeouts {
        keep_alive: Some(Duration::from_secs(10)),
        read: Some(Duration::from_secs(10)),
        write: Some(Duration::from_secs(10)),
    };

    let port = 4242;
    let bind_ip = std::env::var("BIND_IP").unwrap_or("127.0.0.1".to_string());
    println!("HTTP server starting on {}:{}", &bind_ip, port);
    iron.http(&format!("{}:{}", &bind_ip, port)).unwrap();
}
