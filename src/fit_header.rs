use std::io;

use crate::error::{Error, Result};

#[derive(Debug, Default)]
pub struct FitHeader {
    protocol_version: u8,
    profile_version: u16,
    data_records_size: u32,
    data_type: String,
}

impl FitHeader {
    pub fn parse(&mut self, header: &[u8]) -> Result<()> {
        if header.len() < 14 {
            return Err(Error::InvalidHeaderSize {
                received: header.len() as u8,
            });
        }

        self.protocol_version = header[1];
        self.profile_version = u16::from_le_bytes(header[2..4].try_into()?);
        self.data_records_size = u32::from_le_bytes(header[4..8].try_into()?);
        self.data_type = str::from_utf8(&header[8..12])?.to_owned();

        Ok(())
    }

    pub fn validate(content: &[u8]) -> Result<&[u8]> {
        let header_size = *content
            .first()
            .ok_or(io::Error::from(io::ErrorKind::UnexpectedEof))?;

        if header_size < 14 {
            return Err(Error::InvalidHeaderSize {
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
        if header.len() < 14 {
            return Err(Error::InvalidHeaderSize {
                received: header.len() as u8,
            });
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: [u8; 14] = [14, 16, 222, 7, 99, 183, 0, 0, 46, 70, 73, 84, 161, 223];

    #[test]
    fn test_fit_crc_get16() {
        assert_eq!(FitHeader::fit_crc_get16(0, 14), 50305);
    }

    #[test]
    fn test_checksum() {
        assert!(FitHeader::checksum(&HEADER).is_ok());
    }

    #[test]
    fn test_validate() -> Result<()> {
        assert_eq!(FitHeader::validate(&HEADER)?, HEADER);
        Ok(())
    }

    // #[test]
    // fn test_parse() -> Result<()> {
    //     let fit_header = FitHeader::default();
    //     fit_header.parse(&HEADER)?;
    //     Ok(())
    // }
}
