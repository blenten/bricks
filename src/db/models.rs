use std::collections::HashMap;
use super::pgdb;


pub fn from_name(tname: &str) -> Result<DbTable, String>{
	match tname {
		"oks_in" => Ok(DbTable::new("oks_in", Model::OksIn)),
		"stat_record" => Ok(DbTable::new("stat_record", Model::StatRecord)),
		"stat_cluster" => Ok(DbTable::new("stat_cluster", Model::StatCluster)),
		"load_in" => Ok(DbTable::new("load_in", Model::LoadIn)),
		x => Err(format!("No table named: {}", x))
	}
}

pub struct DbTable {
	name: &'static str,
	model: Model,
}
impl DbTable {

	fn new(name: &'static str, model: Model) -> DbTable {
		DbTable {name, model}
	}

	pub fn name(&self) -> &str {
		self.name
	}

	pub fn fields(&self) -> &'static [&'static str] {
		self.model.fields()
	}

	pub fn select(&self) -> Vec<TableRow> {
		let fnames = self.model.fields().to_vec();
		let rows = pgdb::select_all(self.name);
		let mut result: Vec<TableRow> = Vec::with_capacity(rows.len());

		for row in rows.into_iter() {
			let mut fields: HashMap<&'static str, String> = HashMap::with_capacity(fnames.len());
			let mut cpstr = String::new();
			for i in 1..row.len() {
				let item: String = row.get(i);
				cpstr += &item;
				if i == row.len() - 1 {
					cpstr.push('\n')
				}else {
					cpstr.push('\t')
				}
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

	pub fn push(&self, rows: Vec<TableRow>) -> Result<u64, String> {
		if rows.is_empty() {
			return Ok(0)
		}
		if rows[0].rowlen != self.model.fields().len() {
			return Err(format!("Table model is incompatible with the row:\n\t{:?}\n\t{:?}",
				self.model.fields(),
				rows[0].fields))
		}

		Ok(self.model.method().push(self.name, self.model.fields(), rows))
	}

	pub fn newrow(&self, values: Vec<String>) -> Result<TableRow, String> {
		let fnames = self.model.fields().to_vec();
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


enum Model {
	OksIn,
	LoadIn,
	StatRecord,
	StatCluster,
}
impl Model {
	fn fields(&self) -> &'static [&'static str] {
		use Model::*;
		match self {
			OksIn => &["name", "sprav", "area", "year", "material", "levels", "cn", "class"],
			LoadIn => &["cn", "object_name", "name"],
			StatRecord => &["kword", "class", "matches", "cluster_id"],
			StatCluster => &["size"],
		}
	}

	fn method(&self) -> InsertMethod {
		use Model::*;
		match self {
			OksIn => InsertMethod::CopyIn,
			LoadIn => InsertMethod::CopyIn,
			StatRecord => InsertMethod::CopyIn,
			StatCluster => InsertMethod::InsertOne,
		}
	}
}


enum InsertMethod {
	CopyIn,
	InsertOne,
}
impl InsertMethod {
	fn push(&self, tname: &str, tfields: &[&'static str], mut rows: Vec<TableRow>) -> u64 {
		use InsertMethod::*;
		match self {
			CopyIn => pgdb::copy_in(tname, tfields, rows),
			InsertOne => pgdb::insert_one(tname, tfields,rows.remove(0)),
		}
	}
}


#[derive(Debug)]
pub struct TableRow {
	fields: HashMap<&'static str, String>,
	cpstr: String,
	rowlen: usize,
}

impl TableRow {
	pub fn get(&self, field_name: &str) -> Result<&String, String> {
		self.fields.get(field_name)
			.ok_or(format!("No field '{}' found\n\t{:?}", field_name, self.fields))
	}

	pub fn len(&self) -> usize {
		self.rowlen
	}

	pub fn to_cpstr(self) -> String {
		self.cpstr
	}
}




#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn newrow_oks() {
		let table = DbTable::new("t1", Model::OksIn);
		let vals: Vec<String> = table.fields()
			.iter()
			.enumerate()
			.map(|(i, _)| {format!("value{}", i+1)})
			.collect();
		let res_fields: HashMap<&'static str, String> = table.fields().to_vec()
			.into_iter()
			.zip(vals.clone())
			.collect();

		let row = table.newrow(vals).unwrap();

		assert_eq!(row.cpstr, 
			"value1\tvalue2\tvalue3\tvalue4\tvalue5\tvalue6\tvalue7\tvalue8\n".to_string());
		assert_eq!(row.rowlen, 8usize);
		assert_eq!(row.fields, res_fields);
	}
}