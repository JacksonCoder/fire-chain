pub mod enums;
mod parser;
use self::enums::FireChainStatus;
use std::fs::File;
use std::io::Read;
use std;
use self::parser::json_parse;
extern crate json;
use self::json::JsonValue;
use self::json::JsonValue::*;
use std::string::String;
use self::enums::FireChainStatus::Unknown;
pub mod error;
use self::error::fatal_error;

pub fn setup_interface() {

}

pub fn chain_exists(chain: &ChainHandler) -> bool {
    match chain.status {
        FireChainStatus::Unknown => false,
        FireChainStatus::Good => true,
        FireChainStatus::Bad => false
    }
}

pub struct ChainHandler {
    status: FireChainStatus,
    input : ChainInput,
    parsed: Vec<Transaction>,
    version: Option<u32>
}

pub struct ChainInput {
    res: String
}

pub struct Signature {
    // Add stuff here later
}

pub struct Transaction {
    signatures: Vec<Signature>,
    hash: String
    // Add other fields here
}

fn json_read_ledger_file(json_obj: JsonValue) -> (Vec<Transaction>,u32) {
    match json_obj {
        Array(vec) => {
            return (vec![],10);
        },
        _ => fatal_error("Your ledger file is invalid.",1)
    }
    (vec![],1) // Code will never reach here
}

impl ChainHandler {

    pub fn new_unknown(input: ChainInput) -> ChainHandler {
        ChainHandler {
            status: Unknown,
            input: input,
            parsed: vec![],
            version: None
        }
    }

    pub fn parse(&mut self) {
        // Parse chain via JSON conversion
        let json_obj = json_parse(self.input.res.as_str());
        let (chain,version) = json_read_ledger_file(json_obj);
        self.parsed = chain;
        self.version = Some(version);
    }
}

impl ChainInput {

    pub fn new(s: &str) -> ChainInput {
        let mut file = File::open(s);
        match file {
            Ok(mut file) => {
                let mut output = String::new();
                file.read_to_string(& mut output);
                return ChainInput {
                    res: output
                };
            },
            Err(E) => fatal_error(format!("The file '{}' could not be opened.",s).as_str(),-1)
        }
        panic!("")
    }

    pub fn get_result(&self) -> &str {
        self.res.as_str()
    }
}