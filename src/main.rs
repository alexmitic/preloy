fn main() {
    println!("Enter directory of frontend:");

    let mut frontend = String::new();

    std::io::stdin().read_line(&mut frontend)
                .expect("Failed");

    println!("{:?}", frontend);
}
