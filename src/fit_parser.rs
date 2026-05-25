use std::{fs, path::Path};

use crate::{error::Result, fit_data_records_dyn::FitDataRecords, fit_header::FitHeader};

#[derive(Debug, Default)]
pub struct FitParser {
    header: FitHeader,
    data_records: FitDataRecords,
}

impl FitParser {
    pub fn parse(&mut self, swimming_path: &Path) -> Result<()> {
        let content = fs::read(swimming_path)?;

        self.header.parse(&content)?;
        self.data_records.parse(&content)?;

        Ok(())
    }
}
