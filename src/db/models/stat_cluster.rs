use super::{DbTable, TableRow, pgdb};

pub struct StatCluster;

impl DbTable for StatCluster {
	fn name() -> &'static str {
		"stat_cluster"
	}

	fn fields() -> &'static [&'static str] {
		&["size"]
	}

	// fn fields_number() -> usize {
	// 	1
	// }

	fn push(items: Vec<TableRow>) -> u64 {
		let conn = pgdb::connect();
		let cluster_id = conn.execute("INSERT INTO $1 (size) VALUES ($2) RETURNING id;", 
			&[&StatCluster::name().to_string(), &items[0].get("size").clone()]);
		cluster_id.unwrap()
	}
}