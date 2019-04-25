use super::pgdb;

pub mod test_in;
pub use test_in::TestIn;

pub trait DbTable {
	type TableItem;
	fn select() -> Vec<Self::TableItem>;
	fn push(items: Vec<Self::TableItem>);
}


pub trait Inputdata {
	fn keystr(&self) -> &str;
}