# fastqc-parser

An aggregator for FastQC sequence data reports.

This program summarizes the results together. View hundreds of plots in a grid!
No longer will you have open separate files and flipping through them all to check your sequence data quality.

[FastQC](https://github.com/s-andrews/FastQC) outputs zip files with the following contents:
 - fastqc_data.txt
 - summary.txt

FastQC takes your `fastqc.gz` files and outputs:
 - an HTML file
 - a zip file containing the HTML file along with other goodies

This program consumes the unzipped ZIP reports ouputted from FastQC.
To turn those zip files into folders, `unzip` can be used:
```sh
unzip *.zip
```
