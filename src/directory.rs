use std::fs;

pub fn check_directory(path: &String) -> std::io::Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    let mut empty_dirs: Vec<String>;
    let mut dirs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    let dir_files = match fs::read_dir(path) {
        Ok(files) => files,
        Err(e) => return Err(e),
    };

    for file in dir_files {
        let f = match file {
            Ok(fi) => fi,
            Err(e) => return Err(e),
        };

        let path = f.path();

        let file_path = match path.to_str() {
            Some(p) => String::from(p),
            None => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not convert Filename")),
        };

        if path.is_dir() {
            dirs.push(file_path.to_owned());

            empty_dirs = match check_directory(&file_path) {
                Ok(e) => e,
                Err(e) => return Err(e),
            };

            for d in empty_dirs {
                result.push(d);
            }
        } else {
            files.push(file_path);
        }
    }

    if files.len() == 0 && dirs.len() == 0 {
        result.push(path.to_owned());
        return Ok(result);
    }

    if result.len() == dirs.len() && files.len() == 0 {
        result.push(path.to_owned());
        return Ok(result);
    }

    Ok(result)
}

pub fn delete_directories(dirs: &Vec<String>) -> std::io::Result<()> {
  for dir in dirs {
    match fs::remove_dir(dir) {
      Ok(v) => v,
      Err(e) => return Err(e)
    };
  }

  Ok(())
}
