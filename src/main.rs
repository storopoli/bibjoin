use clap::Parser;
use polars::datatypes::DataType::Utf8;
use polars::prelude::*;
use std::fs::File;

struct Config<'a> {
    name: String,
    cols: &'a [&'a str],
    delimiter: u8,
}

impl Config<'_> {
    fn scopus() -> Self {
        Config {
            name: String::from("Scopus"),
            cols: &[
                "Year",
                "Authors",
                "Title",
                "Source title",
                "Volume",
                "Issue",
                "ISSN",
                "Abstract",
                "Author Keywords",
                "Index Keywords",
                "DOI",
            ],
            delimiter: b',',
        }
    }
    fn wos() -> Self {
        Config {
            name: String::from("Web of Science"),
            cols: &[
                "PY", "AU", "TI", "SO", "VL", "IS", "SN", "AB", "DE", "ID", "DI",
            ],
            delimiter: b'\t',
        }
    }
}

fn vec_of_str(v: &[&str]) -> Vec<String> {
    v.iter().map(|&x| x.into()).collect()
}

fn read_file(filepath: &str, config: &Config) -> Result<DataFrame> {
    // read from path
    let df = CsvReader::from_path(&filepath)?
        .with_delimiter(config.delimiter)
        .with_dtypes_slice(Some(vec![Utf8; 999].as_slice()))
        .with_columns(Some(vec_of_str(config.cols)))
        .has_header(true)
        .finish()?;
    let df_filtered = df.select(config.cols.to_vec())?;
    println!("{} file has: {} records", config.name, df.shape().0);
    Ok(df_filtered)
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
#[derive(Parser, Debug)]
#[clap(version = "0.3.0", author = "Jose Storopoli <thestoropoli@gmail.com>")]
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

    // Configs
    let config_scopus = Config::scopus();
    let config_wos = Config::wos();

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
    use std::path::Path;

    fn setup_scopus() -> DataFrame {
        let config_scopus = Config::scopus();
        let scopus_path = Path::new("examples/scopus.csv").to_str().unwrap();
        read_file(scopus_path, &config_scopus).unwrap()
    }

    fn setup_wos() -> DataFrame {
        let config_wos = Config::wos();
        let wos_path = Path::new("examples/wos.txt").to_str().unwrap();
        read_file(wos_path, &config_wos).unwrap()
    }

    #[test]
    fn vstack_shape() {
        // DataFrames
        let mut scopus = setup_scopus();
        let wos = setup_wos();

        vstack_dfs(&mut scopus, &wos);

        assert_eq!(scopus.shape(), (95, 11))
    }

    #[test]
    fn drop_duplicates_shape() {
        // DataFrames
        let mut scopus = setup_scopus();
        let wos = setup_wos();

        vstack_dfs(&mut scopus, &wos);

        let df = drop_duplicates(&scopus, &[String::from("DOI")]).unwrap();
        assert_eq!(df.shape(), (62, 11))
    }
}
