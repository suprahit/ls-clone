use std::{env, error::Error, path::PathBuf, process};

pub struct Config {
    file_path: PathBuf,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let file_path: PathBuf = match args.len() {
        
            1 => {
                env::current_dir()?
            }

            2 => {
                PathBuf::from(args[1].clone())
            }

            _ => {
                eprintln!("Too much arguments!\nusage: ls <path> ");
                process::exit(1);
            }
        };
        Ok(Config { file_path })
    }

    pub fn path(self) -> PathBuf {
        self.file_path
    }
}


