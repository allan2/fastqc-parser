# FastQC Reports

An aggregator for FastQC sequence data reports.

This program summarizes many reports from [FastQC](https://github.com/s-andrews/FastQC) together.

## Features
 - separate tabs for untrimmed sequence data and trimmed sequence data
 - toggle to show failed tests only

 ## Currently Supported Metrics
 - per base sequence quality

No longer will you have open separate files and flipping through them all to check your sequence data quality.
View hundreds of plots in a grid in a single HTML file!

![before trimming](screenshots/before_trim.png?raw=true)
![after trimming](screenshots/after_trim.png?raw=true)

## Usage


```sh
fastqc_reports -i input_dir -t trim_dir -o aggregate_report.html
```

## TODO:
- image folder paths are hardcoded
- copy option for images for self-contained reports
- separate state for show fail only for trimmed and untrimmed
