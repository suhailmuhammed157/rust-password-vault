use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Error, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Service {
    pub service: String,
    pub username: String,
    pub password: String,
}

impl Service {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_service(service: String, username: String, password: String) -> Self {
        Self {
            service,
            username,
            password,
        }
    }

    /* convert raw string into Self struct */
    fn read_raw_json(raw_data: &str) -> Result<Self, Error> {
        let parsed: Service = serde_json::from_str(raw_data).expect("Failed to deserialize");
        Ok(parsed)
    }

    /* convert Self struct into raw string*/
    fn convert_to_json(&self) -> String {
        let json_data = serde_json::to_string(self).expect("Failed to serialize into json");
        json_data
    }

    pub fn write_to_file(&self) {
        let self_in_json = self.convert_to_json();
        let json_output = format!("{}\n", self_in_json);

        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("password.json")
        {
            Ok(mut file) => {
                if let Err(err) = file.write_all(json_output.as_bytes()) {
                    eprintln!("Error while writing to file: {}", err)
                } else {
                    println!("Data added successfully")
                }
            }
            Err(err) => {
                eprintln!("Error while opening the file: {}", err)
            }
        }
    }
}

/* Prompt to the console and get user input */
pub fn prompt(prompt_message: &str) -> String {
    print!("{}", prompt_message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/* Get all passwords from the file */
pub fn get_passwords_from_file() -> Result<Vec<Service>, Error> {
    let file = File::open("password.json")?;
    let reader = BufReader::new(file);

    let mut services = Vec::new();

    for line in reader.lines() {
        if let Ok(service_data_json) = line {
            if let Ok(service) = Service::read_raw_json(&service_data_json) {
                services.push(service);
            }
        }
    }
    Ok(services)
}
