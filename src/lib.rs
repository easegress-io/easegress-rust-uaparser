mod models;
mod errors;
mod uap;
mod regexes;

pub use uap::UAP;

use std::collections::HashMap;
use easegress_sdk::*;
use easegress_macros::easegress_object;

#[easegress_object]
struct ParseUserAgent {
    uap: UAP,
}

#[easegress_object]
impl Program for ParseUserAgent {
    fn new(_param: HashMap<String, String>) -> Self {
        let regexes = include_bytes!("../regexes.yaml");
        Self {
            uap: UAP::from_str(std::str::from_utf8(regexes.as_slice()).unwrap()).unwrap(),
        }
    }

    fn run(&self) -> i32 {
        let user_agent = request::get_header("user-agent".to_string());

        let device = self.uap.parse_device(&user_agent);
        request::set_header("x-ua-device".to_string(), device.device.unwrap().to_string());

        let os = self.uap.parse_os(&user_agent);
        request::set_header("x-ua-os".to_string(), os.os.unwrap().to_string());

        let headers = request::get_all_header();
        for header in headers.iter() {
            for val in header.1.iter() {
                log(LogLevel::Info, format!("{}, {}", header.0, val));
            }
        }
        0
    }
}
