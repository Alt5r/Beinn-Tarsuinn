use std::collections::HashMap;


pub enum ReqType {
    HTTP,
    Unknown

}

pub fn requestParsing(request:&HashMap<String, String>) -> ReqType {
    if let Some(rtype) = request.get("User-Agent") {
        let usr_agnt = request.get("User-Agent").unwrap();
        
        if usr_agnt.contains("chrome") {
            ReqType::HTTP
        } else {
            // some code hjere
            ReqType::Unknown
        }
    } else {
        // no user agent = sus
        ReqType::Unknown
    }
    /* 
    if request[&0].to_lowercase().contains("http") {
        ReqType::HTTP
    } else {
        ReqType::HTTP
    }
    */
}

/// this will check user agents again known bot agents, crawlers and indetifiers of a DDOS attack and flag this request if it contains these indicatorsa 
pub fn userAgentFlag(request:&Vec<String>) -> bool {
    let IIC = ["curl", "python"];
    true
}

// 22

// endpoint flaggers