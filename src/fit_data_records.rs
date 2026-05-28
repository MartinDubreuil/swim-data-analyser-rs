use crate::error::{Error, Result};

#[derive(Debug, Default)]
pub struct FitDataRecords {
    records: Vec<FitDataRecord>,
}

impl FitDataRecords {
    pub fn validate(data_records: &[u8]) -> Result<&[u8]> {
        Ok(data_records)
    }

    pub fn parse(&mut self, data_records: &[u8]) -> Result<()> {
        // while(!end) {

        let record_header = data_records
            .first()
            .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;

        let header_type = (record_header >> 7) & 1;
        let data_type = (record_header >> 6) & 1;

        let mut fit_data_record = FitDataRecord::new(header_type, data_type)?;
        fit_data_record.parse(data_records)?;

        self.records.push(fit_data_record);

        // }

        Ok(())
    }
}

#[derive(Debug, Default)]
struct FitDataRecord {
    record_header_type: RecordHeaderType,
    record_content_type: RecordContentType,
}

impl FitDataRecord {
    pub fn new(header_type: u8, data_type: u8) -> Result<Self> {
        let record_header_type = match header_type {
            0 => RecordHeaderType::Normal(NormalHeader::default()),
            1 => RecordHeaderType::CompressedTimestamp(CompressedTimestampHeader::default()),
            other => {
                return Err(Error::InvalidValue {
                    received: other,
                    expected: 0,
                });
            }
        };

        let record_content_type = match data_type {
            0 => RecordContentType::Definition(DefinitionMessage::default()),
            1 => RecordContentType::Data(DataMessage::default()),
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
        match &mut self.record_header_type {
            RecordHeaderType::Normal(normal) => normal.parse(content)?,
            RecordHeaderType::CompressedTimestamp(compressed_timestamp) => {
                compressed_timestamp.parse(content)?
            }
        };

        match &mut self.record_content_type {
            RecordContentType::Definition(definition) => definition.parse(content)?,
            RecordContentType::Data(data) => data.parse(content)?,
        };

        Ok(())
    }
}

#[derive(Debug)]
enum RecordHeaderType {
    Normal(NormalHeader),
    CompressedTimestamp(CompressedTimestampHeader),
}

impl Default for RecordHeaderType {
    fn default() -> Self {
        Self::Normal(NormalHeader::default())
    }
}

#[derive(Debug, Default)]
struct NormalHeader {}
impl NormalHeader {
    pub fn parse(&mut self, _content: &[u8]) -> Result<()> {
        println!("NormalHeader::parse()"); // Todo: To implement
        Ok(())
    }
}

#[derive(Debug, Default)]
struct CompressedTimestampHeader {}
impl CompressedTimestampHeader {
    pub fn parse(&mut self, _content: &[u8]) -> Result<()> {
        println!("CompressedTimestampHeader::parse()"); // Todo: To implement
        Ok(())
    }
}

#[derive(Debug)]
enum RecordContentType {
    Definition(DefinitionMessage),
    Data(DataMessage),
}

impl Default for RecordContentType {
    fn default() -> Self {
        Self::Definition(DefinitionMessage::default())
    }
}

#[derive(Debug, Default)]
struct DefinitionMessage {}
impl DefinitionMessage {
    pub fn parse(&mut self, _content: &[u8]) -> Result<()> {
        println!("DefinitionMessage::parse()"); // Todo: To implement
        Ok(())
    }
}
#[derive(Debug, Default)]
struct DataMessage {}
impl DataMessage {
    pub fn parse(&mut self, _content: &[u8]) -> Result<()> {
        println!("DataMessage::parse()"); // Todo: To implement
        Ok(())
    }
}
