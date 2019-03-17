extern crate clap;

use clap::App;
use clap::Arg;
use std::env;
use dz1::config::config::Config;

const DEFAULT_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "9000";

fn main() {
    let matches = App::new("Highload homework")
        .version("1.0.0")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .required(true)
        )
        .arg(Arg::with_name("address")
            .short("addr")
            .value_name("ADDRESS")
            .default_value(DEFAULT_ADDRESS)
        )
        .arg(Arg::with_name("port")
            .short("p")
            .value_name("PORT")
            .default_value(DEFAULT_PORT)
        )
        .get_matches();

    let config_name = matches.value_of("config").unwrap();
    let address = matches.value_of("address").unwrap_or_default().to_owned();
    let port = matches.value_of("port").unwrap_or_default().to_owned();

    let config = match Config::read(&config_name[..]) {
        Ok(cfg) => cfg,
        Err(err) => panic!(err),
    };

    println!("{:?}", config);
    println!("Address: {} Port: {}", address, port);
}

// fn main() {
//     let s = "/foo/bar/".as_bytes();
//     s.as_bytes();
// }