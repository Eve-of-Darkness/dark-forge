use flate2::bufread::ZlibDecoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, SeekFrom};
use std::mem::transmute;
use super::{Error, FileInfo};

pub type InfoCollection = HashMap<String, FileInfo>;

/// Mpak Archive
///
/// The archive info as well as the file handle for a
/// Mpak archive.  Some of the raw parts of the header
/// info are kept as separate fields instead of re-reading
/// or calculating them all the time.
///
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
    /// Open Archive
    ///
    /// Given a valid path to a mpak archive; this will return a new
    /// `Mpak` struct loaded with the file header info and an open
    /// file handle to the archive.
    ///
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

    /// File Contents
    ///
    /// Given a filename string reference; this will return a potential
    /// buffer of the decompressed contents in the file if it is found.
    ///
    pub fn file_contents(&mut self, filename: &str) -> Option<Vec<u8>> {
        if self.file.is_none() { return None; }

        match self.file_info.get(filename) {
            None => None,
            Some(info) => {
                let mut out_buffer = Vec::with_capacity(info.decompressed_byte_size as usize);
                let mut in_buffer = vec![0; info.compressed_byte_size as usize];
                let mut file = self.file.as_ref().take().unwrap();
                file.seek(self.to_file_info_offset(info)).unwrap();
                file.read_exact(&mut in_buffer).unwrap();
                let mut decoder = ZlibDecoder::new(&in_buffer[..]);
                decoder.read_to_end(&mut out_buffer).unwrap();
                Some(out_buffer)
            }
        }
    }

    /// File Names
    ///
    /// Provides a collection of string references to the names of the
    /// files compressed in this mpak archive.  If you want to keep a
    /// hold of these for future use you may want to clone them.
    ///
    pub fn file_names(&self) -> Vec<&String> {
        self.file_info.keys().collect()
    }

    // Private functions

    fn to_file_info_offset(&self, info: &FileInfo) -> SeekFrom {
        SeekFrom::Start((21 + self.name_compressed_size + self.dir_compressed_size + info.file_offset) as u64)
    }

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
