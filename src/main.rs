use std::io::Read;
use std::env;
use std::fs::*;

struct Dest {
    name: String,
    target: String,
}

fn main() {
    // Get home directory
    let mut home = match env::home_dir() {
        Some(path) => path.to_str().unwrap().to_owned(),
        None => "".to_string(),
    };
    home.push_str("/target/");

    // Populate vector with files/folders and their destinations
    let mut destinations: Vec<Dest> = Vec::new();
    populate(&mut destinations);


    // Get pathbufs for all files/folders in current directory
    let paths = read_dir("./").unwrap().map(|entry| {
        entry.unwrap().path()
    });

    for pathbuf in paths {
        let path: &str = pathbuf.to_str().unwrap();

        match find_target(path, &destinations) {
            Some(dest) => {
                if path.starts_with("./test.") {
                    root_refactor(path);
                }

                let target = home.as_str().to_owned() + dest.as_str() + &path.replacen("./", "", 1);

                copy(path, target).expect("Unable move file");
            },

            None => println!("Not used {:?}", path),
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

fn populate(destinations: &mut Vec<Dest>) {
    destinations.push(Dest {name: "./test.".to_string(), target: "".to_string()});   
}

fn find_target(target: &str, destinations: &Vec<Dest>) -> Option<String> {
    for dest in destinations {
        if target.starts_with(dest.name.as_str()) {
            return Some(dest.target.to_owned())
        }
    }

    return None
}