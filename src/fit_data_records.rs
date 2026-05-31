use crate::{error::Result, fit_data_record::FitDataRecord};

#[derive(Debug, Default)]
pub struct FitDataRecords {
    records: Vec<FitDataRecord>,
}

impl FitDataRecords {
    pub fn validate(content: &[u8]) -> Result<&[u8]> {
        Ok(content
            .get(..content.len() - 2)
            .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?)
    }

    pub fn parse(&mut self, mut data_records: &[u8]) -> Result<()> {
        // while !data_records.is_empty() {
        // Todo: Fix this
        while data_records.len() > 110 {
            let mut fit_data_record = FitDataRecord::new(data_records)?;
            let data_record = fit_data_record.validate(data_records)?;
            fit_data_record.parse(data_record)?;
            self.records.push(fit_data_record);

            data_records = data_records
                .get(data_record.0.len() + data_record.1.len()..)
                .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;
        }

        Ok(())
    }
}
