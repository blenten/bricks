use super::pgdb;

pub mod test_in;
pub mod control_group;
pub mod stat_record;
pub mod stat_cluster;

pub use self::{
	test_in::TestIn,
	control_group::ControlGroup,
	stat_record::{StatRecord, StatRecordItem},
	stat_cluster::StatCluster,
};

pub trait DbTable {
	type TableItem;
	fn select() -> Vec<Self::TableItem>;
	fn push(items: Vec<Self::TableItem>);
}

pub trait InputRowData {
	fn keystring(&self) -> String;
	fn class(&self) -> &String;
}