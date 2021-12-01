use askama::Template;
use chrono::{DateTime, Utc};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use serde::Deserialize;
use std::{error, fmt, fs, io::Write, path::Path};

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
	filename: String, // we don't really need this
}

/// The inputs are the directories outputted from FastQC after unzipping.
/// Each directory contains the report for a sample.
fn main() -> Result<(), Box<dyn error::Error>> {
	let data_dir = Path::new("data");
	let outfile = data_dir.join("report_aggregated.html");

	let paths = fs::read_dir(data_dir)?;

	// this is just for per base sequence quality for now
	let mut samples = FxHashMap::<String, Flag>::default();

	// Get a directory list of the sample directories.
	for sample in paths {
		let sample = sample?.path();
		if !sample.is_dir() {
			continue;
		}
		let dir = sample
			.file_name()
			.unwrap()
			.to_str()
			.unwrap()
			.split("_fastqc")
			.collect::<Vec<&str>>()[0]
			.to_owned();

		'outer: for file_path in fs::read_dir(sample).unwrap() {
			let direntry = file_path?;
			match direntry.file_name().to_str().unwrap() {
				"summary.txt" => {
					let file = fs::File::open(direntry.path())?;
					let mut rdr = csv::ReaderBuilder::new()
						.delimiter(b'\t')
						.has_headers(false)
						.from_reader(file);
					for res in rdr.deserialize::<Summary>() {
						let res = res?;
						if res.test == "Per base sequence quality" {
							// TODO: Remove clone. It's inexpensive but can be avoided
							samples.insert(dir.clone(), res.flag);
							continue 'outer;
						}
					}
				}
				_ => (),
			}
		}
	}

	for key in samples.keys().sorted() {
		println!("{}", key);
	}

	let html = ReportTemplate::new(samples).render()?;
	let mut file = fs::File::create(outfile)?;
	file.write_all(html.to_string().as_bytes())?;
	Ok(())
}

#[derive(Template)]
#[template(path = "report.html")]
struct ReportTemplate {
	sample_dirs: FxHashMap<String, Flag>,
	dt: DateTime<Utc>,
}

impl ReportTemplate {
	fn new(sample_dirs: FxHashMap<String, Flag>) -> Self {
		ReportTemplate {
			sample_dirs,
			dt: Utc::now(),
		}
	}
}
