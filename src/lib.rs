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

#[derive(PartialEq, Eq, Debug)]
pub struct Card {
    name: String,
    str_value: String,
    comment: Option<String>,
}

impl ::std::str::FromStr for Card {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let name: String = s.chars().take(8).filter(|c| !c.is_whitespace()).collect();
        let value_str: String = s.chars().skip_while(|c| *c != '=')
            .skip(1)
            .filter(|c| !c.is_whitespace()).collect();

        let value: String = value_str.chars().take_while(|c| *c != '/').collect();
        let comment: String = value_str.chars().skip_while(|c| *c != '/').skip(1).collect();

        if !comment.is_empty() {
            Ok(Card {
                name: name,
                str_value: value,
                comment: Some(comment),
            })
        } else {
            Ok(Card {
                name: name,
                str_value: value,
                comment: None,
            })
        }
    }
}

impl FitsFile {
    pub fn open<F: AsRef<Path>>(filename: F) -> Result<Self> {
        FitsFile::open_with_mode(filename, FileMode::ReadOnly)
    }

    pub fn edit<F: AsRef<Path>>(filename: F) -> Result<Self> {
        FitsFile::open_with_mode(filename, FileMode::ReadWrite)
    }

    pub fn create<F: AsRef<Path>>(filename: F) -> Result<Self> {
        unimplemented!()
    }

    /* Private methods */

    fn open_with_mode<F: AsRef<Path>>(filename: F, writemode: FileMode) -> Result<Self> {
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

    /// Determine the HDU structure of the current HDU
    ///
    /// read the required keywords of the current HDU, and initialize the corresponding structure
    /// elements that describe the format of the HDU
    fn determine_hdu_structure(&mut self) -> Result<()> {
        /* TODO: Get the 80-byte card (ffgrec) */
        let card = self.read_record(0)?;
        /* TODO: Get the first 8 characters as the card name */
        /* TODO: Parse the value and comment (ffpsvc) */
        let card: Card = card.parse()?;
        /* TODO: Handle options:
         * - if the name is SIMPLE, it's the primary array, and call ffpinit
         * - if the name is XTENSION, it's an XTENSION header
         *     - get the value string (ffc2s)
         *     - if the value is TABLE, then ascii table
         *     - if the value is BINTABLE or A3DTABLE or 3DTABLE then binary table
         *     - else IMAGE extension
         * - otherwise it is not the start of a new extension and raise error
         * TODO: compare the starting position of the next HDU (if any) and with the size
         * of the whole file to see if this is the last HDU in the file */

        Ok(())
    }

    fn read_record(&self, key_number: usize) -> Result<String> {
        Ok("".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{FitsFile, FileMode, Card};

    #[test]
    fn opening_a_file() {
        let f = FitsFile::open("testdata/full_example.fits").unwrap();
    }

    #[test]
    fn card_parse() {
        let card_str = "FOO     = 2";
        let card: Card = card_str.parse().unwrap();
        assert_eq!(card, Card {
            name: "FOO".to_string(),
            str_value: "2".to_string(),
            comment: None,
        })
    }

    #[test]
    fn card_parse_with_comment() {
        let card_str = "FOO     = 2  / TEST";
        let card: Card = card_str.parse().unwrap();
        assert_eq!(card, Card {
            name: "FOO".to_string(),
            str_value: "2".to_string(),
            comment: Some("TEST".to_string()),
        })
    }
}
