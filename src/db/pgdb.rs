extern crate postgres;
use self::postgres::{Connection, TlsMode, rows::Rows};
use super::models::TableRow;


pub fn connect() -> Connection {
	let connstr = "postgresql://postgres:88005553535@192.168.50.70/sonotest";
	Connection::connect(connstr, TlsMode::None).unwrap()
}

pub fn select_all(tname: &str) -> Rows {
	let conn = connect();
	let cmd = format!("SELECT * FROM {:?}", tname);
	conn.query(cmd.as_str(), &[]).unwrap()
}

pub fn copy_in(tname: &str, fields: &str, items: Vec<TableRow>) -> u64 {
	let cpstr: String = items.into_iter().map(|x| {x.to_cpstr()}).collect();

	let conn = connect();
	let cmd = format!("COPY {} {} FROM STDIN", tname, fields);
	let stmt = conn.prepare(cmd.as_str()).unwrap();
	
	stmt.copy_in(&[], &mut cpstr.as_bytes()).unwrap()
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn connect_tst() {
		connect();
	}
}
