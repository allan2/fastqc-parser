# FastQC Reports

An aggregator for FastQC sequence data reports.

This program summarizes many reports from [FastQC](https://github.com/s-andrews/FastQC) together.

## Features

- separate tabs for untrimmed sequence data and trimmed sequence data
- toggle to show failed tests only
- outputs a report showing trimmed amount per sequence

## Currently Supported Metrics

- per base sequence quality

No longer will you have open separate files and flipping through them all to check your sequence data quality.
View hundreds of plots in a grid in a single HTML file!

![before trimming](screenshots/before_trim.png?raw=true)
![after trimming](screenshots/after_trim.png?raw=true)

## Usage

To generate the HTML aggregate report:

```sh
fastqc_reports aggregate-reports -i input_dir -t trim_dir -o aggregate_report.html
```

To generate a CSV with the trim length differences:

```sh
fastqc_reports trim-length -i input_dir -t trim_dir -o trim_change.csv
```

## TODO:

- change output destination of aggregate_report.html from input_dir to root
- image folder paths are hardcoded
- copy option for images for self-contained reports
- separate state for show fail only for trimmed and untrimmed
- add the trimmed amount to the aggregate report
