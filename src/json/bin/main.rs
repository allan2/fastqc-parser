use clap::Parser;
use serde::Serialize;
use std::{
	error::{self},
	fmt::{self, Debug},
	fs::{self, File},
	io::{self, BufRead},
	str::FromStr,
};
use thiserror::Error;

// The naming convention for Illumina FASTQ folders can be found at https://support.illumina.com/help/BaseSpace_OLH_009008/Content/Source/Informatics/BS/NamingConvention_FASTQ-files-swBS.htm
#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
	#[clap(short, long, value_parser)]
	input_dir: String, // directory of uncompressed `_fastqc` folders with standard Illumina naming
	#[clap(short, long, value_parser)]
	output_file: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SampleMetadata {
	pub sample_name: u16,
	pub total_sequences: u32,
	#[serde(skip_serializing)]
	pub sequences_flagged_as_poor_quality: u32,
	pub sequence_start: u32,
	pub sequence_end: u32,
	pub percent_gc: u8,
}

impl SampleMetadata {
	pub fn new(
		sample_name: u16,
		total_sequences: u32,
		sequences_flagged_as_poor_quality: u32,
		sequence_start: u32,
		sequence_end: u32,
		percent_gc: u8,
	) -> Self {
		Self {
			sample_name,
			total_sequences,
			sequences_flagged_as_poor_quality,
			sequence_start,
			sequence_end,
			percent_gc,
		}
	}
}

/// Creates a JSON of read metadata
fn main() -> Result<(), Box<dyn error::Error>> {
	let args = Args::parse();
	println!("{}", args.input_dir);

	let mut reads = Vec::new();

	// The input directory contains a `_fastqc` directory for each read.
	for sample_dir in fs::read_dir(args.input_dir)? {
		let sample_entry = sample_dir?;
		let f = sample_entry.file_name();
		let read_num = f
			.to_str()
			.unwrap()
			.split("_001_fastq")
			.next()
			.unwrap()
			.chars()
			.last()
			.unwrap();

		// Read 1 and Read 2 will have the same metadata. Parse it from Read 1.
		if read_num != '1' {
			continue;
		}
		let path = sample_entry.path();

		for res in fs::read_dir(path)? {
			let entry = res?;
			if (entry.file_name().to_str().unwrap()) == "fastqc_data.txt" {
				let f = File::open(entry.path())?;
				let data = parse_fastqc_data_txt_file(f)?;
				reads.push(data);
			}
		}
	}

	reads.sort_unstable_by(|a, b| a.sample_name.cmp(&b.sample_name));
	fs::write(&args.output_file, serde_json::to_string(&reads)?)?;

	Ok(())
}

fn parse_fastqc_data_txt_file(file: File) -> Result<SampleMetadata, MyError> {
	let mut sample_num = 0;
	let mut total_seq = 0;
	let mut poor_q = 0;
	let mut seq_start = 0;
	let mut seq_end = 0;
	let mut percent_gc = 0;

	fn get_s(line: String) -> String {
		line.split('\t').nth(1).unwrap().to_owned()
	}

	fn num_from_line<F>(line: String) -> F
	where
		F: FromStr,
		<F as FromStr>::Err: fmt::Debug,
	{
		let s = get_s(line);
		s.parse::<F>().unwrap()
	}

	for (idx, line) in io::BufReader::new(file).lines().enumerate() {
		let s = line.map_err(|_| MyError::Io)?;

		// Validate line.
		let line_starts_with = |s: &str| -> Result<(), MyError> {
			let mut s = s.to_owned();
			s.push('\t');
			if !s.starts_with(&s) {
				return Err(MyError::InvalidBasicStatistics);
			}
			Ok(())
		};

		// matching line numbers
		match idx + 1 {
			4 => {
				line_starts_with("Filename")?;
				let s = get_s(s);
				sample_num = sample_name(&s)?.parse().unwrap();
			}
			7 => {
				line_starts_with("Total Sequences")?;
				total_seq = num_from_line(s);
			}
			8 => {
				line_starts_with("Sequences flagged as poor quality")?;
				poor_q = num_from_line(s);
			}
			9 => {
				line_starts_with("Sequence length")?;
				let s = get_s(s);
				let seq_len = s.split("-").collect::<Vec<_>>();
				seq_start = seq_len[0].parse().unwrap();
				seq_end = seq_len[1].parse().unwrap();
			}
			10 => {
				line_starts_with("%GC")?;
				percent_gc = num_from_line(s);
			}
			_ => {}
		}
	}
	let data = SampleMetadata::new(
		sample_num, total_seq, poor_q, seq_start, seq_end, percent_gc,
	);
	Ok(data)
}

// Gets the sample number from an Illumina FastQC filename.
fn sample_name(filename: &str) -> Result<String, MyError> {
	if let Some((s, _)) = filename.split_once("_") {
		if let Some((_, suffix)) = s.split_once("Sample") {
			if !suffix.is_empty() {
				return Ok(suffix.to_string());
			}
		}
	}
	Err(MyError::InvalidFileName)
}

#[derive(Error, Debug, PartialEq)]
pub enum MyError {
	#[error("error reading line")]
	Io,
	#[error("could not get basic statistic")]
	InvalidBasicStatistics,
	#[error("invalid filename")]
	InvalidFileName,
}

#[cfg(test)]
mod tests {
	use super::{sample_name, MyError};

	#[test]
	fn sample_name_is_name() {
		assert_eq!(
			sample_name("SampleName_S1_L001_R1_001.fastq.gz"),
			Ok("Name".to_string())
		);
	}
	#[test]
	fn sample_name_is_one() {
		assert_eq!(
			sample_name("Sample1_S1_L001_R1_001.fastq.gz"),
			Ok("1".to_string())
		);
	}

	#[test]
	fn sample_name_is_one_padded() {
		assert_eq!(
			sample_name("Sample01_S1_L001_R1_001.fastq.gz"),
			Ok("01".to_string())
		);
	}

	#[test]
	fn sample_name_is_missing() {
		assert_eq!(
			sample_name("Sample_S1_L001_R1_001.fastq.gz"),
			Err(MyError::InvalidFileName)
		);
	}

	#[test]
	fn sample_name_not_standard() {
		assert_eq!(sample_name("Sample123"), Err(MyError::InvalidFileName));
	}
}
