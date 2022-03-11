use plxml::run_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Err(e) = run_file(&args[1]) {
        eprintln!("Error occurred: {}", e);
    }
}
