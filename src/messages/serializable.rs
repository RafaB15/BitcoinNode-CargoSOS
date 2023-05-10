use super::error_message::ErrorMessage;
use std::io::Write;

use bitcoin_hashes::{
    sha256d,
    Hash,
};

use chrono::{
    DateTime,
    offset::Utc,
};

pub trait Serializable {
    
    fn serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorMessage>;
    
}

impl Serializable for i32 {
    fn serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorMessage> {
        match stream.write(&self.to_le_bytes()) {
            Ok(_) => Ok(()),
            _ => Err(ErrorMessage::ErrorInDeserialization),
        }
    }
}

impl Serializable for u32 {
    fn serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorMessage> {
        match stream.write(&self.to_le_bytes()) {
            Ok(_) => Ok(()),
            _ => Err(ErrorMessage::ErrorInDeserialization),
        }
    }
}

impl Serializable for u64 {
    fn serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorMessage> {
        match stream.write(&self.to_le_bytes()) {
            Ok(_) => Ok(()),
            _ => Err(ErrorMessage::ErrorInDeserialization),
        }
    }
}

impl Serializable for Vec<u8> {
    fn serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorMessage> {
        match stream.write(&self) {
            Ok(_) => Ok(()),
            _ => Err(ErrorMessage::ErrorInDeserialization),
        }
    }
}

impl Serializable for [u8] {
    fn serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorMessage> {
        match stream.write(&self) {
            Ok(_) => Ok(()),
            _ => Err(ErrorMessage::ErrorInDeserialization),
        }
    }
}

impl Serializable for bool {
    fn serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorMessage> {
        let boolean: [u8; 1] = match self {
            true => [0x01],
            false => [0x00],
        };

        match stream.write(&boolean) {
            Ok(_) => Ok(()),
            _ => Err(ErrorMessage::ErrorInDeserialization),
        }
    }
}

impl Serializable for String {
    fn serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorMessage> {
        match stream.write(&self.as_bytes()) {
            Ok(_) => Ok(()),
            _ => Err(ErrorMessage::ErrorInDeserialization),
        }
    }
}

impl Serializable for DateTime<Utc> {
    fn serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorMessage> {
        match stream.write(&self.timestamp().to_le_bytes())  {
            Ok(_) => Ok(()),
            _ => Err(ErrorMessage::ErrorInDeserialization),
        }
    }
}

impl Serializable for sha256d::Hash {
    fn serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorMessage> {
        let hash_bytes: &[u8] = self.as_ref();
        let hash_bytes: &[u8; 4] = match (&hash_bytes[0..4]).try_into() {
            Ok(hash_bytes) => hash_bytes,
            _ => return Err(ErrorMessage::ErrorInSerialization),
        };

        match stream.write(hash_bytes) {
            Ok(_) => Ok(()),
            _ => Err(ErrorMessage::ErrorInDeserialization),
        }
    }
}