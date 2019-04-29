use std::collections::HashMap;


pub trait Stats {
	fn as_stats_data(&self) -> Vec<StatsData>;
}


#[derive(Debug)]
pub struct Statistics {
	kw_cl_matches: HashMap<(String, String), u64>,
	cl_matches: HashMap<String, u64>,
	samples: u64,
}
impl Statistics {
	pub fn new() -> Statistics {
		Statistics {
			kw_cl_matches: HashMap::new(),
			cl_matches: HashMap::new(),
			samples: 0,
		}
	}

	pub fn add_data(&mut self, data: StatsData) {
		
		let kwclm = self.kw_cl_matches.entry((data.kword, data.class.clone())).or_insert(0);
		let clm = self.cl_matches.entry(data.class).or_insert(0);

		*kwclm += data.matches;
		*clm += data.matches;
		self.samples += data.matches;
	}
}


pub struct StatsData {
	kword: String,
	class: String,
	matches: u64,
}
impl StatsData {
	pub fn new(kword: &str, class: &str, matches: u64) -> StatsData {
		StatsData {	
			kword: kword.to_string(), 
			class: class.to_string(), 
			matches,
		}
	}
}


pub struct Probs {
	p_match: HashMap<(String, String), f64>,
	p_cl: HashMap<String, f64>
}
impl Probs {
	pub fn from_stats(stats: &Statistics) -> Probs {
		let p_match = stats.kw_cl_matches.iter()
			.map(|i| {
			let (k, v) = (i.0, i.1);
			((*k).clone(), *v as f64 / *stats.cl_matches.get(&k.1).unwrap() as f64)
			}).collect::<HashMap<(String, String), f64>>();

		let p_cl = stats.cl_matches.iter()
			.map(|i| {
			let (k, v) = (i.0, i.1);
			((*k).clone(), *v as f64 / stats.samples as f64)
			}).collect::<HashMap<String, f64>>();

		Probs {
			p_match,
			p_cl,
		}
	}

	pub fn classify(&self, keywords: Vec<String>) ->  Vec<&String>{
		let mut result = Vec::new();
		for (k, v) in self.p_cl.iter() {
			let p_prior = -(v + 1e-27).ln();
			let p_post: f64 = self.p_match.iter()
				.filter(|i| {(i.0).1 == *k && keywords.contains(&(i.0).0)})
				.map(|i| {-(i.1 + 1e-27).ln()}).sum();

			result.push((k, p_prior + p_post));
		}
		result.sort_unstable_by(|a, b| {a.1.partial_cmp(&b.1).unwrap()});
		result.truncate(3);
		let mut result: Vec<&String> = result.into_iter().map(|i| {i.0}).collect();
		result.shrink_to_fit();
		result
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn stats_add_data_simple() {
		let mut stats = Statistics::new();

		stats.add_data(StatsData::new("kword", "class", 300));

		assert_eq!(
			stats.kw_cl_matches.get(&("kword".to_string(), "class".to_string())),
			Some(&300u64));
		assert_eq!(
			stats.cl_matches.get("class"),
			Some(&300u64));
		assert_eq!(stats.samples, 300u64);
	}

	#[test]
	fn stats_add_data_multi() {
		let mut stats = Statistics::new();

		stats.add_data(StatsData::new("kword1", "class", 100));
		stats.add_data(StatsData::new("kword2", "class", 100));
		stats.add_data(StatsData::new("kword3", "class2", 100));

		assert_eq!(
			stats.kw_cl_matches.get(&("kword1".to_string(), "class".to_string())),
			Some(&100u64));
		assert_eq!(
			stats.kw_cl_matches.get(&("kword2".to_string(), "class".to_string())),
			Some(&100u64));
		assert_eq!(
			stats.kw_cl_matches.get(&("kword3".to_string(), "class2".to_string())),
			Some(&100u64));
		assert_eq!(
			stats.cl_matches.get("class"),
			Some(&200u64));
		assert_eq!(
			stats.cl_matches.get("class2"),
			Some(&100u64));
		assert_eq!(stats.samples, 300u64);
	}

	#[test]
	fn probs_from_stats() {
		let mut stats = Statistics::new();
		stats.add_data(StatsData::new("kw1", "cl1", 1));
		stats.add_data(StatsData::new("kw2", "cl1", 3));
		stats.add_data(StatsData::new("kw1", "cl2", 2));
		stats.add_data(StatsData::new("kw2", "cl2", 1));
		
		let probs = Probs::from_stats(&stats);

		assert_eq!(
			*probs.p_cl.get("cl1").unwrap(),
			4.0/7.0f64);
		assert_eq!(
			*probs.p_cl.get("cl2").unwrap(),
			3.0/7.0f64);

		assert_eq!(
			*probs.p_match.get(&("kw1".to_string(), "cl1".to_string())).unwrap(),
			0.25f64);
		assert_eq!(
			*probs.p_match.get(&("kw2".to_string(), "cl1".to_string())).unwrap(),
			0.75f64);
		assert_eq!(
			*probs.p_match.get(&("kw1".to_string(), "cl2".to_string())).unwrap(),
			2.0/3.0f64);
		assert_eq!(
			*probs.p_match.get(&("kw2".to_string(), "cl2".to_string())).unwrap(),
			1.0/3.0f64);
	}

	#[test]
	fn classify_tst() {
		let mut stats = Statistics::new();
		stats.add_data(StatsData::new("kw1", "cl1", 1));
		stats.add_data(StatsData::new("kw2", "cl1", 3));
		stats.add_data(StatsData::new("kw1", "cl2", 2));
		stats.add_data(StatsData::new("kw2", "cl2", 1));
		
		let probs = Probs::from_stats(&stats);

		let input1 = vec!["kw2".to_string(), "kw1".to_string()];
		let input2 = vec!["kw1".to_string()];

		assert_eq!(probs.classify(input1), vec!["cl1", "cl2"]);
		assert_eq!(probs.classify(input2), vec!["cl2", "cl1"]);
	}
}