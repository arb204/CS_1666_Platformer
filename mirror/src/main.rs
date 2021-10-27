#![allow(unused)]
use std::net::UdpSocket;

//
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn main() -> std::io::Result<()> {
	{
		let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
		// Receives a single datagram message on the socket. If `buf` is too small to hold
		// the message, it will be cut off.
		loop{
			let mut buf = [0; 4];
			let (amt, src) = socket.recv_from(&mut buf)?;
			println!("{:?}", buf);
		}

		// Redeclare `buf` as slice of the received data and send reverse data back to origin.
		// let buf = &mut buf[..amt];
		// buf.reverse();
		// socket.send_to(buf, &src)?;
	} // the socket is closed here
	Ok(())
}