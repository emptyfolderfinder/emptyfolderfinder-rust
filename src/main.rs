use std::env;

mod directory;
mod helper;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut del = false;
    let path: &String;
    let dirs: Vec<String>;

    if args.len() < 2 {
        helper::print_help_message();
        return;
    }

    if args[1] == "-d" {
        if args.len() != 3 {
            helper::print_help_message();
            return;
        }

        del = true;
        path = &args[2];
    } else {
        path = &args[1];
    }

    dirs = match directory::check_directory(path) {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };

    if dirs.len() == 0 {
        println!("No empty directories found");
        return;
    }

    if del {        
        match directory::delete_directories(&dirs) {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };
        
        println!("Deleted the following directories");
        helper::print_directories(&dirs);
    } else {
        helper::print_directories(&dirs);
    }
}
