use askama::Template;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::{error, fs, io::Write, path::Path};

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

/// The inputs are the directories outputted from FastQC after unzipping.
/// Each directory contains the report for a sample.
fn main() -> Result<(), Box<dyn error::Error>> {
	let data_dir = Path::new("data");
	let outfile = data_dir.join("report_aggregated.html");

	let paths = fs::read_dir(data_dir)?;

	let mut sample_dirs = Vec::<String>::new();

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
		sample_dirs.push(dir);
	}

	let html = ReportTemplate::new(sample_dirs).render()?;
	let mut file = fs::File::create(outfile)?;
	file.write_all(html.to_string().as_bytes())?;
	Ok(())
}

#[derive(Template)]
#[template(path = "report.html")]
struct ReportTemplate {
	sample_dirs: Vec<String>,
	dt: DateTime<Utc>,
}

impl ReportTemplate {
	fn new(sample_dirs: Vec<String>) -> Self {
		ReportTemplate {
			sample_dirs,
			dt: Utc::now(),
		}
	}
}
