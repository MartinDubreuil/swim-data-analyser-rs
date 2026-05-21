mod error;
mod fit_data_records;
mod fit_header;
mod fit_parser;

use std::path::Path;

use crate::{error::Result, fit_parser::FitParser};

fn main() -> Result<()> {
    // Todo: Pass argument path by CLI
    let swimming_path = Path::new("resources/swimming.fit");

    let mut fit_parser = FitParser::default();
    fit_parser.parse(swimming_path)?;

    println!("{fit_parser:#?}");

    Ok(())
}
