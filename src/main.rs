use std::io::Read;
use std::fs::*;

fn main() {
    let paths = read_dir("./").unwrap().map(|entry| {
        entry.unwrap().path()
    });

    for pathbuf in paths {
        let path: &str = pathbuf.to_str().unwrap();

        if path.starts_with("./test.txt") {
            root_refactor(path);
        }
    }
}

fn root_refactor(path: &str) {
    let mut file = File::open("./test.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    
    contents = contents.replace("js/styles", "css/styles");
    println!("{:?}", contents);
}