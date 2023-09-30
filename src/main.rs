use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args);

    println!("Serach for {}", config.query);
    println!("In file {}", config.file_path);
    println!("File content:\n{}", read_file(&config.file_path));
    // config::show_unfo();
    
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config {query, file_path}
    }
}


fn read_file(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Error while reading file");

    content
}
// fn get_args() -> Vec<String> {
//     let args: Vec<String> = env::args().collect();
//     args
// }
//
// fn read_file_content(file_path: &str) -> String {
//     let content = fs::read_to_string(file_path).
//         expect("Error while reading file");
//     content
// }
