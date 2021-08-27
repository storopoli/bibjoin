#![allow(non_snake_case)]
use clap::Clap;
use polars::prelude::*;
use std::fs::File;

struct Config<'a> {
    cols: &'a [&'a str],
    delimiter: u8,
}

fn vec_of_str(v: &[&str]) -> Vec<String> {
    v.iter().map(|&x| x.into()).collect()
}

fn read_file(filepath: &str, config: &Config) -> Result<DataFrame> {
    // read from path
    let df = CsvReader::from_path(&filepath)?
        .with_delimiter(config.delimiter)
        .with_columns(Some(vec_of_str(config.cols)))
        .infer_schema(None)
        .has_header(true)
        .finish()?;
    Ok(df)
}

fn write_file(df: &DataFrame, filepath: &str) -> Result<()> {
    let mut file = File::create(&filepath).expect("could not create file");
    // write DataFrame to file
    CsvWriter::new(&mut file)
        .has_headers(true)
        .with_delimiter(b',')
        .finish(&df)

}

/// Program to Combine data from Scopus and Web of Science by DOI
#[derive(Clap, Debug)]
#[clap(name = "rsl")]
struct RSL {
    /// Scopus CSV file path
    #[clap(short, long, default_value = "scopus.csv")]
    scopus: String,

    /// Web of Science file path
    #[clap(short, long, default_value = "wos.txt")]
    wos: String,

    /// Output file path
    #[clap(short, long, default_value = "combined.csv")]
    output: String,
}
fn main() {
    // Parse Stuff
    let rsl = RSL::parse();

    // Config Structs
    let config_scopus = Config {
        cols: &["Authors", "Title", "Source title", "DOI"],
        delimiter: b',',
    };
    let config_wos = Config{
        cols: &["AU", "TI", "SO", "DI"],
        delimiter: b'\t',
    };

    // Get Data as DataFrames
    let mut scopus = read_file(rsl.scopus.as_str(), &config_scopus).expect("Could not read Scopus CSV file path");
    let wos = read_file(rsl.wos.as_str(), &config_wos).expect("Could not read Web of Science file path");

    scopus.set_column_names(&["AU", "TI", "SO", "DI"]).expect("Cannot rename Scopus Columns");
    scopus.outer_join(&wos, "DI", "DI").expect("Could not join datasets");
    
    write_file(&scopus, rsl.output.as_str()).expect("Could not save combined file");
    println!("Done!");
}
