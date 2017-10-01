#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate clap;
extern crate web3;

use clap::{Arg, App};
use web3::futures::Future;
use web3::types::Address;

fn main() {
    pretty_env_logger::init().unwrap();
    let matches = App::new("Task 1")
                      .version(crate_version!())
                      .author("Denis Kolodin <deniskolodin@gmail.com")
                      .about("Taks 1 example")
                      .arg(Arg::with_name("ADDRESS")
                               .help("Address to monitor")
                               .required(true)
                               .index(1))
                      .get_matches();

    let address = matches.value_of("ADDRESS").unwrap();
    let address: Address  = address.parse().unwrap();

    let (_eloop, http) = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(http);

    let balance = web3.eth().balance(address, None).wait().unwrap();

    debug!("Balance is: {:?}", balance);
}
