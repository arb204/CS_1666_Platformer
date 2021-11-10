use std::borrow::Borrow;
use std::net::UdpSocket;
use std::convert::TryInto;

use crate::player::player::Player;

#[derive(Clone, Copy)]
pub(crate) enum NetworkingMode {
    Send,
    Receive,
}

pub(crate) fn get_sending_socket() -> UdpSocket {
    get_socket("127.0.0.1:34255")
}

pub(crate) fn get_receiving_socket() -> UdpSocket {
    get_socket("127.0.0.1:34254")
}

fn get_socket(address: &str) -> UdpSocket {
    UdpSocket::bind(address).expect("couldn't bind to address")
}

// refactor to make safe -- return result
fn get_player_position_and_flip(socket: &mut UdpSocket) -> (f32, f32) {
    let mut buf: [u8; 8] = [0; 8];
    let (_amt, _src) = socket.recv_from(&mut buf).unwrap();

    let mut xpos: [u8; 4] = [0; 4];
    for i in 0..4 {
        xpos[i] = buf[i];
    }

    let mut ypos: [u8; 4] = [0; 4];
    for i in 4..8 {
        ypos[i-4] = buf[i];
    }

    let x = f32::from_le_bytes(xpos);

    let y = f32::from_le_bytes(ypos);
    
    //let x = f32::from_le_bytes(xbytes);
    //let (_amt, _src) = socket.recv_from(&mut buf).unwrap();

    //let y = f32::from_le_bytes(ybytes);
    // TODO: add getting flip
    (x, y)
}

pub(crate) fn receive_data(socket: &mut UdpSocket) -> (f32, f32) {
    get_player_position_and_flip(socket)
}

pub(crate) fn send_data(player: &mut Player, socket: &UdpSocket, _flip: bool) {
    let xpos = player.physics.x().to_le_bytes(); 
    let ypos = player.physics.y().to_le_bytes();
    let buf = [xpos, ypos].concat();
    socket.send(&buf);
    
    /*
    socket.send(player.physics.x().to_ne_bytes().borrow());
    socket.send(player.physics.y().to_ne_bytes().borrow());
    */
    
}