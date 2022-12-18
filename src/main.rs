use std::thread;
use std::sync::{Mutex, Arc};
mod bike;
use bike::Bike;

mod networking;
use networking::{send_http, udp_loop};

fn main() {
    //init the bikes
    let id = std::env::args().nth(1).expect("NEEDS a id for bikes");
    let bike = Arc::new(Mutex::new(Bike::new(id.as_str())));
    
    //init the networking
    let bind_addr = "127.0.0.1:1508";

    //Run the http just to make sure it behaves the same
    loop {
        match send_http(&bind_addr) {
            Ok(_n) => break,
            Err(_n) => {println!("Failed to send http")},
        }
    }
    

    //start an UDP loop that sends data in another thread
    let bike_clone = bike.clone(); 
    thread::spawn(move || {udp_loop(&bind_addr, bike_clone)});

    //Start an input loop that gets the input from user to the bike
    loop {
        
    }
}
