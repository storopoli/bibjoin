# Scopus/Web of Science RSL Combining Records

A simple program to run TSV files from Web of Science and CSV Files from Scopus into a single one by DOI.

```bash
❯ ./target/release/RSL -h
rsl

Program to Combine data from Scopus and Web of Science by DOI

USAGE:
    RSL [OPTIONS]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -o, --output <OUTPUT>    Output file path [default: combined.csv]
    -s, --scopus <SCOPUS>    Scopus CSV file path [default: scopus.csv]
    -w, --wos <WOS>          Web of Science file path [default: wos.txt]
```
