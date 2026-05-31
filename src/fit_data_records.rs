use crate::error::{Error, Result};

#[derive(Debug, Default)]
pub struct FitDataRecords {
    records: Vec<FitDataRecord>,
}

impl FitDataRecords {
    pub fn validate(content: &[u8]) -> Result<&[u8]> {
        Ok(&content
            .get(..content.len() - 2)
            .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?)
    }

    pub fn parse(&mut self, mut data_records: &[u8]) -> Result<()> {
        while !data_records.is_empty() {
            // let record_header = data_records
            //     .first()
            //     .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;

            // let header_type = (record_header >> 7) & 1;
            // let data_type = (record_header >> 6) & 1;

            // let mut fit_data_record = FitDataRecord::new(header_type, data_type)?;

            let data_record = FitDataRecord::validate(data_records)?;
            fit_data_recor.parse(data_records)?;
            self.records.push(fit_data_record);

            data_records = data_records
                .get(data_record.len()..)
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
            0 => RecordContentType::Data(DataMessage::default()),
            1 => RecordContentType::Definition(DefinitionMessage::default()),
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

    pub fn validate(data_records: &[u8]) -> Result<&[u8]> {
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

        let record_header = match record_header_type {
            RecordHeaderTypeValue::Normal => NormalHeader::validate(data_records)?,
            RecordHeaderTypeValue::CompressedTimestamp => {
                CompressedTimestampHeader::validate(data_records)?
            }
        };

        let data_type = (record_header_val >> 6) & 1;

        if record_header_type == RecordHeaderTypeValue::CompressedTimestamp && data_type == 0 {
            return Err(Error::InvalidValue {
                received: 1,
                expected: 0,
            });
        }

        let record_content = match data_type {
            0 => DataMessage::validate(data_records),
            1 => DefinitionMessage::validate(data_records),
            other => {
                return Err(Error::InvalidValue {
                    received: other,
                    expected: 0,
                });
            }
        };

        Ok(&data_records
            .get(..record_header.len() + record_content.len())
            .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?)
    }

    fn parse(&mut self, data_record: &[u8]) -> Result<()> {
        match &mut self.record_header_type {
            RecordHeaderType::Normal(normal) => {
                let normal_header = NormalHeader::validate(data_record)?;
                normal.parse(normal_header)?
            }
            RecordHeaderType::CompressedTimestamp(compressed_timestamp) => {
                let compressed_timestamp_header = CompressedTimestampHeader::validate(data_record)?;
                compressed_timestamp.parse(compressed_timestamp_header)?
            }
        };

        match &mut self.record_content_type {
            RecordContentType::Definition(definition) => definition.parse(data_record)?,
            RecordContentType::Data(data) => data.parse(data_record)?,
        };

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
    pub fn validate(content: &[u8]) -> Result<&[u8]> {
        Ok(content
            .get(..1)
            .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?)
    }

    pub fn parse(&mut self, header: &[u8]) -> Result<()> {
        println!("NormalHeader::parse()"); // Todo: To implement

        self.local_message_type = header
            .get(0)
            .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?
            & 0b1111;

        Ok(())
    }
}

#[derive(Debug, Default)]
struct CompressedTimestampHeader {}

impl CompressedTimestampHeader {
    pub fn validate(content: &[u8]) -> Result<&[u8]> {
        Ok(&content
            .get(..1)
            .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?)
    }

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
