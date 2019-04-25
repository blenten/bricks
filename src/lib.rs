use std::error::Error;

mod db;
use db::models::{self, DbTable};


pub fn run() -> Result<(), Box<dyn Error>> {
	println!("Hello, world!");
	let res = models::TestIn::select();
	Ok(())
}