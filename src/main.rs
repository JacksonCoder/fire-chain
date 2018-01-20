mod firechain;
mod arg;
use firechain::setup_interface;
use firechain::chain_exists;
use firechain::ChainHandler;
use firechain::ChainInput;
use std::env::args;
use arg::argparse;

fn server(chain: ChainHandler) {
    // Run continous HTTP server and communicate with chain handle
}

fn main() {

    let argvec: Vec<String> = {
        let mut vec:Vec<String> = vec![];
        for mut i in args() {
            vec.push(i);
        }
        vec
    };

    let arghandle = argparse(argvec);

    // Create chain input
    let chaininput = ChainInput::new(arghandle.ledger_file_path.as_str());

    // Initialize chain handle
    let mut chainhandle = ChainHandler::new_unknown(chaininput);

    chainhandle.parse();

    if chain_exists(&chainhandle) {
        server(chainhandle);
    }
}
