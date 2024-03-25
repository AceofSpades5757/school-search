//! Update the data used in this package.

use std::{error::Error, fs::File, io::prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    // Settings
    let data_file: &str = "./school-search/src/world_universities_and_domains.json";

    // Download
    {
        let mut file = File::create(data_file).expect("Failed to create data_file");
        // https://github.com/Hipo/university-domains-list/blob/master/world_universities_and_domains.json
        let url = "https://github.com/Hipo/university-domains-list/raw/master/world_universities_and_domains.json";
        let response = reqwest::blocking::get(url)?;

        let content = response.text()?;
        file.write_all(&mut content.as_bytes())?;
    }

    // Add Additional Data
    {
        let mut file = File::open(data_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut schools: Vec<serde_json::Value> = serde_json::from_str(&contents)?;

        // ID for Search Engine
        let mut c = 0;
        for school in &mut schools {
            school
                .as_object_mut()
                .expect("Data is invalid")
                .insert("id".to_string(), serde_json::json!(c));
            c += 1;
        }

        let schools: serde_json::Value = schools.into();
        let file = File::create(data_file)?;
        serde_json::to_writer_pretty(&file, &schools)?;
    }

    Ok(())
}
