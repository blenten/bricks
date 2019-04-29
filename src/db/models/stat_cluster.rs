use super::{DbTable, pgdb};

pub struct StatCluster {}
impl StatCluster {
	pub fn name() -> &'static str {
		"stat_cluster"
	}

	pub fn push(&self, item: StatClusterItem) -> u64 {
		let conn = pgdb::connect();
		let cluster_id = conn.execute("INSERT INTO $1 (size) VALUES ($2) RETURNING id;", 
			&[&StatCluster::name().to_string(), &item.size.to_string()]);
		cluster_id.unwrap()
	}
}
impl DbTable for StatCluster {
	type TableItem = StatClusterItem;

	fn select() -> Vec<Self::TableItem> {
		let mut result: Vec<Self::TableItem> = Vec::new();
		for row in pgdb::select_all(StatCluster::name()).into_iter() {
			result.push(
				StatClusterItem {
					id: row.get(0),
					size: row.get(1),
					date: row.get(2),
				})
		}
		result
	}

	fn push(items: Vec<Self::TableItem>) {
		unimplemented!()
	}
}


#[derive(Debug)]
pub struct StatClusterItem {
	id: Option<String>,
	size: String,
	date: Option<String>,
}
impl StatClusterItem {
	pub fn new(size: u64) -> StatClusterItem {
		StatClusterItem {
			id: None,
			size: size.to_string(),
			date: None,
		}
	}
}