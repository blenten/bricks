extern crate postgres;
use crate::db::models::TableRow;
use std::cmp::min;
use self::postgres::{Connection, TlsMode, rows::Rows};


pub fn connect() -> Connection {
	let connstr = "postgresql://postgres:88005553535@192.168.50.70/sonotest";
	Connection::connect(connstr, TlsMode::None).unwrap()
}


pub fn select_all(tname: &str) -> Rows {
	let conn = connect();
	let cmd = format!("SELECT * FROM {:?}", tname);
	conn.query(cmd.as_str(), &[]).unwrap()
}


pub fn copy_in(tname: &str, fields: &[&'static str], mut rows: Vec<TableRow>) -> u64 {
	let conn = connect();
	let cmd = format!("COPY {} {} FROM STDIN", tname, fields.join(", "));
	let stmt = conn.prepare(cmd.as_str()).unwrap();
	let mut rownum = 0;
	let mut cpstr = String::with_capacity(100);

	while !rows.is_empty() {
		cpstr.clear();
		cpstr = rows.drain(..min(10_000, rows.len())).map(|x| {x.to_cpstr()}).collect();
		rownum += stmt.copy_in(&[], &mut cpstr.as_bytes()).unwrap();
	}
	rownum
}

pub fn insert_one(tname: &str, fields: &[&'static str], row: TableRow) -> u64 {
	let conn = connect();
	let return_id = conn.execute("INSERT INTO $1 ($2) VALUES ($3) RETURNING id;", 
		&[
			&tname.to_string(),
			&fields.join(", "), 
			&fields.iter()
				.map(|f| {row.get(f).unwrap().to_string()})
				.collect::<Vec<String>>()
				.join(", "),
		]);
	return_id.unwrap()
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn connect_tst() {
		connect();
	}
}
