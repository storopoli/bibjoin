# Scopus/Web of Science RSL Combining Records

[![GitHub license](https://img.shields.io/github/license/storopoli/bibjoin)](https://github.com/storopoli/bibjoin/blob/main/LICENSE)
[![Build status](https://github.com/storopoli/bibjoin/workflows/ci/badge.svg)](https://github.comstoropoli/bibjoin/workflows/actions)
[![Crates.io](https://img.shields.io/crates/v/bibjoin.svg)](https://crates.io/crates/bibjoin)

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

[![asciicast](https://asciinema.org/a/432787.svg)](https://asciinema.org/a/432787)
