use std::env;
use std::fs;
use walkdir::WalkDir;
use uuid::Uuid;
use std::io::Read;
use std::io::Write;

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
                let mut ext = ext_split.last().unwrap().to_string().to_lowercase();
                if ext == "jpeg".to_string() {
                    ext = "jpg".to_string();
                }
                let out_path = format!("{}/{}.{}", &args[2], auuid, ext);
                println!("{} ->\n {}", fname.clone(), out_path);
                let message = format!("Unable to open {}", fname.clone());
                let mut f = std::fs::File::open(fname.clone()).expect(&message);
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).unwrap();
                // write buffer to new_path
                let mut f = std::fs::File::create(out_path).unwrap();
                f.write_all(&buffer).unwrap();
                fs::remove_file(fname.clone()).expect("Unable to remove file");
                
            };
        }
    }
}
