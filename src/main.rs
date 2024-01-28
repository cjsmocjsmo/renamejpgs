use walkdir::WalkDir;
use std::fs;

fn main() {

    let apath = "/media/pi/taz/Master_HPics/".to_string();

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            let middlename;
            let finalname;

            if fname.contains(" ") {
                middlename = fname.replace(" ", "_");
            } else {
                middlename = fname.clone();
            };

            if middlename.contains(".JPG") {
                finalname = middlename.replace(".JPG", ".jpg");
            } else if middlename.contains(".jpeg") {
                finalname = middlename.replace(".jpeg", ".jpg");
            } else {
                finalname = middlename;
            };

            if finalname != fname {
                fs::rename(&fname, &finalname).expect("Unable to rename file");
                println!("Renamed\n{}\nto\n{}", fname, finalname)
            }
            // fs::rename(&fname, &newname).expect("Unable to rename file");
            // println!("Renamed\n{}\nto\n{}", fname, newname)
        };
    }
}