use std::fs::*;

fn main() {
    let paths = read_dir("./").unwrap().map(|entry| {
        entry.unwrap().path()
    });

    for pathbuf in paths {
        let path: &str = pathbuf.to_str().unwrap();

        if path.starts_with("./test.txt") {
            styles_refactor(path);
        }
    }
}

fn styles_refactor(path: &str) {

}