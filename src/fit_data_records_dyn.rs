use crate::error::{Error, Result};

#[derive(Debug, Default)]
pub struct FitDataRecords {
    records: Vec<FitDataRecord>,
}

impl FitDataRecords {
    pub fn parse(&mut self, content: &[u8]) -> Result<()> {
        let data_records = &content[14..]; // ToDo: Use variable instead of magic number

        // while(!end) {

        let record_header = data_records.first().unwrap();

        let header_type = (record_header >> 7) & 1;
        let data_type = (record_header >> 6) & 1;

        let mut fit_data_record = FitDataRecord::new(header_type, data_type)?;
        fit_data_record.parse(data_records)?;

        self.records.push(fit_data_record);

        // }

        Ok(())
    }
}

trait RecordType: std::fmt::Debug {
    fn parse(&mut self, content: &[u8]) -> Result<()>;
}

#[derive(Debug)]
struct FitDataRecord {
    record_header_type: Box<dyn RecordType>,  // RecordHeaderType
    record_content_type: Box<dyn RecordType>, // RecordContentType
}

impl FitDataRecord {
    pub fn new(header_type: u8, data_type: u8) -> Result<Self> {
        let record_header_type: Box<dyn RecordType> = match header_type {
            0 => Box::new(NormalHeader::default()),
            1 => Box::new(CompressedTimestampHeader::default()),
            other => {
                return Err(Error::InvalidValue {
                    received: other,
                    expected: 0,
                });
            }
        };

        let record_content_type: Box<dyn RecordType> = match data_type {
            0 => Box::new(DefinitionMessage::default()),
            1 => Box::new(DataMessage::default()),
            other => {
                return Err(Error::InvalidValue {
                    received: other,
                    expected: 0,
                });
            }
        };

        Ok(Self {
            record_header_type,
            record_content_type,
        })
    }

    fn parse(&mut self, content: &[u8]) -> Result<()> {
        self.record_header_type.parse(content)?;
        self.record_content_type.parse(content)?;

        Ok(())
    }
}

#[derive(Debug, Default)]
struct NormalHeader {}
impl RecordType for NormalHeader {
    fn parse(&mut self, content: &[u8]) -> Result<()> {
        println!("NormalHeader::parse()"); // Todo: To implement
        Ok(())
    }
}

#[derive(Debug, Default)]
struct CompressedTimestampHeader {}
impl RecordType for CompressedTimestampHeader {
    fn parse(&mut self, content: &[u8]) -> Result<()> {
        println!("CompressedTimestampHeader::parse()"); // Todo: To implement
        Ok(())
    }
}

#[derive(Debug, Default)]
struct DefinitionMessage {}
impl RecordType for DefinitionMessage {
    fn parse(&mut self, content: &[u8]) -> Result<()> {
        println!("DefinitionMessage::parse()"); // Todo: To implement
        Ok(())
    }
}

#[derive(Debug, Default)]
struct DataMessage {}
impl RecordType for DataMessage {
    fn parse(&mut self, content: &[u8]) -> Result<()> {
        println!("DataMessage::parse()"); // Todo: To implement
        Ok(())
    }
}
