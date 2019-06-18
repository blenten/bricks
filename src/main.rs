use std::{
	process,
	env,
};

fn main() {
	
    if let Err(e) = bricks::run(env::args().collect()) {
    	eprintln!("Application error: {:?}", e);
    	process::exit(1);
    }
}
