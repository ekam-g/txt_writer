use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{Error, Write};

use crate::WriteData;

impl WriteData {
    pub fn add<T: Display, E: Display>(&self, data: T, path: E) -> Result<(), Error> {
        let final_path = path.to_string();
        let output = OpenOptions::new().write(true).append(true).open(final_path);
        return WriteData {}.process(output, &data);
    }
    pub fn replace<T: Display, E: Display>(&self, data: T, path: E) -> Result<(), Error> {
        let final_path = path.to_string();
        let output = File::create(final_path);
        return WriteData {}.process(output, &data);
    }
    fn process<T: Display>(&self, output: Result<File, Error>, data: T) -> Result<(), Error> {
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
