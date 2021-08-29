use clap::{AppSettings, Clap};
use polars::prelude::*;
use std::fs::File;

struct Config<'a> {
    name: String,
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
    println!("{} file has: {} records", config.name, df.shape().0);
    Ok(df)
}

fn write_file(df: &DataFrame, filepath: &str) -> Result<()> {
    println!("Combined records file has: {} records", df.shape().0);
    let mut file = File::create(&filepath).expect("could not create file");
    // write DataFrame to file
    CsvWriter::new(&mut file)
        .has_headers(true)
        .with_delimiter(b',')
        .finish(df)
}

fn vstack_dfs(df: &mut DataFrame, other_df: &DataFrame) {
    df.vstack_mut(other_df)
        .expect("Could not combine datasets together");
}

fn drop_duplicates(df: &DataFrame, subset: &[String]) -> Result<DataFrame> {
    df.drop_duplicates(true, Some(subset))
}
/// Program to combine data from Scopus and Web of Science by DOI
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
struct BibJoin {
    /// Scopus CSV file path
    #[clap(short, long)]
    scopus: String,

    /// Web of Science file path
    #[clap(short, long)]
    wos: String,

    /// Output file path
    #[clap(short, long, default_value = "combined.csv")]
    output: String,
}
fn main() {
    // Parse Stuff
    let bibjoin = BibJoin::parse();

    // Config Structs
    let config_scopus = Config {
        name: String::from("Scopus"),
        cols: &["Authors", "Title", "Source title", "DOI"],
        delimiter: b',',
    };
    let config_wos = Config {
        name: String::from("Web of Science"),
        cols: &["AU", "TI", "SO", "DI"],
        delimiter: b'\t',
    };

    // Get Data as DataFrames
    let mut scopus = read_file(bibjoin.scopus.as_str(), &config_scopus)
        .expect("Could not read Scopus CSV file path");
    let wos = read_file(bibjoin.wos.as_str(), &config_wos)
        .expect("Could not read Web of Science file path");

    // Append DataFrames
    vstack_dfs(&mut scopus, &wos);

    // Drop Duplicates
    let df = drop_duplicates(&scopus, &[String::from("DOI")])
        .expect("Could not drop duplicates from combined records");

    write_file(&df, bibjoin.output.as_str()).expect("Could not save combined file");
    println!("Done!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::df;

    #[test]
    fn vstack_shape() {
        let mut scopus = df! [
            "Authors" => ["a", "b", "c"],
            "Title" => ["Title 1", "Title 2", "Title 3"],
            "Source title" => ["Journal 1", "Journal 2", "Journal 3"],
            "DOI" => ["1", "2", "3"]
        ]
        .unwrap();

        let wos = df! [
            "AU" => ["a", "c", "d"],
            "TI" => ["Title 1", "Title 3", "Title 4"],
            "SO" => ["Journal 1", "Journal 3", "Journal 4"],
            "DI" => ["1", "3", "4"]
        ]
        .unwrap();

        vstack_dfs(&mut scopus, &wos);

        assert_eq!(scopus.shape(), (6, 5))
    }

    #[test]
    fn drop_duplicates_shape() {
        let mut scopus = df! [
            "Authors" => ["a", "b", "c"],
            "Title" => ["Title 1", "Title 2", "Title 3"],
            "Source title" => ["Journal 1", "Journal 2", "Journal 3"],
            "DOI" => ["1", "2", "3"]
        ]
        .unwrap();

        let wos = df! [
            "AU" => ["a", "c", "d"],
            "TI" => ["Title 1", "Title 3", "Title 4"],
            "SO" => ["Journal 1", "Journal 3", "Journal 4"],
            "DI" => ["1", "3", "4"]
        ]
        .unwrap();

        vstack_dfs(&mut scopus, &wos);
        let df = drop_duplicates(&scopus, &[String::from("DOI")]).unwrap();
        assert_eq!(df.shape(), (4, 5))
    }
}
