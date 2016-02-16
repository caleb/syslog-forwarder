extern crate mioco;
extern crate env_logger;

use std::net::{SocketAddr,ToSocketAddrs};
use std::io::{Read, Write};
use mioco::udp::{UdpSocket};

const DEFAULT_LISTEN_ADDR: &'static str = "127.0.0.1:5555";
const DEFAULT_DEST_ADDR: &'static str = "rsyslog:514";

fn main() {
  mioco::start(|| {
    let addr = DEFAULT_LISTEN_ADDR.parse().unwrap();
    let out_addr = DEFAULT_DEST_ADDR.to_socket_addrs().unwrap().nth(1).unwrap();

    let mut listener = UdpSocket::bound(&addr).unwrap();
    let out_sock = try!(UdpSocket::v4());

    println!("Starting syslog-fowarder server on {:?}", listener.local_addr().unwrap());

    loop {
      let mut buf = [0u8; 1024 * 2];
      let (size, _in_addr) = try!(listener.read(&mut buf));

      if size > 0 {
        let out_clone = out_addr.clone();
        let mut out_sock_clone = try!(out_sock.try_clone());
        println!("Received {} bytes", size);
        mioco::spawn(move || {
          try!(out_sock_clone.write(&buf[0..size], &out_clone));
          Ok(())
        });
      }
    }
  });
}
