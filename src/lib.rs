use std::error::Error;

mod db;
use db::models::{self, DbTable, InputRowData};
mod filter;
mod classifier;
use classifier::{Stats, StatsData};

impl<T: InputRowData> Stats for T{
	fn as_stats_data(&self) -> Vec<StatsData> {
		let mut result: Vec<StatsData> = Vec::new();
		for kword in filter::filter_keystring(self.keystring()).into_iter() {
			result.push(StatsData::new(&kword, self.class(), 1));
		}
		result
	}
}
impl Stats for models::StatRecordItem {
	fn as_stats_data(&self) -> Vec<StatsData> {
		let (kw, cl, m) = self.stats();
		vec![StatsData::new(kw, cl, m)]
	}
}

pub fn run() -> Result<(), Box<dyn Error>> {
	println!("Hello, world!");
	let res = models::TestIn::select();
	println!("{:?}", res);
	Ok(())
}