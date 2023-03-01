use std::collections::HashMap;

use easegress_macros::easegress_object;
use easegress_sdk::*;

use uap::UAP;

static UA_DEVICE_HEADER: &str = "x-ua-device";
static UA_OS_HEADER: &str = "x-ua-os";

#[easegress_object]
struct ParseUserAgent {
    uap: UAP,
}

#[easegress_object]
impl Program for ParseUserAgent {
    fn new(_param: HashMap<String, String>) -> Self {
        let regexes = include_bytes!("../../regexes.yaml");
        Self {
            uap: UAP::from_str(std::str::from_utf8(regexes.as_slice()).unwrap()).unwrap(),
        }
    }

    fn run(&self) -> i32 {
        let user_agent = request::get_header("user-agent".to_string());

        let device = self.uap.parse_device(&user_agent);
        request::set_header(UA_DEVICE_HEADER.to_string(), device.device.unwrap().to_string());

        let os = self.uap.parse_os(&user_agent);
        request::set_header(UA_OS_HEADER.to_string(), os.os.unwrap().to_string());
        0
    }
}
