use askama::Template;
use chrono::{DateTime, Utc};
use clap::{App, Arg, ArgMatches};
use csv::Writer;
use serde::Deserialize;
use std::{
	collections::BTreeMap,
	error, fs,
	io::Write,
	path::{Path, PathBuf},
};

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
	let output_file_arg = Arg::new("output_file")
		.long("output_file")
		.short('o')
		.takes_value(true);
	let input_dir_arg = Arg::new("input_dir")
		.long("input_dir")
		.short('i')
		.takes_value(true)
		.required(true)
		.help("Location of FastQC reports");
	let trimmed_dir_arg = Arg::new("trimmed_dir")
		.long("trimmed_dir")
		.short('t')
		.takes_value(true)
		.required(true)
		.help("Location of trimmed FastQC reports");

	let mut app = App::new("fastqc_report")
		.version("0.1")
		.about("Aggregator for FastQC reports")
		.subcommand(
			App::new("aggregate-report")
				.arg(
					output_file_arg
						.clone()
						.default_value("aggregate_report.html")
						.help("Output file for the report"),
				)
				.arg(input_dir_arg.clone())
				.arg(trimmed_dir_arg.clone()),
		)
		.subcommand(
			App::new("trim-length")
				.arg(
					output_file_arg
						.default_value("trimmed_length_deltas.csv")
						.help("Output file for the trimmed length deltas"),
				)
				.arg(input_dir_arg)
				.arg(trimmed_dir_arg),
		);

	let matches = app.clone().get_matches();

	match matches.subcommand() {
		Some(("aggregate-report", submatches)) => {
			let (outfile, data_dir, trimmed_dir) = dests_from_argmatches(submatches);
			let samples = samples_map(data_dir)?;

			let trimmed = samples_map(trimmed_dir)?;
			let html = ReportTemplate::new(samples, trimmed).render()?;
			let mut file = fs::File::create(outfile)?;
			file.write_all(html.to_string().as_bytes())?;
		}
		Some(("trim-length", submatches)) => {
			let (outfile, data_dir, trimmed_dir) = dests_from_argmatches(submatches);
			let mut wtr = Writer::from_path(outfile)?;
			// add trim length logic
		}
		_ => app.print_help()?,
	}

	Ok(())
}

fn samples_map(path: &Path) -> Result<BTreeMap<String, Flag>, Box<dyn error::Error>> {
	let paths = fs::read_dir(path)?;

	// this is just for per base sequence quality for now
	let mut samples = BTreeMap::<String, Flag>::default();

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
							let mut k = dir.clone();
							// We use zero-padded filenames as keys.
							// For example, Sample1 is Sample01.
							// This is for natural sort.
							let part = k.splitn(2, "Sample").collect::<Vec<&str>>()[1]
								.splitn(2, "_")
								.collect::<Vec<&str>>();
							let sample_num = part[0].parse::<u32>()?;
							if sample_num < 10 {
								k = format!("Sample0{}_{}", sample_num, part[1]);
							}
							samples.insert(k, res.flag);
							continue 'outer;
						}
					}
				}
				_ => (),
			}
		}
	}
	Ok(samples)
}

/// Get the outfile, input_dir, and trimmed_dir in a tuple from the subcommand ArgMatches.
fn dests_from_argmatches(matches: &ArgMatches) -> (PathBuf, &Path, &Path) {
	let outfile = match matches.value_of("output_file") {
		Some(v) => v,
		None => unreachable!("No output file specified"),
	};

	let data_dir = match matches.value_of("input_dir") {
		Some(v) => Path::new(v),
		None => unreachable!("No input directory specified"),
	};
	let outfile = data_dir.join(outfile);

	let trimmed_dir = match matches.value_of("trimmed_dir") {
		Some(v) => Path::new(v),
		None => unreachable!("No trimmed directory specified"),
	};
	(outfile, data_dir, trimmed_dir)
}

#[derive(Template)]
#[template(path = "report.html")]
struct ReportTemplate {
	sample_dirs: BTreeMap<String, Flag>,
	trimmed_dirs: BTreeMap<String, Flag>,
	dt: DateTime<Utc>,
}

impl ReportTemplate {
	fn new(sample_dirs: BTreeMap<String, Flag>, trimmed_dirs: BTreeMap<String, Flag>) -> Self {
		ReportTemplate {
			sample_dirs,
			trimmed_dirs,
			dt: Utc::now(),
		}
	}
}
