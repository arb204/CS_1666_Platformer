#![allow(unused)]

use std::net::UdpSocket;

fn main() {
	{
		let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
		// Receives a single datagram message on the socket. If `buf` is too small to hold
		// the message, it will be cut off.

		loop{
			let mut buf = [0; 4];
			let (amt, src) = socket.recv_from(&mut buf).unwrap();
			let x = f32::from_le_bytes(buf);
			let (amt, src) = socket.recv_from(&mut buf).unwrap();
			let y = f32::from_le_bytes(buf);
			println!("{:?} {:?}", x, y);
		}

	} // the socket is closed here
}