extern crate chrono;
extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate urlencoded;
extern crate uuid;

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

#[macro_use]
extern crate rspten;

#[macro_use]
pub mod pages;

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
            let mut resp = build_response($template, $data);
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
    // let mut router = Router::new();
    let mut router = pages::get_router();

    use mount::Mount;
    use staticfile::Static;
    use std::path::Path;

    router.get("/services", api_http_get_services_json, "get the services");
    router.get("/", root_page, "root_page");

    fn insert_new_service() {
        use schema::Services::dsl::*;

        let db = get_db();
        use uuid::Uuid;
        let my_uuid = Uuid::new_v4();
        let rand_uuid = format!("{}", my_uuid);

        let mut svc = Service {
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

    let mut s = rspten::RspServer::new();

    s.run(router, "test service", 4242);
}
