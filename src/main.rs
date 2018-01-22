mod firechain;
mod server;

extern crate clap;
use firechain::setup_interface;
use firechain::chain_exists;
use firechain::Handler;
use firechain::Input;
use clap::{App, Arg, SubCommand, ArgMatches};

extern crate tiny_http;

use tiny_http::Server;
use std::sync::Arc;

extern crate url;

use url::{Url, ParseError};

fn server(matches: ArgMatches) {
    // Get ledger file path
    let ledger_file_path = match matches.value_of("dbpath") {
        Some(path) => path,
        None => "transaction_ledger.json"
    };
    // Get designated client reciever port (default is 4545)
    let port = match matches.value_of("port") {
        Some(portnumber) => portnumber.parse::<i32>().unwrap(),
        None => 4545
    };
    // Worker threads (default 4)
    let threads = match matches.value_of("threads") {
        Some(threadcount) => threadcount.parse::<i32>().unwrap(),
        None => 4
    };
    // Launch on interface 0.0.0.0
    println!("Starting server on port {}", port);
    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();
    let server = Arc::new(server);
    let mut guards = Vec::with_capacity(4);

    for i in 0..4 {
        // Get local thread copy of server
        let server = server.clone();
        // Get handler
        let input = Input::new(ledger_file_path);
        let handle = Handler::new_unknown(input);

        let guard = std::thread::spawn(move || {
            loop {
                let rq = server.recv().unwrap();
                println!("Recieved request: {:?} on worker thread {}", rq, i);

                // Switch request path
                let rqurl = Url::parse(&format!("http://localhost:4545{}", rq.url())).unwrap();
                let response = match rqurl.path() {
                    "/fetch" => {
                        server::not_implemented()
                    },
                    _ => {
                        server::not_found()
                    }
                };
                rq.respond(response);
            }
        });
        guards.push(guard);
    }
    loop {} // Infinite loop until manual program exit
}

fn main() {
    let matches = App::new("FireChain")
        .version("0.1.0")
        .author("Jackson Lewis <st.japa6@gmail.com>")
        .about("A simple, very fast linkage system for semi-decentralized protocols")
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a custom config file'
                              <COMMAND>              'Set the command to be used'
                              -l, --log=[FILE]                'Set an log file to output to'
                              -d, --dbpath=[FILE]                 'Set the path to the transaction lists database file'
                              -p, --port=[INT]                'Set the port for the server to use'")
        .get_matches();
    let command = matches.value_of("COMMAND").unwrap();
    match command {
        "server" => server(matches.clone()),
        _ => firechain::error::fatal_error("Invalid command.", -1)
    }
}
