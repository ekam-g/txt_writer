use std::fs::{File, OpenOptions};
use std::io::{Error, Write};

use crate::WriteData;

impl WriteData {
    pub fn drop_add(&self, data: String, path: String) -> Result<(), Error> {
        WriteData {}.add(&data, path)
    }
    pub fn drop_replace(&self, data: String, path: String) -> Result<(), Error> {
        WriteData {}.replace(&data, path)
    }
    pub fn add(&self, data: &String, path: String) -> Result<(), Error> {
        let output = OpenOptions::new().write(true).append(true).open(path);
        return WriteData {}.process(output, &data);
    }
    pub fn replace(&self, data: &String, path: String) -> Result<(), Error> {
        let output = File::create(path);
        return WriteData {}.process(output, &data);
    }
    fn process(&self, output: Result<File, Error>, data: &String) -> Result<(), Error> {
        return match output {
            Ok(mut file) => {
                let error = write!(file, "{}\n", data);
                match error {
                    Ok(..) => {
                        let final_error = file.flush();
                        match final_error {
                            Err(e) => Err(e),
                            Ok(_) => Ok(()),
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        };
    }
}
