#[allow(dead_code)]
#[allow(unused_imports)]

#[macro_use]


mod filesync_rs_server;

use filesync_rs_server::{FSRServer, Display};
use clap::{Arg, App};
use tokio::net::{TcpStream, tcp::ConnectFuture};
use std::cell::RefCell;

pub static mut SHUTDOWN: RefCell<u32> = RefCell::new(0);


fn main() {
    println!("Hello, world!");


    let matches = App::new("filesync-rs")
        .version("0.1")
        .author("Richard Joe")
        .about("The client for monitoring the sori platform.")
        .arg(Arg::with_name("base")
             .short("b")
             .long("base")
             .help("Destination IP address to connect.")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .help("Port for listening")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("dir")
             .short("d")
             .long("dir")
             .help("directory for sync..")
             .takes_value(true)
             .required(true))
        .get_matches();
    
    println!("{}", matches.value_of("base").unwrap().to_string());
    println!("{}", matches.value_of("port").unwrap().to_string());
    println!("{}", matches.value_of("dir").unwrap().to_string());
    
    // let fsr_server = FSRServer::new(matches.value_of("base").unwrap().to_string(),
    // matches.value_of("port").unwrap().to_string(),
    // matches.value_of("dir").unwrap().to_string());
    
    let future = Display(FSRServer::new(matches.value_of("base").unwrap().to_string(),
                                        matches.value_of("port").unwrap().to_string(),
                                        matches.value_of("dir").unwrap().to_string()));
        
    tokio::run(future);
}
