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

	// Check all paths in the data directory.
	for path in paths {
		let folder_path = path.unwrap().path();

		if !folder_path.is_dir() {
			continue;
		}

		for file_path in fs::read_dir(folder_path).unwrap() {
			let direntry = file_path.unwrap();
			match direntry.file_name().to_str().unwrap() {
				"summary.txt" => {
					let file = fs::File::open(direntry.path()).unwrap();
					let mut rdr = csv::ReaderBuilder::new()
						.delimiter(b'\t')
						.has_headers(false)
						.from_reader(file);
					for result in rdr.deserialize::<Summary>() {
						println!("{:?}", result.unwrap());
					}
				}
				_ => (),
			}
		}
	}
}
