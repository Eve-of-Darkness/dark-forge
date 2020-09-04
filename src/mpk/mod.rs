use std::fs::File;
use std::io::{
    prelude::*,
    Error as IOError,
    SeekFrom,
};
use std::mem::transmute;
use flate2::bufread::ZlibDecoder;

#[derive(Debug)]
pub struct Mpak {
    pub file: File,
    pub dir_crc32: u32,
    pub dir_compressed_size: u32,
    pub archive_name_compressed_size: u32,
    pub file_count: u32,
}

#[derive(Debug)]
pub enum Error {
    IO(IOError)
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Self { Error::IO(error) }
}

impl Mpak {
    pub fn open(filename: &str) -> Result<Self, Error> {
        let mut file = File::open(filename)?;
        file.seek(SeekFrom::Start(5))?;
        let mut xor_byte: u8 = 0;

        Ok(Mpak {
            // Please note the order of these calls *is* important
            dir_crc32: Mpak::read_next_u32(&mut file, &mut xor_byte)?,
            dir_compressed_size: Mpak::read_next_u32(&mut file, &mut xor_byte)?,
            archive_name_compressed_size: Mpak::read_next_u32(&mut file, &mut xor_byte)?,
            file_count: Mpak::read_next_u32(&mut file, &mut xor_byte)?,
            file,
        })
    }

    pub fn dump_contents(&mut self) -> Result<(), Error> {
        let mut in_buffer: [u8; 1024] = [0; 1024];
        let mut out_buffer: [u8; 1024] = [0; 1024];
        let in_size = self.file.read(&mut in_buffer)?;
        let mut decoder = ZlibDecoder::new(&in_buffer[0..in_size]);
        let out_size = decoder.read(&mut out_buffer)?;
        let string = std::str::from_utf8(&out_buffer[0..out_size]).unwrap();
        println!("I read in {} bytes", in_size);
        println!("I read out {} bytes", out_size);
        println!("I read: {}", &string);
        println!("buffer_in {}", &decoder.total_in());
        decoder.reset(&in_buffer[(decoder.total_in() as usize)..]);
        let out_size = decoder.read(&mut out_buffer)?;
        println!("I read out {} bytes", out_size);
        println!("The in buffer was {}", decoder.total_in());

        let out_size = decoder.read(&mut out_buffer)?;
        println!("I read out {} bytes", out_size);
        println!("The in buffer was {}", decoder.total_in());

        let out_size = decoder.read(&mut out_buffer)?;
        println!("I read out {} bytes", out_size);
        println!("The in buffer was {}", decoder.total_in());
        //println!("I read: {}", &string);
        Ok(())
    }

    // Private functions

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
