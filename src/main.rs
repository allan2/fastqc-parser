use rustc_hash::FxHashSet;
use serde::Deserialize;
use std::{fs, path::Path, ffi::OsString};

#[derive(Debug, Deserialize)]
enum Flag {
	#[serde(rename = "PASS")]
	Pass,
	#[serde(rename = "FAIL")]
	Fail,
	#[serde(rename = "WARN")]
	Warn,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Summary {
	flag: Flag,
	test: String,
	filename: String,
}

/// The inputs are the directories outputted from FastQC. Each directory contains the report for a sample.
fn main() {
	let data_dir = Path::new("data");
	let paths = fs::read_dir(data_dir).unwrap();
	let mut samples = FxHashSet::<OsString>::default();

	// Check all paths in the data directory.
	for sample in paths {
		let filename = sample.unwrap().file_name();

		// if !sample_dir_path.is_dir() {
		// 	continue;
		// }

		//let curr = sample_dir_path.to_owned();

		//samples.insert(curr.file_name().unwrap().to_str().unwrap());

		// for path in fs::read_dir(sample_dir_path).unwrap() {
		// 	let path = path.unwrap().path();
		// 	match path.file_name().unwrap().to_str().unwrap() {
		// 		"summary.txt" => {
		// 			let file = fs::File::open(path).unwrap();
		// 			let mut rdr = csv::ReaderBuilder::new()
		// 				.delimiter(b'\t')
		// 				.has_headers(false)
		// 				.from_reader(file);
		// 			for res in rdr.deserialize::<Summary>() {
		// 				summaries.push(res.unwrap());
		// 			}
		// 		}
		// 		_ => (),
		// 	}
		// }
	}
	//println!("{:?}", summaries);
}
