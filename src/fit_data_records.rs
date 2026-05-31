use crate::error::{Error, Result};

use enum_dispatch::enum_dispatch; // Todo: Use enum_dispatch

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
        while !data_records.is_empty() {
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

#[derive(Debug, Default)]
struct FitDataRecord {
    record_header_type: RecordHeaderType,
    record_content_type: RecordContentType,
}

impl FitDataRecord {
    pub fn new(data_records: &[u8]) -> Result<Self> {
        let record_header_val = data_records
            .first()
            .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;

        let header_type = (record_header_val >> 7) & 1;

        let record_header_type = match header_type {
            0 => RecordHeaderTypeValue::Normal,
            1 => RecordHeaderTypeValue::CompressedTimestamp,
            other => {
                return Err(Error::InvalidValue {
                    received: other,
                    expected: 0,
                });
            }
        };

        let content_type = (record_header_val >> 6) & 1;

        let message_type = match content_type {
            0 => RecordContentTypeValue::Data,
            1 => RecordContentTypeValue::Definition,
            other => {
                return Err(Error::InvalidValue {
                    received: other,
                    expected: 0,
                });
            }
        };

        Ok(Self::dispatch(record_header_type, message_type))
    }

    fn dispatch(header_type: RecordHeaderTypeValue, content_type: RecordContentTypeValue) -> Self {
        let record_header_type = match header_type {
            RecordHeaderTypeValue::Normal => RecordHeaderType::Normal(NormalHeader::default()),
            RecordHeaderTypeValue::CompressedTimestamp => {
                RecordHeaderType::CompressedTimestamp(CompressedTimestampHeader::default())
            }
        };

        let record_content_type = match content_type {
            RecordContentTypeValue::Data => RecordContentType::Data(DataMessage::default()),
            RecordContentTypeValue::Definition => {
                RecordContentType::Definition(DefinitionMessage::default())
            }
        };

        Self {
            record_header_type,
            record_content_type,
        }
    }

    fn validate<'a>(&self, data_records: &'a [u8]) -> Result<(&'a [u8], &'a [u8])> {
        let header_type = self.record_header_type.validate(data_records)?;
        let record_content = self.record_content_type.validate(data_records)?;

        Ok((header_type, record_content))
        // Ok(data_records
        //     .get(..header_type.len() + record_content.len())
        //     .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?)
    }

    pub fn parse(&mut self, data_record: (&[u8], &[u8])) -> Result<()> {
        let record_header = data_record.0;
        self.record_header_type.parse(record_header)?;

        let record_content = data_record.1;
        self.record_content_type.parse(record_content)?;

        Ok(())
    }
}

enum RecordHeaderTypeValue {
    Normal,
    CompressedTimestamp,
}

#[derive(Debug)]
enum RecordHeaderType {
    Normal(NormalHeader),
    CompressedTimestamp(CompressedTimestampHeader),
}

impl RecordHeaderType {
    fn validate<'a>(&self, data_records: &'a [u8]) -> Result<&'a [u8]> {
        match self {
            RecordHeaderType::Normal(_) => NormalHeader::validate(data_records),
            RecordHeaderType::CompressedTimestamp(_) => {
                CompressedTimestampHeader::validate(data_records)
            }
        }
    }

    fn parse(&mut self, record_header: &[u8]) -> Result<()> {
        match self {
            RecordHeaderType::Normal(normal_header) => normal_header.parse(record_header),
            RecordHeaderType::CompressedTimestamp(compressed_timestamp) => {
                compressed_timestamp.parse(record_header)
            }
        }
    }
}

impl Default for RecordHeaderType {
    fn default() -> Self {
        Self::Normal(NormalHeader::default())
    }
}

#[derive(Debug, Default)]
struct NormalHeader {
    local_message_type: u8,
}

impl NormalHeader {
    pub fn validate(data_records: &[u8]) -> Result<&[u8]> {
        Ok(data_records
            .get(..1)
            .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?)
    }

    pub fn parse(&mut self, record_header: &[u8]) -> Result<()> {
        // println!("NormalHeader::parse()"); // Todo: To implement

        self.local_message_type = record_header
            .first()
            .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?
            & 0b1111;

        Ok(())
    }
}

#[derive(Debug, Default)]
struct CompressedTimestampHeader {}

impl CompressedTimestampHeader {
    pub fn validate(data_records: &[u8]) -> Result<&[u8]> {
        Ok(data_records
            .get(..1)
            .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?)
    }

    pub fn parse(&mut self, _record_header: &[u8]) -> Result<()> {
        // println!("CompressedTimestampHeader::parse()"); // Todo: To implement
        Ok(())
    }
}

enum RecordContentTypeValue {
    Data,
    Definition,
}

#[derive(Debug)]
#[enum_dispatch(validate)]
enum RecordContentType {
    Definition(DefinitionMessage),
    Data(DataMessage),
}

impl RecordContentType {
    fn validate<'a>(&self, data_records: &'a [u8]) -> Result<&'a [u8]> {
        match self {
            RecordContentType::Definition(_) => DefinitionMessage::validate(data_records),
            RecordContentType::Data(_) => DataMessage::validate(data_records),
        }
    }

    fn parse(&mut self, data_record: &[u8]) -> Result<()> {
        match self {
            RecordContentType::Definition(definition_message) => {
                definition_message.parse(data_record)
            }
            RecordContentType::Data(data_message) => data_message.parse(data_record),
        }
    }
}

impl Default for RecordContentType {
    fn default() -> Self {
        Self::Definition(DefinitionMessage::default())
    }
}

#[derive(Debug, Default)]
struct DefinitionMessage {}
impl DefinitionMessage {
    pub fn validate(data_records: &[u8]) -> Result<&[u8]> {
        Ok(data_records
            .get(..100)
            .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?)
    }

    pub fn parse(&mut self, _data_record: &[u8]) -> Result<()> {
        // println!("DefinitionMessage::parse()"); // Todo: To implement
        Ok(())
    }
}
#[derive(Debug, Default)]
struct DataMessage {}
impl DataMessage {
    pub fn validate(data_records: &[u8]) -> Result<&[u8]> {
        Ok(data_records
            .get(..100)
            .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?)
    }

    pub fn parse(&mut self, _data_record: &[u8]) -> Result<()> {
        // println!("DataMessage::parse()"); // Todo: To implement
        Ok(())
    }
}
