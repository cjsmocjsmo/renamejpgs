use std::env;
use std::fs;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: renamejpgs <directory>");
        return;
    }

    let apath = &args[1];

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            if fname.contains("pussy") {
                // remove fname
                fs::remove_file(&fname).expect("Unable to remove file");
            } else {
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
                } else if middlename.contains(".PNG") {
                    finalname = middlename.replace(".PNG", ".png");
                } else if middlename.contains(".BMP") {
                    finalname = middlename.replace(".BMP", ".bmp");
                } else {
                    finalname = middlename;
                };

                if finalname != fname {
                    fs::rename(&fname, &finalname).expect("Unable to rename file");
                    println!("Renamed\n{}\nto\n{}", fname, finalname)
                }
            };
        }
    }
}
