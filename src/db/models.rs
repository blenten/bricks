use super::pgdb;

pub mod test_in;
pub use test_in::TestIn;
pub mod control_group;
pub use control_group::ControlGroup;
pub mod stat_record;
pub use stat_record::{StatRecord, StatRecordItem};
pub mod stat_cluster;
pub use stat_cluster::StatCluster;

pub trait DbTable {
	type TableItem;
	fn select() -> Vec<Self::TableItem>;
	fn push(items: Vec<Self::TableItem>);
}


pub trait InputRowData {
	fn keystring(&self) -> String;
	fn class(&self) -> &String;
}