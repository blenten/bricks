use std::collections::HashMap;
use std::cmp::min;
use super::pgdb;

pub mod oks_in;
pub mod stat_record;
pub mod stat_cluster;

pub use self::{
	oks_in::OksIn,
	stat_record::StatRecord,
	stat_cluster::StatCluster,
};


pub trait DbTable {
	fn name() -> &'static str;
	fn fields() -> &'static [&'static str];
	// fn fields_number() -> usize;

	fn select() -> Vec<TableRow> {
		select(Self::name(), Self::fields())		
	}

	fn push(rows: Vec<TableRow>) -> u64 {
		push(Self::name(), Self::fields(), rows)
	}

	fn newrow(values: Vec<String>) -> Result<TableRow, String> {
		let fnames = Self::fields().to_vec();
		if fnames.len() != values.len() {
			return Err(format!("Wrong number of values. Expected {}, got {}", 
				fnames.len(), values.len()));
		}

		Ok(TableRow {
			cpstr: values.as_slice().join("\t") + "\n",
			rowlen: fnames.len(),
			fields: fnames.into_iter().zip(values.into_iter()).collect(),
		})
	}
}

pub trait ControlGroup: DbTable {
	fn cgroup() -> &'static str;

	fn select_cg() -> Vec<TableRow> {
		select(Self::cgroup(), Self::fields())
	}

	fn push_cg(rows: Vec<TableRow>) -> u64 {
		push(Self::cgroup(), Self::fields(), rows)
	}
}


#[derive(Debug)]
pub struct TableRow {
	fields: HashMap<&'static str, String>,
	cpstr: String,
	rowlen: usize,
}

impl TableRow {
	pub fn get(&self, field_name: &str) -> Option<&String> {
		self.fields.get(field_name)
	}

	pub fn len(&self) -> usize {
		self.rowlen
	}

	pub fn to_cpstr(self) -> String {
		self.cpstr
	}
}


fn select(tname: &str, fields: &[&'static str]) -> Vec<TableRow> {
	let fnames = fields.to_vec();
	let rows = pgdb::select_all(tname);
	let mut result: Vec<TableRow> = Vec::with_capacity(rows.len());

	for row in rows.into_iter() {
		let mut fields: HashMap<&'static str, String> = HashMap::with_capacity(fnames.len());
		let mut cpstr = String::new();
		for i in 1..row.len() {
			let item: String = row.get(i);
			cpstr += &item;
			fields.insert(fnames.get(i).unwrap(), item);
		}
		result.push(TableRow {
			cpstr,
			fields,
			rowlen: fnames.len(),
		})
	}
	result
}

fn push(tname: &str, fields: &[&'static str], mut rows: Vec<TableRow>) -> u64 {
	let mut rownum = 0;
	while !rows.is_empty() {
		let cpstr = rows.drain(..min(10_000, rows.len())).map(|x| {x.to_cpstr()}).collect();
		rownum += pgdb::copy_in(tname, fields, cpstr);
	}
	rownum
}



#[cfg(test)]
mod tests {
	use super::*;

	struct TestTable;
	impl DbTable for TestTable {
		fn name() -> &'static str {
			"test_table"
		}

		fn fields() -> &'static [&'static str] {
			&["field1", "field2", "field3"]
		}

		// fn fields_number() -> usize {
		// 	3
		// }

		fn select() -> Vec<TableRow> {unimplemented!()}
		fn push(_rows: Vec<TableRow>) -> u64{unimplemented!()}
	}

	#[test]
	fn newrow_tst() {
		let row = TestTable::newrow(vec!["value1".to_string(), 
			"value2".to_string(), "value3".to_string(),]).unwrap();

		let mut res_fields: HashMap<&'static str, String> = HashMap::new();
		res_fields.insert("field1", "value1".to_string());
		res_fields.insert("field2", "value2".to_string());
		res_fields.insert("field3", "value3".to_string());

		assert_eq!(row.cpstr, "value1\tvalue2\tvalue3\n".to_string());
		assert_eq!(row.rowlen, 3usize);
		assert_eq!(row.fields, res_fields);
	}
}