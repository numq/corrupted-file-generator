use std::error::Error;
use std::fs::File;
use std::io::Write;

use base64::Engine;
use rand::RngCore;

pub struct FileGenerator;

impl FileGenerator {
    pub fn create(path: &str, size_in_bytes: u64) -> Result<File, Box<dyn Error>> {
        let mut rng = rand::thread_rng();
        let mut random_data: Vec<u8> = vec![0; size_in_bytes as usize];
        rng.fill_bytes(&mut random_data);
        let encoded_string = base64::engine::general_purpose::STANDARD.encode(&random_data);
        let mut file = File::create(path)?;
        file.write_all(encoded_string.as_bytes())?;
        file.set_len(size_in_bytes)?;
        Ok(file)
    }
}