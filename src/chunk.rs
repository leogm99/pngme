use std::{
    fmt::Display,
    str::{from_utf8, Utf8Error},
};

use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::ChunkType;

pub struct Chunk {
    data_size: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

pub fn crc(bytes: &[u8]) -> u32 {
    let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let mut digest = crc.digest();
    digest.update(bytes);
    digest.finalize()
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_string = from_utf8(self.chunk_data.as_ref());
        if as_string.is_err() {
            return Err(std::fmt::Error);
        }
        write!(f, "{}", as_string.unwrap())
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = String;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err("Not enough data to build chunk".into());
        }
        let data_size = u32::from_be_bytes(data[0..4].try_into().unwrap()) as usize;
        let chunk_type_data: [u8; 4] = data[4..8].try_into().unwrap();
        let chunk_type = ChunkType::try_from(chunk_type_data);
        if let Err(msg) = chunk_type {
            return Err(msg);
        }
        let chunk_type = chunk_type.unwrap();
        let chunk_data: Vec<u8> = data[8..(8 + data_size)].into();
        let valid_crc = crc(chunk_type
            .bytes()
            .iter()
            .chain(chunk_data.iter())
            .copied()
            .collect::<Vec<u8>>()
            .as_slice());
        let crc = u32::from_be_bytes(data[(8 + data_size)..].try_into().unwrap());
        if crc != valid_crc {
            return Err(format!("Invalid crc, expected {valid_crc}, found {crc}"));
        }
        Ok(Chunk {
            data_size: data_size as u32,
            chunk_type,
            chunk_data,
            crc,
        })
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc = crc(chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect::<Vec<u8>>()
            .as_slice());
        Chunk {
            chunk_type,
            data_size: data.len() as u32,
            chunk_data: data,
            crc,
        }
    }

    pub const fn crc(&self) -> u32 {
        self.crc
    }

    pub const fn length(&self) -> u32 {
        self.data_size
    }

    pub const fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data_as_string(&self) -> Result<&str, Utf8Error> {
        from_utf8(&self.chunk_data.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }
    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
