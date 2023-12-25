fn main() {
    let proceed = true;
    if proceed {
        println!("proceeding");
    } else {
        println!("stopping");
    }
    let height = 190;
    if height > 180 {
        println!("tall");
    } else if height > 160 {
        println!("average");
    } else {
        println!("short");
    }
}
