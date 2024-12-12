// rules for firewall here

use core::net;

mod reqtypes;
use reqtypes::*;

pub fn master(request: Vec<String>) -> Result<String, net::AddrParseError> {
    println!("{:?}", request);
    
    let requestType: ReqType = requestParsing(&request);

    match requestType {
        ReqType::HTTP => Ok(HTTP_handler(request)),
        _ => Ok(ERROR_handler(request)),
    }


    
}


pub fn HTTP_handler(request: Vec<String>) -> String {
    unimplemented!();
}

pub fn ERROR_handler(request:Vec<String>) -> String {
    unimplemented!();
}