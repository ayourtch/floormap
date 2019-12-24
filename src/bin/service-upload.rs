extern crate iron;
extern crate multipart;

use iron::prelude::*;

use iron::headers::ContentType;
use multipart::server::iron::Intercept;
use multipart::server::Entries;

pub fn thread_sleep_ms(ms: u64) {
    use std::thread;
    use std::time::Duration;
    thread::sleep(Duration::from_millis(ms));
}

pub fn handle_upload(orig_filename: &str, real_filename: &str) -> (String, Option<i32>) {
    use std::env;
    use std::process::Command;

    let mut child0 = Command::new("/bin/sh");
    let mut child = child0
        .arg("-c")
        .arg("./scripts/upload-script")
        .env("ORIG_FILENAME", orig_filename)
        .env("REAL_FILENAME", real_filename);

    /* change dir to job workspace */
    // std::env::set_current_dir(XXX).unwrap();

    // setsid();
    let mut child_spawned = child.spawn().expect("failed to execute process");

    use std::process::ExitStatus;
    let mut maybe_status: Option<ExitStatus> = None;

    loop {
        match child_spawned.try_wait() {
            Ok(Some(status)) => {
                /* done */
                maybe_status = Some(status);
                break;
            }
            Ok(None) => {
                println!("Status not ready yet from pid {}", child_spawned.id());
            }
            Err(e) => {
                panic!("Error attempting to wait: {:?}", e);
            }
        }
        thread_sleep_ms(5000);
    }
    let status = maybe_status.unwrap();
    match status.code() {
        Some(code) => println!("Finished with status code {}", code),
        None => println!("Finished due to signal"),
    }
    return ("".to_string(), status.code());
}

fn main() {
    // We start with a basic request handler chain.
    let mut chain = Chain::new(|req: &mut Request| {
        if let Some(entries) = req.extensions.get::<Entries>() {
            if let Some(file) = entries.files.get("test") {
                if file.len() == 1 {
                    let file = &file[0];
                    println!("File found: {:#?}, size: {}", &file, file.size);
                    if file.size > 0 {
                        println!("Handling upload...");
                        let orig_fname: String = file
                            .filename
                            .as_ref()
                            .unwrap_or(&"unknown".to_string())
                            .to_string();
                        let real_fname = file.path.to_str().unwrap_or("x").to_string();
                        handle_upload(&orig_fname, &real_fname);
                    }
                }
            }
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
