use super::csvrw;

pub fn from_csv(filename: &str) ->  Vec<RowOKS1> {
	let mut result = Vec::new();
	for (i, row) in csvrw::read_csv(filename).enumerate() {
		let row = match row {
			Ok(res) => res,
			Err(e) => {
				eprintln!("Could not read row {}\n\t{}", i, e);
				continue;
			},
		};

		result.push(RowOKS1 {
			name: row[3].to_string(),
			sprname: row[4].to_string(),
			area: row[6].to_string(),
			year: row[7].to_string(),
			material: row[8].to_string(),
			levels: row[9].to_string(),
			cn: row[0].to_string(),
			class: row[2].to_string(),
		})
	}
	result
}


#[derive(Debug)]
pub struct RowOKS1 {
	pub name: String,
	pub sprname: String,
	pub area: String,
	pub year: String,
	pub material: String,
	pub levels: String,
	pub cn: String,
	pub class: String,
}