use lazy_static::lazy_static;
use worker::{Env, event, Fetch, Request, Response};

use uap::UAP;

static UA_DEVICE_HEADER: &str = "x-ua-device";
static UA_OS_HEADER: &str = "x-ua-os";

fn new_regexes() -> UAP {
    let regexes = include_bytes!("../../regexes.yaml");
    UAP::from_str(std::str::from_utf8(regexes.as_slice()).unwrap()).unwrap()
}

lazy_static! {
    static ref UAPAERSER: UAP = new_regexes();
}

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: worker::Context) -> worker::Result<Response> {
    let mut new_req = req.clone_mut().unwrap();

    let user_agent = req.headers().get("user-agent").unwrap();
    if let Some(ua) = user_agent {
        let device = UAPAERSER.parse_device(&ua);
        let os = UAPAERSER.parse_os(&ua);
        match new_req.headers_mut() {
            Ok(headers) => {
                headers.set(UA_DEVICE_HEADER, device.device.unwrap().as_ref()).unwrap();
                headers.set(UA_OS_HEADER, os.os.unwrap().as_ref()).unwrap();
            }
            Err(_) => {}
        }
    }
    let response = Fetch::Request(new_req).send().await;
    match response {
        Ok(res) => {
            Ok(Response::from(res))
        }
        Err(e) => {
            Response::error(e.to_string(), 400)
        }
    }
}
