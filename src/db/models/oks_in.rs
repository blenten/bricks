use super::{DbTable, ControlGroup, TableRow};


pub struct OksIn;

impl DbTable for OksIn {

	fn name() -> &'static str {
		"test_in"
	}

	fn fields() -> &'static [&'static str] {
		&["name", "sprav", "area", "year", "material", "levels", "cn", "class"]
	}

	// fn fields_number() -> usize {
	// 	8
	// }
}

impl ControlGroup for OksIn {

	fn cgroup() -> &'static str {
		"oks_in_cgroup"
	}
}