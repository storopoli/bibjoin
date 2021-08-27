# Scopus/Web of Science RSL Combining Records

Combine CSV/TSV files from Scopus and Web of Science by DOI.

## Install

```bash
cargo install bibjoin
```

## Usage

```bash
❯ bibjoin -h
bibjoin

Program to combine data from Scopus and Web of Science by DOI

USAGE:
    bibjoin [OPTIONS] --scopus <SCOPUS> --wos <WOS>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -o, --output <OUTPUT>    Output file path [default: combined.csv]
    -s, --scopus <SCOPUS>    Scopus CSV file path
    -w, --wos <WOS>          Web of Science file path
```

[![asciicast](https://asciinema.org/a/2AbTvSaBiGeQU5UdcM95ZLERD.svg)](https://asciinema.org/a/2AbTvSaBiGeQU5UdcM95ZLERD)
