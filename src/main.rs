#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate clap;
extern crate web3;
extern crate futures;
extern crate tokio_core;

use clap::{Arg, App};
use web3::futures::Future;
use web3::types::{Address, U256};
use futures::Stream;
use tokio_core::reactor::{Core, Interval};
use std::time::Duration;

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


    let duration = Duration::new(5, 0);
    let mut core = Core::new().unwrap();
    let interval = Interval::new(duration, &core.handle()).unwrap();
    let interval = interval.then(|_| {
        web3.eth().balance(address, None)
    });
    let mut prev = U256::from(0);
    let collector = interval.for_each(|new| {
        if prev != new {
            prev = new;
            debug!("Balance is: {:?}", new);
        }
        Ok(())
    });

    core.run(collector).unwrap();

}
