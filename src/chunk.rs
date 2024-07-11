use crc::{Crc, CRC_32_ISO_HDLC};
use std::{fmt::Display, string::FromUtf8Error};
use crate::chunk_type::ChunkType;

#[derive(Debug)]
pub struct Chunk{
    length: [u8;4],
    chunk_type:  ChunkType,
    chunk_data:  Vec<u8>,
    crc:  [u8;4]
}


impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self{
        

        let crc_  =  Chunk::_generate_crc(&chunk_type.bytes(), &data);
        
        Chunk{
            length:  (data.len() as u32).to_be_bytes(),
            chunk_type,
            chunk_data:  data,
            crc: crc_.to_be_bytes()
        }
    }
    pub fn length(&self) -> u32{
        u32::from_be_bytes(self.length)
    }
    pub fn chunk_type(&self) -> &ChunkType{
        &self.chunk_type
    }
    fn data(&self) -> &[u8]{
        &self.chunk_data
    }
    fn crc(&self) -> u32{
        u32::from_be_bytes(self.crc)
    }
    fn data_as_string(&self) -> Result<String, FromUtf8Error>{
        String::from_utf8(self.chunk_data.clone())
    }
    fn as_bytes(&self) -> Vec<u8>{
        self.chunk_data.clone()
    }

    fn _generate_crc(chunk_type:&[u8], data:&[u8]) -> u32{
        pub const PNG_CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        
        let mut digest = PNG_CRC.digest();
        digest.update(chunk_type);
        digest.update(data);
        digest.finalize()
    }
}

impl Display for Chunk{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,  "{:?}",  self)
    }
}

impl TryFrom<&[u8]> for Chunk{
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // slice into two
        // first four will be tured into chuck type validated or error will be thrown
        // length of remaining data should not be greater than u32
        // turn data to vector
        //  use both chunk type and data an get the check some then return everything

        let length: [u8;4] =  value.iter().take(4).cloned().collect::<Vec<u8>>().try_into().map_err(|_| "invalid length expected 4bytes array")?;
        let numeric_len =  u32::from_be_bytes(length) as usize;
        let chunk_type:[u8;4]=  value.iter().skip(4).take(4).cloned().collect::<Vec<u8>>().try_into().map_err(|_| "invalid length chunk_type is expected be 4bytes")?;
        let chunk_type = ChunkType::try_from(chunk_type)?;
        
        let chunk_data =  value.iter().skip(8).take( numeric_len ).cloned().collect::<Vec<u8>>();
        let crc: [u8;4] = value.iter().skip(8 +  numeric_len).take(4).cloned().collect::<Vec<u8>>().try_into().map_err(|_| "invalid length for crc expected 4bytes array")?;

        if Self::_generate_crc(&chunk_type.bytes(), &chunk_data).ne(&u32::from_be_bytes(crc)){
            return  Err("corrupted data".to_string());
        }
    
        Ok(Chunk{
            length:  length,
            chunk_type: chunk_type,
            chunk_data:  chunk_data,
            crc:  crc
        })
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
        let data = "This is where your secret message will be!".as_bytes().to_vec();
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
