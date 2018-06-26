use std::io::Read;
use std::env;
use std::fs::*;

fn main() {
    let mut home = match env::home_dir() {
        Some(path) => path.to_str().unwrap().to_owned(),
        None => "".to_string(),
    };
    home.push_str("/target/");

    let paths = read_dir("./").unwrap().map(|entry| {
        entry.unwrap().path()
    });

    for pathbuf in paths {
        let path: &str = pathbuf.to_str().unwrap();

        if path.starts_with("./test.") {
            root_refactor(path);

            let target = home.as_str().to_owned() + &path.replacen("./", "", 1);

            copy(path, target).expect("Unable move file");
        }
    }
}

fn root_refactor(path: &str) {
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    
    contents = contents.replace("js/styles", "css/styles");

    write(path, contents).expect("Unable to write file");
}