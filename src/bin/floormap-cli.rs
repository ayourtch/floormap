extern crate clap;
extern crate csv;
extern crate image;
extern crate zip;
#[macro_use]
extern crate serde;

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
                // full_text = contents;
            }
        }
        let pathname =
            if std::path::Path::new(&format!("{}/page-{:01}.png-thumb.png", &dirname, page_nr))
                .exists()
            {
                let pathname = format!("{}/page-{:01}.png", &dirname, page_nr);
                // println!("Checking path {}", &pathname);
                let path = Path::new(&pathname);
                if !path.exists() {
                    break;
                }
                pathname
            } else {
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
                pathname
            };
        let page_name = format!("Page {:02}", page_nr);
        db_insert_new_floormap(
            &page_name,
            &description,
            &full_text,
            &pathname,
            &plan_uuid,
            999999,
        );

        page_nr = page_nr + 1;
    }
    let page_count = page_nr - 1;
    if page_count == 0 {
        panic!("No pages found of format 'page-N.png'");
    }
    println!("imported {} pages", page_count);
}

#[serde(default)]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct ExportMapObject {
    pub Label: String,
    pub PositionX: i32,
    pub PositionY: i32,
}

fn export_cropped_floorplan(dir: &str) {
    use floormap::apiv1::api_get_map_objects;
    use floormap::*;
    use image::{imageops, GenericImageView, ImageBuffer, RgbImage};
    use serde::Serialize;
    use std::convert::TryInto;
    use std::fs;
    use std::io;
    use std::io::Write;

    fs::create_dir_all(dir).unwrap();
    let start_of_time = FlexTimestamp::from_timestamp(0);
    let results = api_get_map_objects(&start_of_time);

    let zip_file = std::fs::File::create(format!("/tmp/export.zip")).unwrap();
    //let mut w = std::io::Cursor::new(zip_file);
    let mut zip = zip::ZipWriter::new(zip_file);

    for floor in &results.FloorMaps {
        let mut has_objects = false;
        for object in &results.MapObjects {
            if object.ParentMapUUID == floor.FloorMapUUID {
                has_objects = true;
            }
        }
        let zip_options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        if has_objects {
            zip.add_directory(&floor.Name, zip_options.clone());
            let dir_floor = format!("{}/{}", &dir, &floor.Name);
            fs::create_dir_all(&dir_floor).unwrap();
            println!("==== {:?}", &floor);
            let db = get_db();
            let fm = db_get_floormap(&floor.FloorMapUUID).unwrap();

            let mut img = image::open(&fm.FloorMapFileName).unwrap();

            let (mut clip_W, mut clip_H) = img.dimensions();
            if fm.ClipWidth != 0 {
                clip_W = fm.ClipWidth as u32;
            }
            if fm.ClipHeight != 0 {
                clip_H = fm.ClipHeight as u32;
            }
            let clip_X = fm.ClipLeft.try_into().unwrap_or(0);
            let clip_Y = fm.ClipTop.try_into().unwrap_or(0);

            let mut subimg = image::imageops::crop(&mut img, clip_X, clip_Y, clip_W, clip_H);
            let subimg_name = format!("{}/floor.png", &dir_floor);
            subimg.to_image().save(&subimg_name).unwrap();

            zip.start_file(&format!("{}/floor.png", &floor.Name), zip_options.clone());
            zip.write(&std::fs::read(&subimg_name).unwrap());
            let csv_file = std::fs::File::create(format!("{}/floor.csv", &dir_floor)).unwrap();
            let mut wtr = csv::Writer::from_writer(csv_file);
            let mut wtr2 = csv::Writer::from_writer(vec![]);
            zip.start_file(&format!("{}/floor.csv", &floor.Name), zip_options.clone());
            for o in &results.MapObjects {
                if o.ParentMapUUID == floor.FloorMapUUID {
                    let x = o.MapX + o.ArrowX - fm.ClipLeft;
                    let y = o.MapY + o.ArrowY - fm.ClipTop;
                    if x >= 0
                        && y >= 0
                        && x <= clip_W.try_into().unwrap()
                        && y <= clip_H.try_into().unwrap()
                    {
                        let r = ExportMapObject {
                            Label: o.Name.clone(),
                            PositionX: x,
                            PositionY: y,
                        };
                        wtr.serialize(&r).unwrap();
                        wtr2.serialize(&r).unwrap();
                    }
                }
            }
            wtr.flush().unwrap();
            wtr2.flush().unwrap();
            // let csv_data = String::from_utf8(wtr2.into_inner().unwrap()).unwrap();
            let csv_data = wtr2.into_inner().unwrap();
            zip.write(&csv_data);
        }
    }
    zip.finish();
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
        .subcommand(
            SubCommand::with_name("export-database")
                .about("export the database to file")
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .required(true)
                        .value_name("FILE")
                        .help("output file name"),
                ),
        )
        .subcommand(
            SubCommand::with_name("export-cropped-floorplan")
                .about("export the cropped floorplan")
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .required(true)
                        .value_name("DIR")
                        .help("output dir name"),
                ),
        )
        .subcommand(
            SubCommand::with_name("import-database")
                .about("import the database from file")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .required(true)
                        .value_name("FILE")
                        .help("input file name"),
                ),
        )
        .subcommand(SubCommand::with_name("export-assets").about("export the assets to stdout"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("export-cropped-floorplan") {
        let dirname = matches.value_of("output").unwrap();
        export_cropped_floorplan(&dirname);
    }
    if let Some(matches) = matches.subcommand_matches("export-database") {
        use std::fs::File;
        use std::io::prelude::*;
        let filename = matches.value_of("output").unwrap();
        let j = floormap::db::db_get_json();
        let mut file = File::create(filename).unwrap();
        file.write_all(j.as_bytes());
    }
    if let Some(matches) = matches.subcommand_matches("import-database") {
        let filename = matches.value_of("input").unwrap();
        let js = std::fs::read_to_string(filename).unwrap();
        floormap::db::db_put_json(&js);
    }

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
