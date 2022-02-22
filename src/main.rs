use plxml::run;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Err(e) = run(&args[1]) {
        panic!("Error occurred: {}", e);
    }
}
