extern crate csv;
use self::csv::StringRecordsIntoIter;
use std::process;
use std::fs::File;

pub fn read_csv(filename: &str) -> StringRecordsIntoIter<File> {
    let file = match File::open(filename) {
    	Ok(f) => f,
    	Err(e) => {
    		eprintln!("Could not open {}\n\tError: {}", filename, e);
    		process::exit(1)
    	}
    };
    let rdr = csv::ReaderBuilder::new()
    	.delimiter(b';')
        .has_headers(false)
        .from_reader(file);
    rdr.into_records()
}