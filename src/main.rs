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
    let path = env::current_exe()?.parent().unwrap().join(arg.clone());

    if path.exists() {
        let out = {
            let mut cd = env::current_dir()?;
            cd.push(arg.clone());
            cd
        };

        if path.is_dir() {
            // TODO
        } else {
            // Let's assume that all dirs on the path exist
            fs::copy(path, out)?;
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, path.to_str().unwrap()).into());
    }

    Ok(())
}
