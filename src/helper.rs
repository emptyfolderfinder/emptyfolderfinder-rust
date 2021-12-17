pub fn print_help_message() {
    println!("Usage: emptyfolderfinder [-d] <Path>");
    println!("-d: Delete folders");
}

pub fn print_directories(dirs: &Vec<String>) {
    for dir in dirs {
        println!("{}", dir);
    }
}
