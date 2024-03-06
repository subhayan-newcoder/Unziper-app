use zip;
use std::{fs,io,env};


fn main() {
    let input_folder = env::args().nth(1).expect("Please provide the path of the zip folder");

    let f = fs::File::open(input_folder).unwrap();
    let mut archive = zip::ZipArchive::new(f).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        if (file.name()).ends_with("/") {
            fs::create_dir_all(&outpath).unwrap();
            println!("{} folder is created", file.name());
        } else {
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
            println!("{} file is created", file.name());
        }
    }
}
