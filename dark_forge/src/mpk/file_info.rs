use std::mem::transmute;

const TOTAL_FILE_BYTE_SIZE: usize = 284;
const FILE_DETAIL_POS: usize = 256;

#[derive(Debug)]
pub struct FileInfo {
    pub name: String,
    pub timestamp: u32,
    pub unknown: u32,
    pub memory_offset: u32,
    pub decompressed_byte_size: u32,
    pub file_offset: u32,
    pub compressed_byte_size: u32,
    pub compressed_crc32: u32,
}

impl FileInfo {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.len() < TOTAL_FILE_BYTE_SIZE {
            panic!("FileInfo bytes must be at least {} bytes!", TOTAL_FILE_BYTE_SIZE);
        }

        let mut pos: usize = 0;
        while bytes[pos] != 0 { pos += 1 };
        let name = String::from_utf8(bytes[0..pos].to_vec()).unwrap();

        // Magical position where critical info is found
        pos = FILE_DETAIL_POS;

        Self {
            name,
            // Please note the order of these calls *is* important
            timestamp: Self::read_next_u32(&bytes, &mut pos),
            unknown: Self::read_next_u32(&bytes, &mut pos),
            memory_offset: Self::read_next_u32(&bytes, &mut pos),
            decompressed_byte_size: Self::read_next_u32(&bytes, &mut pos),
            file_offset: Self::read_next_u32(&bytes, &mut pos),
            compressed_byte_size: Self::read_next_u32(&bytes, &mut pos),
            compressed_crc32: Self::read_next_u32(&bytes, &mut pos),
        }
    }

    pub fn raw_byte_size() -> usize { TOTAL_FILE_BYTE_SIZE }

    // Private Functions

    fn read_next_u32(bytes: &[u8], pos: &mut usize) -> u32 {
        let mut buff: [u8; 4] = [0; 4];
        buff.copy_from_slice(&bytes[*pos..(*pos+4)]);
        *pos += 4;
        unsafe { transmute(buff) }
    }
}
