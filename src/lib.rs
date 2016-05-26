//! This crate is a small wrapper around the [buerostatus](https://github.com/fsr/buerostatus) API
//! of the [@fsr](https://github.com/fsr) that tells you whether someone is in our office or not.
//!
//! # Example
//! ```
//! # use buerostatus::*;
//! if let Ok(is_open) = get_buerostatus() {
//!     if is_open { println!("Someone's inside!"); }
//!     else { println!("No one is there..."); }
//! }
//! else {
//!     println!("An error occured!");
//! }
//! ```
//!
//! The function `get_buerostatus()` offers a precise error message in case of an error. See the enum
//! `ApiError` for more information.

extern crate hyper;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::error::Error as HyperError;
use std::io::Read;

/// A set of errors that may occur during the API call.
#[derive(Debug)]
pub enum ApiError {
    /// An error originating from the `hyper` crate such as "No Internet connection".
    Network(HyperError),
    /// The Return Code of the website is not _200_.
    StatusNotOk(hyper::status::StatusCode),
    /// The API call delivered a wrong message.
    WrongMessage
}

/// Gets the buerostatus from ifsr.de.
///
/// Returns `true`, when the office is open, `false` if not and an `ApiError` if any form of error is occured during execution.
///
/// # Example
/// ```
/// # use buerostatus::*;
/// if let Ok(is_open) = get_buerostatus() {
///     if is_open {
///         println!("Someone's inside!");
///     }
/// }
/// else {
///     // Error Handling...
/// }
/// ```
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

    let mut message = String::new();
    res.read_to_string(&mut message).unwrap();

    // Check the content of the response
    match message.as_ref() {
        "0" => Ok(false),
        "1" => Ok(true),
        _   => Err(ApiError::WrongMessage), // just in case, the server _should_ always return 1 or 0.
    }
}
