mod classifier;
mod filter;
mod db;

use {
	std::time::Instant,
	db::models::{
		self, 
		DbTable,
		TableRow
	},
	classifier::{
		Statistics,
		StatsData,
	}
};


pub fn run(args: Vec<String>) -> Result<(), String> {
	let now = Instant::now();
	let config = Config::from_args(args)?;
	match config.mode {	
		Mode::Train => train(config)?,
		Mode::Classify => classify(config)?,
	};

	println!("Time: {}", now.elapsed().as_secs());
	Ok(())
}

fn train(config: Config) -> Result<(), String> {
	let input = config.input.select();
	let mut stats = Statistics::new();
	for stdata in input.into_iter() {
		stats.add_datavec(config.dataset.get_data_from(stdata)?);
	}
	Ok(())
}

fn classify(config: Config) -> Result<(), String> {
	unimplemented!()
}


struct Config {
	mode: Mode,
	input: DbTable,
	output: DbTable,
	dataset: Dataset,
}

impl Config {
	fn from_args(args: Vec<String>) -> Result<Config, String> {
		if args.len() < 5 {
			return Err("Not enough arguments, at least 4 expected".to_string());
		}
		Ok(Config {
			mode: Mode::from_name(&args[1])?,
			input: models::from_name(&args[2])?,
			output: models::from_name(&args[3])?,
			dataset: Dataset::from_name(&args[4])?,
		})
	}
}


enum Mode {
	Train,
	Classify,
}
impl Mode {
	fn from_name(name: &str) -> Result<Mode, String> {
		match name {
			"train" => Ok(Mode::Train),
			"classify" => Ok(Mode::Classify),
			x => Err(format!("No mode named: {}", x))
		}
	}
}


#[derive(Debug)]
enum Dataset {
	OksV1,
	OksV2,
	Load
}

impl Dataset {

	fn from_name(name: &str) -> Result<Dataset, String> {
		match name {
			"oks1" => Ok(Dataset::OksV1),
			"oks2" => Ok(Dataset::OksV2),
			"load" => Ok(Dataset::Load),
			x => Err(format!("No dataset named: {}", x))
		}
	}

	fn data_from_strec(table_row: TableRow) -> Result<StatsData, String> {
		Ok(StatsData::new(
			table_row.get("kword")?, 
			table_row.get("class")?, 
			table_row.get("matches")?.parse()
				.map_err(|err: std::num::ParseIntError| {err.to_string()})?),
		)
	}

	fn get_data_from(&self, table_row: TableRow) -> Result<Vec<StatsData>, String> {
		let mut kwords: Vec<String> = match self {
			OksV1 => filter::filter_keystring(
				table_row.get("name")?.to_string() + table_row.get("sprav")?
			),
			OksV2 => {
				let mut res = vec![
					table_row.get("area")?.to_string(),
					table_row.get("year")?.to_string(),
					table_row.get("material")?.to_string()
				];
				res.append(&mut table_row.get("levels")?
					.split_whitespace()
					.map(|x|{x.to_string()})
					.collect::<Vec<String>>());
				res
			},
			Load => filter::filter_keystring(
				table_row.get("name")?.to_string() + table_row.get("object_name")?
			),
		};
		let mut result: Vec<StatsData> = Vec::new();
		for kword in kwords.into_iter() {
			result.push(StatsData::new(kword.as_str(), table_row.get("class")?, 1));
		}
		Ok(result)
	}
}