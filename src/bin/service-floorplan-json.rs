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
extern crate floorplan;

use self::floorplan::apiv1::*;
use self::floorplan::models::*;
use self::floorplan::*;

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
    use iron::headers::ContentType;
    let mut bytes = vec![];
    let data_built = data.build();
    template
        .render_data(&mut bytes, &data_built)
        .expect("Failed to render");
    let payload = std::str::from_utf8(&bytes).unwrap();

    let mut resp = Response::with((status::Ok, payload));
    resp.headers.set(ContentType::html());
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
    use floorplan::template::get_page_mapbuilder;
    use iron::headers::ContentType;
    use urlencoded::UrlEncodedQuery;

    let return_url = match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref hashmap) => match (hashmap.get("ReturnUrl")) {
            Some(a) => format!("{}", a[0]),
            _ => format!("/"),
        },
        Err(ref e) => {
            println!("{:?}", e);
            format!("/")
        }
    };

    // let auth_user = LoginSessionState::new("", None);

    let template = match floorplan::template::maybe_compile_template("root") {
        Ok(t) => t,
        Err(e) => {
            return Ok(Response::with((
                status::Unauthorized,
                format!("Error occured: {}", e),
            )));
        }
    };

    println!("Login page return URL: {}", &return_url);

    let page_title = format!("root");

    let mut data = get_page_mapbuilder(req, &page_title);
    data = data.insert_str("ReturnUrl", return_url.clone());

    let redirect_to = "".to_string();

    render_response!(template, data, redirect_to)
}

fn main() {
    let mut router = Router::new();

    use mount::Mount;
    use staticfile::Static;
    use std::path::Path;

    router.get("/services", api_http_get_services_json, "get the services");
    router.get(
        "/api/v1/mapobjects/get/json",
        api_http_get_map_objects_for_map,
        "mapobjects-for-map",
    );
    router.get("/", root_page, "root_page");

    fn insert_new_service() {
        use schema::Services::dsl::*;

        let db = get_db();
        use uuid::Uuid;
        let my_uuid = Uuid::new_v4();
        let rand_uuid = format!("{}", my_uuid);

        let svc = Service {
            ServiceUUID: rand_uuid,
            MenuOrder: 0,
            Deleted: false,
            ServiceName: "SomeName".to_owned(),
            ServiceLabel: "SomeName".to_owned(),
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

    fn api_http_get_map_objects_for_map(_: &mut Request) -> IronResult<Response> {
        use std::str::FromStr;
        let map_uuid = Uuid::from_str("4b06c4b4-fb3a-11e9-af57-fb611161d50b").unwrap();
        let new_results = api_get_map_objects_for_map(map_uuid);
        let payload = serde_json::to_string(&new_results).unwrap();
        Ok(Response::with((status::Ok, payload)))
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
