extern crate json;
use self::json::parse;
use self::json::stringify;
use firechain::error::fatal_error;

pub fn json_parse(input: &str) -> json::JsonValue {
    let res = parse(input);
    match res {
        Ok(json_obj) => {return json_obj; },
        Err(E) => fatal_error(format!("The designated FireChain file has invalid JSON data: {}",E).as_str(),-1)
    }
    json::JsonValue::Null
}

pub fn json_to_str(input: json::JsonValue) -> String {
    stringify(input)
}