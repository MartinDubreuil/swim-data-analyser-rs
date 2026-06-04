use crate::{
    error::{Error, Result},
    record_content::{DataMessage, DefinitionMessage, RecordContentType, RecordContentTypeValue},
    record_header::{
        CompressedTimestampHeader, NormalHeader, RecordHeaderType, RecordHeaderTypeValue,
    },
};

#[derive(Debug)]
pub struct FitDataRecord {
    record_header_type: RecordHeaderType,
    record_content_type: RecordContentType,
}

impl FitDataRecord {
    pub fn new(data_records: &[u8]) -> Result<Self> {
        let record_header_byte = data_records
            .first()
            .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;

        let header_type = (record_header_byte >> 7) & 1;

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

        let content_type = (record_header_byte >> 6) & 1;

        let record_content_type = match content_type {
            0 => RecordContentTypeValue::Data,
            1 => RecordContentTypeValue::Definition,
            other => {
                return Err(Error::InvalidValue {
                    received: other,
                    expected: 0,
                });
            }
        };

        Ok(Self::dispatch(record_header_type, record_content_type))
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

    pub fn validate<'a>(&self, data_records: &'a [u8]) -> Result<(&'a [u8], &'a [u8])> {
        let header_type = self.record_header_type.validate(data_records)?;
        let record_content = self.record_content_type.validate(data_records)?;

        Ok((header_type, record_content))
    }

    pub fn parse(&mut self, data_record: (&[u8], &[u8])) -> Result<()> {
        let record_header = data_record.0;
        self.record_header_type.parse(record_header)?;

        let record_content = data_record.1;
        self.record_content_type.parse(record_content)?;

        Ok(())
    }
}
