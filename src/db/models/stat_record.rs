use super::{DbTable, pgdb};

pub struct StatRecord {}
impl StatRecord {
	pub fn name() -> &'static str {
		"stat_record"
	}
}

impl DbTable for StatRecord {
	type TableItem = StatRecordItem;

	fn select() -> Vec<Self::TableItem> {
		let mut result: Vec<Self::TableItem> = Vec::new();
		for row in pgdb::select_all(StatRecord::name()).into_iter() {
			result.push(
				StatRecordItem {
					kword: row.get(1),
					class: row.get(2),
					matches: row.get(3),
					cluster_id: row.get(4),
				})
		}
		result
	}

	fn push(items: Vec<Self::TableItem>) {
		for chunk in items.as_slice().chunks(10000) {
			pgdb::copy_in(StatRecord::name(), chunk);
		}
	}
}


#[derive(Debug)]
pub struct StatRecordItem {
	kword: String,
	class: String,
	matches: String,
	cluster_id: String,
}
impl StatRecordItem {
	pub fn stats(&self) -> (&String, &String, u64) {
		(&self.kword, &self.class, self.matches.parse().unwrap())
	}
}

impl pgdb::CopyFrom for StatRecordItem {
	fn to_cpfmt(&self) -> String {
		format!("{}\t{}\t{}\t{}\n", 
			self.kword,
			self.class,
			self.matches,
			self.cluster_id)
	}
}