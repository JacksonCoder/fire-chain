use std::process::exit;
pub fn fatal_error(msg: &str, code: i32) {
    println!("AN ERROR HAS OCCURED IN FIRECHAIN. ERROR MESSAGE BELOW:");
    println!("{}",msg);
    exit(code);
}