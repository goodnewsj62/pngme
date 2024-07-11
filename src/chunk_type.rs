use std::{ fmt::Display, str::FromStr,  str};


#[derive(PartialEq,  Eq, Debug)]
pub struct ChunkType([u8;4]);


impl ChunkType{
    pub fn bytes(&self) -> [u8;4]{
        self.0
    }


    fn is_valid(&self) -> bool{
        self.is_critical() && self.is_reserved_bit_valid()
    }
    fn is_critical(&self) -> bool{
        self._bin_chars_rev(0)[5].eq( &'0')
    }
    fn is_public(&self) ->bool{
        self._bin_chars_rev(1)[5].eq( &'0')
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self._bin_chars_rev(2)[5].eq( &'0')
    }
    fn is_safe_to_copy(&self) -> bool {
        self._bin_chars_rev(3)[5].eq( &'1')
    }


    
    fn _to_binary_string(&self, index: usize) ->String{
        if index > 3{
            panic!("index out of range")
        }
        format!("{:b}", self.0[index])
    }

    fn _bin_chars_rev(&self, index:usize) -> Vec<char>{
        self._to_binary_string(index).chars().rev().collect()
    }
}

impl Display for ChunkType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_ =  str::from_utf8(&self.0).expect("invalid ascii character was found present");
        write!(f, "{}", str_ )
    }
}

impl TryFrom<[u8; 4]> for ChunkType{
    type Error = String;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for val in value{
            match val {
                65..=90 | 97..=122 =>{},
                val => {return  Err(format!("An invalid ascii character with code {} found",  val));}
            }
        }

        Ok(Self(value))
    }
}


impl FromStr for ChunkType{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_bytes =s.as_bytes();
        if str_bytes.len() > 4 {
            return  Err("chunk type expects 4bytes of ascii characters".to_string());
        }
        
        let valid_bytes: [u8;4]=  str_bytes.try_into().unwrap();

        ChunkType::try_from(valid_bytes)
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