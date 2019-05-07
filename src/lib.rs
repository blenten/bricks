mod classifier;
mod filter;
mod db;
mod files;
extern crate clap;

use {
	clap::ArgMatches,
	std::{env, error::Error},
	db::models::{self, DbTable, InputRowData},
	classifier::StatsData,
};


pub fn run(config: ArgMatches) -> Result<(), Box<dyn Error>> {
	Ok(())
}


#[derive(Debug)]
struct Config {
	command: Option<String>,
	input: Option<String>,
	table: Option<String>,
	options: Option<Vec<String>>,
}