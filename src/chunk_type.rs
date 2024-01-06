use std::str;
use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Debug)]
pub struct ChunkType {
    raw: [u8; 4],
}

impl ChunkType {
    pub const fn bytes(&self) -> [u8; 4] {
        self.raw
    }

    pub const fn is_critical(&self) -> bool {
        self.raw[0].is_ascii_uppercase()
    }

    pub const fn is_public(&self) -> bool {
        self.raw[1].is_ascii_uppercase()
    }

    pub const fn is_valid(&self) -> bool {
        self.raw[2].is_ascii_uppercase()
    }

    pub const fn is_safe_to_copy(&self) -> bool {
        self.raw[3].is_ascii_lowercase()
    }

    pub const fn is_reserved_bit_valid(&self) -> bool {
        self.is_valid()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = String;

    fn try_from(raw_chunk_type: [u8; 4]) -> Result<Self, Self::Error> {
        for index in 0..raw_chunk_type.len() {
            let byte = raw_chunk_type[index];

            if !byte.is_ascii() || !byte.is_ascii_alphabetic() {
                return Err(format!("Invalid byte {byte} at position {index}"));
            }
        }
        Ok(Self {
            raw: raw_chunk_type,
        })
    }
}

impl FromStr for ChunkType {
    type Err = String;

    fn from_str(str_chunk: &str) -> Result<Self, Self::Err> {
        let str_chunk_bytes = str_chunk.as_bytes();
        let chunk_size = str_chunk_bytes.len();
        if str_chunk_bytes.len() != 4 {
            return Err(format!("Invalid chunk of size {chunk_size}"));
        }
        let raw_chunk: [u8; 4] = str_chunk_bytes[0..4].try_into().unwrap();
        Self::try_from(raw_chunk)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x: String = self.raw.map(|b| b as char).iter().collect();
        write!(f, "{}", x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
