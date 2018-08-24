use std::io;
use std::env;
use std::fs;
use std::io::Read;

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

    // let mut target_clone = target.clone();

    home.push_str("/Coda/CodaEasy-Frontend/dist/");

    // Populate vector with files/folders and their destinations
    let mut destinations: Vec<Dest> = Vec::new();
    populate(&mut destinations);
    
    clear_target(&target, &destinations).expect("Could not clear target folder");;

    fs::create_dir(target.as_str().to_owned() + "/js").expect("Could not create folder");
    fs::create_dir(target.as_str().to_owned() + "/css").expect("Could not create folder");

    move_folder(home, target, &destinations);
}

fn move_folder(home: String, target: String, destinations: &Vec<Dest>) {
    //Get pathbufs for all files/folders in frontend directory
    let paths = fs::read_dir(home.as_str().to_owned()).unwrap().map(|entry| {
        entry.unwrap().path()
    });

    for pathbuf in paths {
        let path = pathbuf.file_name().unwrap().to_str().unwrap();

        if path != "assets" && path != "partners" && path != "compiler-output" && path != "avatars"  {
            match find_target(path, &destinations) {
                Some(dest) => {
                    if path.starts_with("root.html") {
                        root_refactor(&(home.as_str().to_owned() + "root.html"));
                    }

                    let from = pathbuf.to_str().unwrap();
                    let to = target.as_str().to_owned() + dest.as_str() + &path;

                    fs::copy(from, to)
                        .expect("Unable to move file");
                },

                None => println!("Not transfered {:?}", path),
            }
        } else {
            let mut assets = home.clone();
            assets.push_str((path.to_owned() + "/").as_str());

            let mut assets_target = target.clone();
            assets_target.push_str((path.to_owned() + "/").as_str());

            let _result = fs::create_dir(assets_target.as_str().to_owned());

            move_folder(assets, assets_target, &destinations);
        }   
    }
}

fn root_refactor(path: &str) {
    let mut file = fs::File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    
    contents = contents.replace("js/styles", "css/styles");

    fs::write(path, contents).expect("Unable to write file");
}

fn populate(destinations: &mut Vec<Dest>) {
    // HTML
    destinations.push(Dest {name: "root.html".to_string(), target: "".to_string()});   

    // JS
    destinations.push(Dest {name: "scripts".to_string(), target: "js/".to_string()});   
    destinations.push(Dest {name: "main".to_string(), target: "js/".to_string()});   
    destinations.push(Dest {name: "inline".to_string(), target: "js/".to_string()});   
    destinations.push(Dest {name: "polyfills".to_string(), target: "js/".to_string()});   
    destinations.push(Dest {name: "vendor".to_string(), target: "js/".to_string()});   

    // CSS
    destinations.push(Dest {name: "fontawesome-webfont".to_string(), target: "js/".to_string()});       
    destinations.push(Dest {name: "fa-brands-400".to_string(), target: "css/".to_string()});       
    destinations.push(Dest {name: "fa-regular-400".to_string(), target: "css/".to_string()});       
    destinations.push(Dest {name: "fa-solid-900".to_string(), target: "css/".to_string()});       
    destinations.push(Dest {name: "styles".to_string(), target: "css/".to_string()});       

    // Assets
    destinations.push(Dest {name: "about_platform_background.b7637a780117b2d5a091".to_string(), target: "js/".to_string()});
    destinations.push(Dest {name: "about_platform_background".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "background.2ddf02001a27dbe477f1".to_string(), target: "js/".to_string()});
    destinations.push(Dest {name: "background".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "botkyrka_workshop".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "coda-easy-about-us-bg.caa5ae56c9bc0d36d4fc".to_string(), target: "js/".to_string()});
    destinations.push(Dest {name: "coda-easy-about-us-bg".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "coda-easy-graph".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "codaeasy-computer".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "codaeasy-logo".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "code_tree_icon".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "cookie-icon".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "education-bg.77ecf82aa15c2147b2e2".to_string(), target: "js/".to_string()});
    destinations.push(Dest {name: "education-bg".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "first-time-showcase".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "grading_icon".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "guldraven".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "lager".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "left_box".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "logo".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "no_avatar".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "own_material_icon".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "right_box".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "strings".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "study.8539caeca31e5087fbd3".to_string(), target: "js/".to_string()});
    destinations.push(Dest {name: "study".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "vc-logo".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "camp_bg.a74f16de71b5ea6411a8".to_string(), target: "js/".to_string()});
    destinations.push(Dest {name: "camp_bg".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "aleksandar.jpeg".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "carl-johan.jpeg".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "johan.jpeg".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "mathilda.jpeg".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "oskar.jpeg".to_string(), target: "".to_string()});

    // Compiler-output
    destinations.push(Dest {name: "index_out_of_bounds".to_string(), target: "".to_string()});

    // Partners
    destinations.push(Dest {name: "arboga".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "atvexa".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "botkyrka".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "kth".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "metapontum".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "montessori-norrtalje".to_string(), target: "".to_string()});
    destinations.push(Dest {name: "ssis".to_string(), target: "".to_string()});

}

fn find_target(target: &str, destinations: &Vec<Dest>) -> Option<String> {
    for dest in destinations {
        if target.starts_with(dest.name.as_str()) {
            return Some(dest.target.to_owned())
        }
    }

    return None
}

fn should_delete(target: &str, destinations: &Vec<Dest>) -> bool {
    for dest in destinations {
        if target.starts_with(dest.name.as_str()) {
            return true
        }
    }

    return false
}

fn clear_target(target: &String, destinations: &Vec<Dest>) -> io::Result<()> {
    //Get pathbufs for all files/folders in frontend directory
    let paths = fs::read_dir(target.as_str().to_owned()).unwrap().map(|entry| {
        entry.unwrap().path()
    });

    for pathbuf in paths {
        let path = pathbuf.file_name().unwrap().to_str().unwrap();

        if path == "js" || path == "css" || path == "assets" {

            fs::remove_dir_all(pathbuf.to_str().unwrap())?;
        } else if path != "images" {

            if should_delete(path, &destinations) {
                fs::remove_file(pathbuf.to_str().unwrap())?;
            }
        } else {
            println!("Not deleted {:?}", path);
        }
    }

    Ok(())
}