mod classifier;
mod filter;
mod db;
mod files;
extern crate clap;

use {
	clap::ArgMatches,
	db::models::{
		self, 
		DbTable
	},
};


pub fn run(config: ArgMatches) -> Result<(), String> {
	
	if let Some(matches) = config.subcommand_matches("load")  {

		let filearg = matches.value_of("file").ok_or("No filearg")?;
		let out_table = matches.value_of("table").ok_or("No out_table")?;
		let cgroup = matches.is_present("control");

		println!("Loading {}", filearg);
		return load(filearg, out_table, cgroup);
	}
	if let Some(matches) = config.subcommand_matches("train") {
		println!("train");
		return Ok(())
	}
	if let Some(matches) = config.subcommand_matches("classify") {
		println!("classify");
		return Ok(())
	}
	Err("None of the commands matched.".to_string())
}


fn load(input_file: &str, output_table: &str, cgroup: bool) -> Result<(), String> {
	Ok(())
}