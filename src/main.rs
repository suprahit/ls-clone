use std::env;
use ls_clone::Config;
use std::process;



fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("{:?}", config.path());
}
