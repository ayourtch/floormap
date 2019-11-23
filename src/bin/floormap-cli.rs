extern crate clap;
use clap::{App, Arg, SubCommand};

use floormap;
use floormap::flextimestamp::*;

fn import_pages_from(dirname: &str) {
    use floormap::db::db_insert_new_floormap;
    use floormap::db::db_insert_new_floorplan;
    use std::path::Path;

    println!("Importing floor plan from {}", &dirname);
    let dirname = format!("{}/images", &dirname);
    let mut page_nr = 1;
    let now = FlexTimestamp::now();
    let description = format!("imported via cli at {:?}", now);
    let plan_uuid = db_insert_new_floorplan("floorplan", &description, &dirname, None);
    loop {
        let mut description = format!("");
        let mut full_text = format!("");
        let pathname = format!("{}/page-{}.txt", &dirname, page_nr);
        let path = std::path::Path::new(&pathname);
        if path.exists() {
            use std::fs::File;
            use std::io::prelude::*;
            let mut file = File::open(path).unwrap();
            let mut contents = String::new();
            if let Ok(res) = file.read_to_string(&mut contents) {
                description = contents
                    .lines()
                    .next()
                    .clone()
                    .map(|x| x.clone().trim())
                    .unwrap_or("")
                    .to_string();
                full_text = contents;
            }
        }
        let pathname = format!("{}/page-{:02}.png", &dirname, page_nr);
        let pathname = format!("{}/page-{:02}.png-thumb.png", &dirname, page_nr);
        let path = std::path::Path::new(&pathname);
        if !path.exists() {
            break;
        }
        let pathname = format!("{}/page-{:02}.png", &dirname, page_nr);
        // println!("Checking path {}", &pathname);
        let path = Path::new(&pathname);
        if !path.exists() {
            break;
        }
        let page_name = format!("Page {:02}", page_nr);
        db_insert_new_floormap(&page_name, &description, &full_text, &pathname, &plan_uuid);

        page_nr = page_nr + 1;
    }
    let page_count = page_nr - 1;
    if page_count == 0 {
        panic!("No pages found of format 'page-N.png'");
    }
    println!("imported {} pages", page_count);
}

fn main() {
    let matches = App::new("FloorMap CLI")
        .version("1.0")
        .author("Andrew Yourtchenko <ayourtch@gmail.com>")
        .about("CLI for MyFloorMap")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely"),
        )
        .subcommand(
            SubCommand::with_name("import-floor-plan")
                .about("Import the floor plan from a directory")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        // .required(true)
                        .value_name("DIR")
                        .help("input directory with files"),
                ),
        )
        .subcommand(SubCommand::with_name("export-assets").about("export the assets to stdout"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("import-floor-plan") {
        // use std::path::Path;

        if let Some(dirname) = matches.value_of("input") {
            import_pages_from(dirname);
        } else {
            // List the import directories...
            let base_dir = "/var/a3s/http/floor-plan-images";

            println!("Available imports:");
            for entry in std::fs::read_dir(base_dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                let subdir_name = path
                    .file_name()
                    .ok_or("No filename")
                    .unwrap()
                    .to_string_lossy();
                if let Ok(unix_ts) = subdir_name.parse::<i64>() {
                    let ts = FlexTimestamp::from_timestamp(unix_ts);
                    let dirname = format!("{}/{}/images", &base_dir, &subdir_name);
                    let mut page_nr = 1;
                    loop {
                        let pathname = format!("{}/page-{:02}.png", &dirname, page_nr);
                        let path = std::path::Path::new(&pathname);
                        if !path.exists() {
                            break;
                        }
                        let pathname = format!("{}/page-{:02}.png-thumb.png", &dirname, page_nr);
                        let path = std::path::Path::new(&pathname);
                        if !path.exists() {
                            break;
                        }
                        page_nr = page_nr + 1;
                    }
                    if page_nr > 1 {
                        println!("{}/{}: {:?}", &base_dir, &subdir_name, &ts);
                    }
                }
            }
        }
    }
}
