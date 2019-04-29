use std::process;

fn main() {
    if let Err(e) = bricks::run() {
    	eprintln!("Application error: {:?}", e);
    	process::exit(1);
    }
}
