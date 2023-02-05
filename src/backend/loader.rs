use std::{fs, io::Error};

pub fn open_file(fname: &str) -> Result<Vec<u8>, Error> {
    match fs::read(fname) {
        Ok(data) => Ok(data),
        Err(e) => Err(e),
    }
}

