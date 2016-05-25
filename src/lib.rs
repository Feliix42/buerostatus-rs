extern crate hyper;

use hyper::Client;
use hyper::status::StatusCode;
use std::io::Read;

pub fn get_buerostatus() -> Option<bool> {
    let client = Client::new();
    let url = "https://www.ifsr.de/buerostatus/output.php";

    // Make the request to ifsr.de
    let mut res = match client.get(url).send() {
        Ok(resp) => resp,
        Err(_)   => return None,
    };

    // Check if response from Server is Status 200.
    if res.status != StatusCode::Ok {
        return None;
    }

    let mut onezero = String::new();
    res.read_to_string(&mut onezero).unwrap();

    // Check the content of the response
    match onezero.as_ref() {
        "0" => Some(false),
        "1" => Some(true),
        _   => { // just in case, the server _should_ always return 1 or 0.
            println!("Unknown return value from ifsr.de!");
            None
        }
    }
}
