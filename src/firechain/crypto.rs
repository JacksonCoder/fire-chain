extern crate crypto;

use firechain::PKey;
use self::crypto::ed25519::verify;

pub fn ecdsa_check(key: &str, msg: &str, sig: &str) -> bool {
    verify(msg.as_bytes(), key.as_bytes(), sig.as_bytes())
}

pub fn match_signature(key_list: Vec<PKey>, msg: &str, hash: &str) -> Option<PKey> {
    for key in key_list {
        if ecdsa_check(key.get_public().as_str(), msg, hash) {
            return Some(key);
        }
    }
    None
}