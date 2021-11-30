use std::{fs, path::Path};

use serde::Deserialize;

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
fn main() {
	let data_dir = Path::new("data");

	let paths = fs::read_dir(data_dir).unwrap();

	let mut summaries = Vec::<Summary>::new();

	// Check all paths in the data directory.
	for sample_dir_path in paths {
		let sample_dir_path = sample_dir_path.unwrap().path();

		if !sample_dir_path.is_dir() {
			continue;
		}

		for path in fs::read_dir(sample_dir_path).unwrap() {
			let path = path.unwrap().path();

			match path.file_name().unwrap().to_str().unwrap() {
				"summary.txt" => {
					let file = fs::File::open(path).unwrap();
					let mut rdr = csv::ReaderBuilder::new()
						.delimiter(b'\t')
						.has_headers(false)
						.from_reader(file);
					for res in rdr.deserialize::<Summary>() {
						summaries.push(res.unwrap());
					}
				}
				_ => (),
			}
		}
	}
	println!("{:?}", summaries);
}
