use std::fs;
use std::io;
use std::path::Path;

mod error;
use error::Error;
use error::Result;

#[derive(Debug, Default)]
struct FitHeader {
    protocol_version: u8,
    profile_version: u16,
    data_records_size: u32,
    data_type: String,
}

impl FitHeader {
    pub fn parse(&mut self, content: &[u8]) -> Result<()> {
        let header = Self::validate(content)?;

        self.protocol_version = header[1];
        self.profile_version = u16::from_le_bytes(header[2..4].try_into()?);
        self.data_records_size = u32::from_le_bytes(header[4..8].try_into()?);
        self.data_type = str::from_utf8(&header[8..12])?.to_owned();

        Ok(())
    }

    fn validate(content: &[u8]) -> Result<&[u8]> {
        let header_size = *content
            .first()
            .ok_or(io::Error::from(io::ErrorKind::UnexpectedEof))?;

        if header_size < 14 {
            return Err(Error::InvaliHeaderSize {
                received: header_size,
            });
        }

        let header = content
            .get(..header_size as usize)
            .ok_or(io::Error::from(io::ErrorKind::UnexpectedEof))?;

        Self::checksum(header)?;

        if &header[8..12] != b".FIT" {
            return Err(Error::MagicNumber {
                received: str::from_utf8(&header[8..12])?.to_string(),
            });
        }

        Ok(header)
    }

    fn checksum(header: &[u8]) -> Result<()> {
        let mut crc: u16 = 0;
        for byte in &header[..12] {
            crc = Self::fit_crc_get16(crc, *byte);
        }

        let expected_crc = u16::from_le_bytes(header[12..14].try_into()?);
        if crc != expected_crc {
            return Err(Error::InvalidCrc {
                received: crc,
                expected: expected_crc,
            });
        }

        Ok(())
    }

    fn fit_crc_get16(mut crc: u16, byte: u8) -> u16 {
        const CRC_TABLE: [u16; 16] = [
            0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401, 0xA001, 0x6C00, 0x7800,
            0xB401, 0x5000, 0x9C01, 0x8801, 0x4400,
        ];

        // compute checksum of lower four bits of byte
        let mut tmp = CRC_TABLE[(crc & 0xF) as usize];
        crc = (crc >> 4) & 0x0FFF;
        crc = crc ^ tmp ^ CRC_TABLE[(byte & 0xF) as usize];

        // now compute checksum of upper four bits of byte
        tmp = CRC_TABLE[(crc & 0xF) as usize];
        crc = (crc >> 4) & 0x0FFF;
        crc = crc ^ tmp ^ CRC_TABLE[((byte >> 4) & 0xF) as usize];

        crc
    }
}

// data_records: Vec<FitRecordHeader, RecordContent>,

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

    fn parse(&self, content: &[u8]) -> Result<()> {
        match &self.record_header_type {
            RecordHeaderType::Normal(normal) => normal.parse(content),
            RecordHeaderType::CompressedTimestamp(compressed_timestamp) => {
                compressed_timestamp.parse(content)
            }
        };

        match &self.record_content_type {
            RecordContentType::Definition(definition) => definition.parse(content),
            RecordContentType::Data(data) => data.parse(content),
        };

        Ok(())
    }
}

#[derive(Debug, Default)]
struct NormalHeader {}
impl NormalHeader {
    pub fn parse(&self, content: &[u8]) {}
}

#[derive(Debug, Default)]
struct CompressedTimestampHeader {}
impl CompressedTimestampHeader {
    pub fn parse(&self, content: &[u8]) {}
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
struct DefinitionMessage {}
impl DefinitionMessage {
    pub fn parse(&self, content: &[u8]) {}
}
#[derive(Debug, Default)]
struct DataMessage {}
impl DataMessage {
    pub fn parse(&self, content: &[u8]) {}
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
struct FitDataRecords {
    records: Vec<FitDataRecord>,
}

impl FitDataRecords {
    pub fn parse(&mut self, content: &[u8]) -> Result<()> {
        let data_records = &content[14..]; // ToDo: Use variable instead of magic number

        // while(!end) {

        let record_header = data_records.first().unwrap();

        let header_type = (record_header >> 7) & 1;
        let data_type = (record_header >> 6) & 1;

        // ToDo: match sur header_type et data_type pour convertir en enum ?

        let fit_data_record = FitDataRecord::new(header_type, data_type)?;
        fit_data_record.parse(data_records)?;

        self.records.push(fit_data_record);

        // }

        Ok(())
    }
}

#[derive(Debug, Default)]
struct FitParser {
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

fn main() -> Result<()> {
    // Todo: Pass argument path by CLI
    let swimming_path = Path::new("resources/swimming.fit");

    let mut fit_parser = FitParser::default();

    fit_parser.parse(swimming_path)?;

    println!("{fit_parser:#?}");

    Ok(())
}
