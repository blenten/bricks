use super::DbTable;

pub struct StatRecord;

impl DbTable for StatRecord {

	fn name() -> &'static str {
		"stat_record"
	}

	fn fields() -> &'static [&'static str] {
		&["kword", "class", "matches", "cluster_id"]
	}

	// fn fields_number() -> usize {
	// 	4
	// }
}