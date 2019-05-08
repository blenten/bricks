use super::{DbTable, ControlGroup, TableRow};


pub struct TestIn;

impl DbTable for TestIn {

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

impl ControlGroup for TestIn {

	fn cgroup() -> &'static str {
		"test_in_control"
	}

	fn select_cg() -> Vec<TableRow> {
		unimplemented!()
	}

	fn push_cg(items: Vec<TableRow>) {
		unimplemented!()
	}
}