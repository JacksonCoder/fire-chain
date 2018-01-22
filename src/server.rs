extern crate tiny_http;

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

pub fn invalid_args() -> Response<Cursor<Vec<u8>>> {
    Response::from_string("{
        \"did_succeed\": false,
        \"msg\": \"Invalid arguments or not enough arguments were passed to the requested query, command, or resource.\"
    }").with_status_code(200)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
}