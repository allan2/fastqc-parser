# fastqc-parser

Often, we want to gather results from multiple FastQC files. The number could be in the hundreds.

This program summarizes the results together.

[FastQC](https://github.com/s-andrews/FastQC) outputs zip files with the following contents:
 - fastqc_data.txt
 - summary.txt

FastQC takes your `fastqc.gz` files and outputs:
 - an HTML file
 - a zip file containing the HTML file along with other goodies

This program relies on the unzipped directories of those ZIP files, all placed in a root directory.
To turn those zip files into folders, `unzip` can be used:
```sh
unzip \*.zip
```

Roadmp
 - async

WORK IN PROGRESS
