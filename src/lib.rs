// TODO: disable these
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
struct Error {
    msg: String,
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        write!(f, "FITS Error: {}", self.msg)
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        "fits error"
    }
}

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

#[derive(Clone, Copy, Debug)]
pub enum FileMode {
    ReadOnly,
    ReadWrite,
}

#[derive(Debug)]
pub struct FitsFile {
    filename: String,
    filesize: u64,
    filehandle: File,
    writemode: FileMode,
    datastart: Option<u64>,
}

impl FitsFile {
    pub fn open<F: AsRef<Path>>(filename: F, writemode: FileMode) -> Result<Self> {
        let f = File::open(&filename)?;

        let metadata = fs::metadata(&filename)?;

        let mut fits_file = FitsFile {
            filename: filename.as_ref().to_str().unwrap().to_string(),
            filesize: metadata.len(),
            filehandle: f,
            writemode: writemode,
            datastart: None,
        };

        /* TODO: Load the first record ffldrc(*fptr, 0, REPORT_EOF, status) */

        /* TODO: determine HDU structure (ffrhdu(*fptr, &hdutyp, status)) */
        fits_file.determine_hdu_structure()?;

        /* TODO: Handle extended filename syntax */

        Ok(fits_file)
    }

    /* Private methods */

    fn determine_hdu_structure(&mut self) -> Result<()> {
        Ok(())
    }
}
