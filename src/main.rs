#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate clap;
extern crate web3;
extern crate futures;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;

use clap::{Arg, App};
use web3::types::{Address, U256};
use futures::{future, Stream};
use tokio_core::reactor::{Core, Interval};
use std::time::Duration;
use std::io::Read;
use std::fs::File;

fn main() {
    pretty_env_logger::init().unwrap();
    let matches = App::new("Task 1")
                      .version(crate_version!())
                      .author("Denis Kolodin <deniskolodin@gmail.com")
                      .about("Taks 1 example")
                      .arg(Arg::with_name("FILE")
                               .help("File with addresses")
                               .required(true)
                               .index(1))
                      .get_matches();

    let file = matches.value_of("FILE").unwrap();
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let addresses: Vec<Address> = serde_json::from_str(&contents).unwrap();

    println!("{}", contents);

    let mut core = Core::new().unwrap();

    let duration = Duration::new(5, 0);
    let handle = core.handle();
    let workers = addresses.into_iter().map(|address| {
        let http = web3::transports::Http::with_event_loop("http://localhost:8545", &handle, 1).unwrap();
        let web3 = web3::Web3::new(http);

        let mut prev = U256::from(0);
        Interval::new(duration, &handle).unwrap()
        .then(move |_| {
            web3.eth().balance(address, None)
        })
        .for_each(move |new| {
            if prev != new {
                prev = new;
                debug!("Balance is: {:?}", new);
            }
            Ok(())
        })
    });

    let collector = future::join_all(workers);
    core.run(collector).unwrap();
}
