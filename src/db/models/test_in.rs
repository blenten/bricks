use super::{DbTable, InputRowData, pgdb};


pub struct TestIn;
impl TestIn {
	pub fn name() -> &'static str {
		"test_in"
	}
}


impl DbTable for TestIn {
	type TableItem = TestInItem;

	fn select() -> Vec<Self::TableItem> {
		let mut result: Vec<Self::TableItem> = Vec::new();
		for row in pgdb::select_all(TestIn::name()).into_iter() {
			result.push(TestInItem {
				cn: row.get(6),
				util: row.get(1),
				sprav: row.get(2),
				result: row.get(3),
				v1: row.get(4),
				v2: row.get(5),
			})
		}
		result
	}

	fn push(items: Vec<Self::TableItem>){
		for chunk in items.as_slice().chunks(10000) {
			pgdb::copy_in(TestIn::name(), chunk);
		}
	}
}


#[derive(Debug)]
pub struct TestInItem {
	cn: String,
	util: String,
	sprav: String,
	result: String,
	v1: String,
	v2: String,
}

impl  InputRowData for TestInItem {
	fn keystring(&self) -> String {
		self.util.clone() + &self.v1
	}

	fn class(&self) -> &String {
		&self.result
	}
}

impl pgdb::CopyFrom for TestInItem {
	fn to_cpfmt(&self) -> String {
		format!("{}\t{}\t{}\t{}\t{}\t{}\n", 
			self.util,
			self.sprav,
			self.result,
			self.v1,
			self.v2,
			self.cn)
	}
}