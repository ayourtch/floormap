extern crate iron;
extern crate multipart;

use iron::prelude::*;

use multipart::server::iron::Intercept;
use multipart::server::Entries;
use iron::headers::ContentType;

fn main() {
    // We start with a basic request handler chain.
    let mut chain = Chain::new(|req: &mut Request| {
        if let Some(entries) = req.extensions.get::<Entries>() {
            Ok(Response::with(format!("{:#?}", entries)))
        } else {
            let contents = std::fs::read_to_string("upload_form.html").unwrap();
            let mut resp = Response::with(contents);
            resp.headers.set(ContentType::html());
            Ok(resp)
        }
    });

    // `Intercept` will read out the entries and place them as an extension in the request.
    // It has various builder-style methods for changing how it will behave, but has sane settings
    // by default.
    chain.link_before(Intercept::default().file_size_limit(64000000));

    let port = 4243;
    let bind_ip = std::env::var("BIND_IP").unwrap_or("127.0.0.1".to_string());
    println!("HTTP server starting on {}:{}", &bind_ip, port);
    let http_bind = format!("{}:{}", &bind_ip, port);


    Iron::new(chain).http(&http_bind).unwrap();
}
