use std::{env, error::Error, fs, path::PathBuf, process};

pub struct Config {
    file_path: PathBuf,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let file_path: PathBuf = match args.len() {
            1 => env::current_dir()?,

            2 => PathBuf::from(args[1].clone()),

            _ => {
                eprintln!("\nusage: ls <path> ");
                process::exit(1);
            }
        };
        Ok(Config { file_path })
    }

    pub fn path(self) -> PathBuf {
        self.file_path
    }
}

pub fn get_dir_files(path: PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();
    let dir = fs::read_dir(path)?;
    for file in dir {
        files.push(file?.file_name().to_str().unwrap().to_string());
    }

    Ok(files)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_list = get_dir_files(config.path())?;
    for file in file_list {
        println!("{}", file)
    }

    Ok(())
}
