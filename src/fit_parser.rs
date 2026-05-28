use std::{fs, path::Path};

use crate::{error::Result, fit_data_records::FitDataRecords, fit_header::FitHeader};

#[derive(Debug, Default)]
pub struct FitParser {
    header: FitHeader,
    data_records: FitDataRecords,
}

impl FitParser {
    pub fn parse(&mut self, swimming_path: &Path) -> Result<()> {
        let content = fs::read(swimming_path)?;
        let mut cursor = 0;

        let header = FitHeader::validate(&content)?;
        self.header.parse(header)?;
        cursor += header.len();

        let data_records = FitDataRecords::validate(
            content
                .get(cursor..)
                .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?,
        )?;
        self.data_records.parse(data_records)?;

        Ok(())
    }
}
