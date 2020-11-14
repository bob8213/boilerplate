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
            let file = fs::read_to_string(&path)?;
            let mut input = String::new();
            let stdin = io::stdin();
            let mut content = String::new();

            for (i, s) in file.split("%?%").enumerate() {
                content.push_str(if (i & 1) != 0 {
                    println!("'{}':", s);
                    input.clear();
                    stdin.read_line(&mut input).unwrap();
                    input.trim_end()
                } else {
                    s
                });
            }

            fs::write(out, content)?;
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, path.to_str().unwrap()).into());
    }

    Ok(())
}
