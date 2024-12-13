// rules for firewall here
use std::collections::HashMap;
use core::net;

mod reqtypes;
use reqtypes::*;

pub fn master(request:HashMap<String, String>) -> Result<String, net::AddrParseError> {
    println!("{:?}", request);
    
    let requestType: ReqType = requestParsing(&request);

    match requestType {
        ReqType::HTTP => Ok(HTTP_handler(request)),
        _ => Ok(ERROR_handler(request)),
    }


    
}


pub fn HTTP_handler(request: HashMap<String, String>) -> String {
    unimplemented!();
}

pub fn ERROR_handler(request:HashMap<String, String>) -> String {
    unimplemented!();
}