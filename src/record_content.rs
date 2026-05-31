use crate::error::Result;

use enum_dispatch::enum_dispatch; // Todo: Use enum_dispatch

pub enum RecordContentTypeValue {
    Data,
    Definition,
}

#[derive(Debug)]
#[enum_dispatch(validate)]
pub enum RecordContentType {
    Definition(DefinitionMessage),
    Data(DataMessage),
}

impl RecordContentType {
    pub fn validate<'a>(&self, data_records: &'a [u8]) -> Result<&'a [u8]> {
        match self {
            RecordContentType::Definition(_) => DefinitionMessage::validate(data_records),
            RecordContentType::Data(_) => DataMessage::validate(data_records),
        }
    }

    pub fn parse(&mut self, data_record: &[u8]) -> Result<()> {
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
pub struct DefinitionMessage {}
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
pub struct DataMessage {}
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
