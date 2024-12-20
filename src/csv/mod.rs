use std::{error::Error, io, process, fs};
use csv::Reader;

#[derive(Debug, serde::Deserialize)]

/// contains data abouta malicious haeader
pub struct header {
    header_content:String,
    reason:String,
    tool:String,
    cat:String,
    link:String,
    severity:String,
    metadata_usage:String,

}

impl header {

    fn new(v1: String, v2: String, v3: String, v4: String,v5: String, v6: String, v7: String) -> Self {
        

        header {header_content: v1, reason: v2, tool:v3, cat:v4, link:v5, severity:v6, metadata_usage:v7}
    }

    pub fn get_header(&self) -> &String {
        &self.header_content
    }

    pub fn get_reason(&self) -> &String {
        &self.reason
    }
    pub fn get_tool(&self) -> &String {
        &self.tool
    }
    pub fn get_cat(&self) -> &String {
        &self.cat
    }
    pub fn get_link(&self) -> &String {
        &self.link
    }
    pub fn get_severity(&self) -> &String {
        &self.severity
    }
    pub fn get_meta(&self) -> &String {
        &self.metadata_usage
    }

}

pub async fn get_agents() -> Result<Vec<header>, Box<dyn Error + Send + Sync>> {
    if fs::metadata("data.csv").is_ok() {
        println!("the file is found")
    } else {
        println!("file not found ")
    }
    let mut rdr = csv::Reader::from_path("data.csv")?;
    println!("called");
    
    let mut lst:Vec<header> = Vec::new();
    for result in rdr.records() {
        let record = result?;

        //println!("Record: {:?}", record);
        let r:Vec<String> = record.iter().map(|s| s.to_string()).collect();

        lst.push(header::new(r[0].clone(), r[1].clone(), r[2].clone(), r[3].clone(), r[4].clone(), r[7].clone(), r[8].clone()))

    }
    Ok(lst)
}