use walkdir::WalkDir;
use std::fs;
use clap::{Arg, App};

fn main() {
    let app = App::new("RenameJpgs")
        .version("0.1.0")
        .author("cjsmo")
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .help("Path to rename files in")
            .takes_value(true)
            .required(true));

    let matches = app.get_matches();

    let apath = matches.value_of("path").unwrap();

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            if fname.contains(".jpg.jpg") {
                let newname = fname.replace(".jpg.jpg", ".jpg");
                fs::rename(&fname, &newname).expect("Unable to rename file");
                println!("Renamed\n{}\nto\n{}", fname, newname)
            }
        };
    }
}