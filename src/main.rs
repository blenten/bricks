extern crate clap;
use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
	let config = App::new("Bricks")
		.version("0.6")
		.subcommand(SubCommand::with_name("load")
			.arg(Arg::with_name("file")
				.required(true)
				.index(1)
				.help("Input file <ftype:fname>"))
			.arg(Arg::with_name("table")
				.required(true)
				.index(2)
				.help("Table for input <tname>"))
			.arg(Arg::with_name("control")
				.short("c")
				.takes_value(true)
				.help("Control group table")))
		.subcommand(SubCommand::with_name("train")
			.arg(Arg::with_name("input")
				.required(true)
				.index(1)
				.help("Input table <tname>"))
			.arg(Arg::with_name("output")
				.short("o")
				.long("output")
				.required(true)
				.takes_value(true)
				.multiple(true)
				.help("Output stats tables")))
		.subcommand(SubCommand::with_name("classify")
			.arg(Arg::with_name("input")
				.required(true)
				.index(1)
				.help("Input table <tname>"))
			.arg(Arg::with_name("stats")
				.required(true)
				.index(2)
				.help("Stats table <tname>"))
			.arg(Arg::with_name("output")
				.short("o")
				.long("output")
				.takes_value(true)
				.help("Output file")))
		.get_matches();

    if let Err(e) = bricks::run(config) {
    	eprintln!("Application error: {:?}", e);
    	process::exit(1);
    }
}
