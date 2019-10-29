extern crate chrono;
extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;

use iron::prelude::*;
use iron::status;

extern crate diesel;

extern crate floorplan;

use self::floorplan::models::*;
use self::floorplan::*;
use self::floorplan::apiv1::*;

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

fn main() {
    let mut router = Router::new();
    

    use mount::Mount;
    use staticfile::Static;
    use std::path::Path;


    router.get(
        "/services",
        api_http_get_services_json,
        "get the services",
    );

    fn api_http_get_services_json(_: &mut Request) -> IronResult<Response> {
        let new_results = api_get_all_services();
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
    iron.http("127.0.0.1:4242").unwrap();
}


