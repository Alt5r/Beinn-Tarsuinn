
pub enum ReqType {
    HTTP,

}

pub fn requestParsing(request:&Vec<String>) -> ReqType {
    if request[0].to_lowercase().contains("http") {
        ReqType::HTTP
    } else {
        ReqType::HTTP
    }
}