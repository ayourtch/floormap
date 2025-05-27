use super::DB_TYPE_NAME;
use iron::prelude::*;
use mustache::Template;

use iron;

static GIT_COMMIT_INFO: &'static str = include_str!(concat!(env!("OUT_DIR"), "/commit-info.txt"));

pub mod menu;
use menu::*;

pub fn get_page_mapbuilder(req: &mut Request, page_title: &str) -> mustache::MapBuilder {
    use mustache::Data;
    use mustache::MapBuilder;
    let mut data = MapBuilder::new();

    data = data.insert_str("page_title", page_title);
    data = data.insert_str(
        "user_ip_address",
        get_real_ip(req).unwrap_or(format!("IP unknown")),
    );
    data = data.insert_str("backend_type", DB_TYPE_NAME);
    data = data.insert_str("git_commit_info", GIT_COMMIT_INFO);
    let menu = get_page_menu("", "");

    data = data.insert_vec("menu", |builder| insert_menu(builder, &menu));

    data
}

fn get_real_ip(req: &mut Request) -> Option<String> {
/*
    FIXME
    header! { (XRealIP, "X-Real-IP") => [String] }
    // let x_real_ip = req.headers.get_raw("X-Real-IP");
    match req.headers.get::<XRealIP>() {
        Some(xri) => Some(xri.clone().to_string()),
        None => None, // "0.0.0.0".to_string(),
    }
*/
    None
}

pub fn maybe_compile_template(name: &str) -> Result<Template, mustache::Error> {
    mustache::compile_path(format!("./templates/{}.mustache", name))
}

pub fn compile_template(name: &str) -> Template {
    maybe_compile_template(name).expect("Failed to compile")
}
