use std::collections::HashMap;


pub enum ReqType {
    HTTP,

}

pub fn requestParsing(request:&HashMap<String, String>) -> ReqType {
    if let Some(rtype) = request.get("")
    if request[0].to_lowercase().contains("http") {
        ReqType::HTTP
    } else {
        ReqType::HTTP
    }
}

/// this will check user agents again known bot agents, crawlers and indetifiers of a DDOS attack and flag this request if it contains these indicatorsa 
pub fn userAgentFlag(request:&Vec<String>) -> bool {
    let IIC:Vec<String> = ["curl", "python"];
}

// 22

// endpoint flaggers