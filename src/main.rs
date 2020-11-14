use std::env;
use std::fs;
use std::io;

fn main() {
    let copy = match env::args().nth(1) {
        Some(arg) => copy_file(arg.clone()),
        None => Err("One path argument is required".into()),
    };

    match copy {
        Ok(_) => println!("Copy was successful!"),
        Err(err) => println!("Error: {:?}", err),
    }
}

fn copy_file(arg: String) -> Result<(), Box<dyn std::error::Error>> {
    let files = env::current_exe()?.parent().unwrap().join("files");
    let _ = fs::create_dir(&files);
    let path = files.join(arg.clone());

    if path.exists() {
        let out = {
            let mut cd = env::current_dir()?;
            cd.push(arg.clone());
            cd
        };

        if path.is_dir() {
            return Err("Only files are currently supported".into());
        } else {
            fs::copy(path, out)?;
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, path.to_str().unwrap()).into());
    }

    Ok(())
}
