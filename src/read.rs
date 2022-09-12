use crate::ReadData;
use std::fmt::Display;
use std::fs::{self, File};
use std::io;
use std::io::{BufRead, BufReader};

impl ReadData {
    pub fn read<T: Display>(&self, path: T) -> Result<Vec<String>, io::Error> {
        let file = File::open(path.to_string());
        let mut v: Vec<String> = Vec::new();
        match file {
            Ok(success) => {
                let reader = BufReader::new(success);
                for line in reader.lines() {
                    match line {
                        Ok(l) => {
                            v.push(l);
                        }
                        Err(e) => {
                            v.push(e.to_string());
                        }
                    }
                }
            }
            Err(error) => {
                v.push(error.to_string());
                return Err(error);
            }
        }
        return Ok(v);
    }
    pub fn read_one<T: Display>(&self, path: T) -> Result<String, io::Error> {
        let data = fs::read_to_string(path.to_string());
        match data {
            Err(e) => Err(e),
            Ok(final_data) => Ok(final_data),
        }
    }
}
