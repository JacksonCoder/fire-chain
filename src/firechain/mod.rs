pub mod enums;
mod parser;
mod crypto;
use self::enums::FireChainStatus;
use std::fs::File;
use std::io::Read;
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

pub fn chain_exists(chain: &Handler) -> bool {
    match chain.status {
        FireChainStatus::Unknown => false,
        FireChainStatus::Good => true,
        FireChainStatus::Bad => false
    }
}

pub struct Handler {
    status: FireChainStatus,
    input: Input,
    transactions: Vec<Transaction>,
    version: Option<u32>
}

pub struct Input {
    res: String
}

pub struct Signature {
    pub_key: String,
    msg: String,
    hash: String
}

pub struct PKey {
    key: String
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

impl Handler {
    pub fn new_unknown(input: Input) -> Handler {
        Handler {
            status: Unknown,
            input: input,
            transactions: vec![],
            version: None
        }
    }

    pub fn parse(&mut self) {
        // Parse chain via JSON conversion
        let json_obj = json_parse(self.input.get_result());
        let (transaction_list, version) = json_read_ledger_file(json_obj);
        self.transactions = transaction_list;
        self.version = Some(version);
    }
}

impl Input {
    pub fn new(s: &str) -> Input {
        let mut file = File::open(s);
        match file {
            Ok(mut file) => {
                let mut output = String::new();
                file.read_to_string(& mut output);
                return Input {
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

impl Signature {
    pub fn new_from_hash(hash: String, public_key_list: Vec<PKey>, msg: String) -> Option<Signature> {
        let pkey = crypto::match_signature(public_key_list, msg.as_str(), hash.as_str());
        match pkey {
            Some(key) => Some(Signature::new(hash, msg, key)),
            None => None
        }
    }
    pub fn new(hash: String, msg: String, pkey: PKey) -> Signature {
        Signature {
            hash: hash,
            msg: msg,
            pub_key: pkey.get_public()
        }
    }
}

impl PKey {
    pub fn get_public(&self) -> String {
        self.key.clone()
    }
}