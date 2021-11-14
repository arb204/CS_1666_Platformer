use std::borrow::Borrow;
use std::net::UdpSocket;
use crate::player::Player;


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

// refactor to make safe -- return result
fn get_portal_position_and_flip(socket: &mut UdpSocket) -> (f32, f32, f32, f32) {
    let mut buf = [0; 4];
    //let mut portal_vec: Vec<(f32, f32)> = vec!();

    let (_amt, _src) = socket.recv_from(&mut buf).unwrap();
    let x1 = f32::from_le_bytes(buf);
    let (_amt, _src) = socket.recv_from(&mut buf).unwrap();
    let y1 = f32::from_le_bytes(buf);

    let (_amt, _src) = socket.recv_from(&mut buf).unwrap();
    let x2 = f32::from_le_bytes(buf);
    let (_amt, _src) = socket.recv_from(&mut buf).unwrap();
    let y2 = f32::from_le_bytes(buf);
    // TODO: add getting flip
    (x1,y1,x2,y2)
}

pub(crate) fn receive_player_data(socket: &mut UdpSocket) -> (f32, f32) {
    get_player_position_and_flip(socket)
}

pub(crate) fn receive_portal_data(socket: &mut UdpSocket) -> (f32, f32, f32, f32) {
    get_portal_position_and_flip(socket)
}

pub(crate) fn send_data(player: &mut Player, socket: &UdpSocket, _flip: bool) {
    let xpos = player.physics.x().to_le_bytes(); 
    let ypos = player.physics.y().to_le_bytes();
    let buf = [xpos, ypos].concat();
    socket.send(&buf);
    
    /*
    socket.send(player.physics.x().to_ne_bytes().borrow());
    socket.send(player.physics.y().to_ne_bytes().borrow());

    for p in &player.portal.portals {
        socket.send(p.x().to_ne_bytes().borrow());
        socket.send(p.y().to_ne_bytes().borrow());
    }

    
}