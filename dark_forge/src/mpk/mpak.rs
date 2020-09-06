use flate2::bufread::ZlibDecoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, SeekFrom};
use std::mem::transmute;
use super::{Error, FileInfo};

pub type InfoCollection = HashMap<String, FileInfo>;

#[derive(Debug)]
pub struct Mpak {
    pub file: Option<File>,
    pub dir_crc32: u32,
    pub dir_compressed_size: u32,
    pub name_compressed_size: u32,
    pub file_count: u32,
    pub name: String,
    pub file_info: InfoCollection,
}

impl Mpak {
    pub fn open(filename: &str) -> Result<Self, Error> {
        let mut file = File::open(filename)?;
        file.seek(SeekFrom::Start(5))?;
        let mut xor_byte: u8 = 0;

        // Please note the order of these calls *is* important
        let dir_crc32 = Mpak::read_next_u32(&mut file, &mut xor_byte)?;
        let dir_compressed_size = Mpak::read_next_u32(&mut file, &mut xor_byte)?;
        let name_compressed_size = Mpak::read_next_u32(&mut file, &mut xor_byte)?;
        let file_count = Mpak::read_next_u32(&mut file, &mut xor_byte)?;
        let name = Mpak::read_name(&mut file, name_compressed_size)?;
        let file_info = Mpak::read_file_details(&mut file, dir_compressed_size)?;

        Ok(Mpak {
            dir_crc32,
            dir_compressed_size,
            name_compressed_size,
            file_count,
            name,
            file_info,
            file: Some(file),
        })
    }

    // Private functions

    fn read_file_details(file: &mut File, buffer_size: u32) -> Result<InfoCollection, Error> {
        let mut buffer = vec![0; buffer_size as usize];
        file.read_exact(&mut buffer)?;
        let mut decoder = ZlibDecoder::new(&buffer[..]);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer)?;
        let mut file_info = HashMap::new();

        let infos = buffer
            .chunks(FileInfo::raw_byte_size())
            .map(FileInfo::from_bytes);

        for info in infos {
            file_info.insert(info.name.clone(), info);
        }

        Ok(file_info)
    }

    fn read_name(file: &mut File, buffer_size: u32) -> Result<String, Error> {
        let mut buffer = vec![0; buffer_size as usize];
        file.read_exact(&mut buffer)?;
        let mut decoder = ZlibDecoder::new(&buffer[..]);
        let mut name = String::new();
        decoder.read_to_string(&mut name)?;
        Ok(name)
    }

    fn read_next_u32(file: &mut File, xor_byte: &mut u8) -> Result<u32, Error> {
        let mut bytes: [u8; 4] = [0; 4];
        file.read_exact(&mut bytes)?;
        for byte in 0..4 {
            bytes[byte] = bytes[byte] ^ *xor_byte;
            *xor_byte += 1;
        }

        Ok(unsafe { transmute(bytes) })
    }
}
