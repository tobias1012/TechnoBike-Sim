use std::{net::TcpStream, net::UdpSocket, io::Write};
use std::io;
use std::sync::{Mutex, Arc};
use hex;


use crate::bike::Bike;

pub fn send_http(addr: &str) -> io::Result<usize>{
    let mut tcp = TcpStream::connect(addr).expect("failed to bind");
    tcp.write(b"POST smth")

}

pub fn udp_loop(addr: &str, bike: Arc<Mutex<Bike>>) {
    let udp = UdpSocket::bind("127.0.0.1:6334").expect("yeehaw");
    loop {
        //construct the packet to send
        let unlocked_bike = bike.lock().unwrap().clone();

        let mut buf: [u8; 49] = [0; 49];
        
        //magic bytes
        hex::decode_to_slice("14310102", &mut buf[0..4]).expect("msg");

        //Enter serial number 
        let id = unlocked_bike.id.as_bytes();
        buf[5..21].clone_from_slice(id);

        //send the package
        match udp.send_to(&buf, addr){
            Ok(_) => {},
            Err(_) => {println!("failed to send udp")}
        }
    }
}