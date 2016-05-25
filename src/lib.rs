extern crate hyper;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::error::Error as HyperError;
use std::io::Read;

#[derive(Debug)]
pub enum ApiError {
    Network(HyperError),
    StatusNotOk(hyper::status::StatusCode),
    WrongMessage
}

pub fn get_buerostatus() -> Result<bool, ApiError> {
    let client = Client::new();
    let url = "https://www.ifsr.de/buerostatus/output.php";

    // Make the request to ifsr.de
    let mut res = match client.get(url).send() {
        Ok(resp) => resp,
        Err(err)   => return Err(ApiError::Network(err)),
    };

    // Check if response from Server is Status 200.
    if res.status != StatusCode::Ok {
        return Err(ApiError::StatusNotOk(res.status));
    }

    let mut onezero = String::new();
    res.read_to_string(&mut onezero).unwrap();

    // Check the content of the response
    match onezero.as_ref() {
        "0" => Ok(false),
        "1" => Ok(true),
        _   => Err(ApiError::WrongMessage), // just in case, the server _should_ always return 1 or 0.
    }
}
