pub mod oks1;
mod csvrw;


pub enum BFile {
	OKS1(Vec<oks1::RowOKS1>),
}

pub fn get_file(filearg: &str) -> Result<BFile, String> {
	let args: Vec<&str> = filearg.split(':').collect();
	match args[0] {
		"oks1" => Ok(BFile::OKS1(oks1::from_csv(args[1]))),
		e => return Err(format!("Filetype {} is undefined.", e)),
	}
}