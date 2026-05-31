use crate::error::Result;

pub enum RecordHeaderTypeValue {
    Normal,
    CompressedTimestamp,
}

#[derive(Debug)]
pub enum RecordHeaderType {
    Normal(NormalHeader),
    CompressedTimestamp(CompressedTimestampHeader),
}

impl RecordHeaderType {
    pub fn validate<'a>(&self, data_records: &'a [u8]) -> Result<&'a [u8]> {
        match self {
            RecordHeaderType::Normal(_) => NormalHeader::validate(data_records),
            RecordHeaderType::CompressedTimestamp(_) => {
                CompressedTimestampHeader::validate(data_records)
            }
        }
    }

    pub fn parse(&mut self, record_header: &[u8]) -> Result<()> {
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
pub struct NormalHeader {
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
pub struct CompressedTimestampHeader {}

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
