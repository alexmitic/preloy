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

    let mut target = home.to_owned();
    target.push_str("/Coda/CodaEasy-Backend/src/main/resources/static/");

    home.push_str("/Coda/CodaEasy-Frontend/dist/");

    // Populate vector with files/folders and their destinations
    let mut destinations: Vec<Dest> = Vec::new();
    populate(&mut destinations);

    move_folder(home, target, &destinations);
}

fn move_folder(home: String, target: String, destinations: &Vec<Dest>) {
    //Get pathbufs for all files/folders in frontend directory
    let paths = read_dir(home.as_str().to_owned()).unwrap().map(|entry| {
        entry.unwrap().path()
    });

    for pathbuf in paths {
        let path = pathbuf.file_name().unwrap().to_str().unwrap();

        match find_target(path, &destinations) {
            Some(dest) => {
                if path.starts_with("root.html") {
                    root_refactor(&(home.as_str().to_owned() + "root.html"));
                }

                let from = pathbuf.to_str().unwrap();
                let to = target.as_str().to_owned() + dest.as_str() + &path;

                copy(from, to)
                    .expect("Unable to move file");
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
    // HTML
    destinations.push(Dest {name: "root.html".to_string(), target: "".to_string()});   

    // JS
    destinations.push(Dest {name: "scripts".to_string(), target: "js/".to_string()});   
    destinations.push(Dest {name: "main".to_string(), target: "js/".to_string()});   
    destinations.push(Dest {name: "inline".to_string(), target: "js/".to_string()});   
    destinations.push(Dest {name: "polyfills".to_string(), target: "js/".to_string()});   

    // CSS
    destinations.push(Dest {name: "fontawesome-webfont".to_string(), target: "css/".to_string()});       
    destinations.push(Dest {name: "styles".to_string(), target: "css/".to_string()});       
}

fn find_target(target: &str, destinations: &Vec<Dest>) -> Option<String> {
    for dest in destinations {
        if target.starts_with(dest.name.as_str()) {
            return Some(dest.target.to_owned())
        }
    }

    return None
}