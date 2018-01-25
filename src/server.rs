extern crate tiny_http;
extern crate url;
use firechain;
use firechain::setup_interface;
use firechain::chain_exists;
use firechain::Handler;
use firechain::Input;

use std::io::Cursor;
use std::str::FromStr;
use tiny_http::{Response, Header};

pub fn not_implemented() -> Response<Cursor<Vec<u8>>> {
    Response::from_string("{
        \"did_succeed\": true,
        \"type\" : \"not-yet-implemented\",
        \"msg\" : \"This feature is not implemented yet\"
    }").with_status_code(200)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
}


pub fn not_found() -> Response<Cursor<Vec<u8>>> {
    Response::from_string("{
        \"did_succeed\": false,
        \"msg\": \"The requested resource or command was not found.\"
    }").with_status_code(404)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
}

fn invalid_args() -> Response<Cursor<Vec<u8>>> {
    Response::from_string("{
        \"did_succeed\": false,
        \"msg\": \"Invalid arguments or not enough arguments were passed to the requested query, command, or resource.\"
    }").with_status_code(200)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
}

fn return_json(string: &str) -> Response<Cursor<Vec<u8>>> {
    Response::from_string(string).with_status_code(200)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
}

pub fn handle_fetch(query: url::form_urlencoded::Parse) -> Response<Cursor<Vec<u8>>> {
    /*
    Input format for /fetch:
    /fetch?type=record|history&&user=<some public key>&&time_nonce=<some epoch value>&&validation_sig=<signed epoch value>
    if requester for history: for_user=<some public key>
    if requester for record: record_hash=<hash> or unique_rec_ident=<record identifier>

    */
    let mut is_record_request: bool;
    let mut user:Option<String> = None;
    let mut time_nonce: Option<String> = None;
    let mut validation_sig: Option<String> = None;
    for qpair in query {
        if qpair.0.clone().into_owned() == "type" {
            is_record_request = match qpair.1.clone().into_owned().clone().as_str() {
                "record" => true,
                "history" => false,
                _ => true
            }
        }
        if qpair.0.clone().into_owned() == "user" {
            user = Some(qpair.1.clone().into_owned());
        }
        if qpair.0.clone().into_owned().clone() == "time_nonce" {
            time_nonce = Some(qpair.1.clone().into_owned().clone());
        }
        if qpair.0.clone().into_owned().clone() == "validation_sig" {
            validation_sig = Some(qpair.1.clone().into_owned().clone());
        }
    }
    if let None = user {
        return invalid_args();
    }
    if let None = time_nonce {
        return invalid_args();
    }
    if let None = validation_sig {
        return invalid_args();
    }

    not_implemented()
}