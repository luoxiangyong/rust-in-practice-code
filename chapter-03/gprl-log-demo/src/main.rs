extern crate log;
extern crate simple_logger;
#[warn(unused_imports)]
use log::{info, warn};

fn main() {
    simple_logger::init().unwrap();
    info!("This a info log from log crate");
    warn!("This is an warn message.");

}
