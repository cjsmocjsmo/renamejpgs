use std::env;
use std::fs;
use walkdir::WalkDir;
use uuid::Uuid;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: renamejpgs <directory> <directory>");
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
                let auuid = Uuid::new_v4();
                let ext_split = fname.split(".").collect::<Vec<&str>>();
                let ext = ext_split.last().unwrap().to_string().to_lowercase();
                let out_path = format!("{}/{}.{}", &args[2], auuid, ext);
                

                // open the file in a buffer and read the contents to bytes
                let mut buffer = fs::read(&fname).expect("Unable to read file");
                // write the bytes to the new location
                fs::write(&out_path, &mut buffer).expect("Unable to write file");
                

                
            };
        }
    }
}
