use std::borrow::Borrow;
use std::net::UdpSocket;
use crate::player::Player;

// try getting from nslookup
pub const SEND_ADDR: &str = "127.0.0.1:34255";
pub const REC_ADDR: &str = "127.0.0.1:34254";
const PACKET_SIZE: usize = 25;

#[derive(Clone, Copy)]
pub(crate) enum NetworkingMode {
    Send,
    Receive,
}

pub(crate) fn get_sending_socket() -> UdpSocket {
    get_socket(SEND_ADDR)
}

pub(crate) fn get_receiving_socket() -> UdpSocket {
    get_socket(REC_ADDR)
}

fn get_socket(address: &str) -> UdpSocket {
    let socket = UdpSocket::bind(address).expect("couldn't bind to address");
    let debug = false;
    if debug {
        println!("{:?}", socket);
    }
    socket
}

pub(crate) fn get_packet_buffer(socket: &mut UdpSocket) -> [u8; PACKET_SIZE] {
    let mut buf: [u8; PACKET_SIZE] = [0; PACKET_SIZE];
    let (_amt, _src) =  socket.recv_from(& mut buf).unwrap();
    return buf;
}


// refactor to make safe -- return result
pub(crate) fn get_player_position_and_flip(socket: &mut UdpSocket, buf: &mut [u8; PACKET_SIZE])
    -> Result<(f32, f32, bool), String> {
    let mut xpos: [u8; 4] = [0; 4];
    for i in 0..4 {
        xpos[i] = buf[i];
    }

    let mut ypos: [u8; 4] = [0; 4];
    for i in 4..8 {
        ypos[i-4] = buf[i];
    }
    let flip = match buf[9] {
        0 => false,
        1 => true,
        _ => {Err(String::from("Error: player flip is neither 1 nor 0"))},
    };

    let x = f32::from_le_bytes(xpos);
    let y = f32::from_le_bytes(ypos);

    Ok((x, y, flip))
}

// refactor to make safe -- return result
pub(crate) fn get_portal_position_and_flip(socket: &mut UdpSocket, buf: &mut [u8; PACKET_SIZE]) -> (f32, f32, f32, f32) {
    let mut xpos_1: [u8; 4] = [0; 4];
    for i in 9..13 {
        xpos_1[i-8] = buf[i];
    }

    let mut ypos_1: [u8; 4] = [0; 4];
    for i in 13..17 {
        ypos_1[i-12] = buf[i];
    }

    let mut xpos_2: [u8; 4] = [0; 4];
    for i in 17..21 {
        xpos_2[i-16] = buf[i];
    }

    let mut ypos_2: [u8; 4] = [0; 4];
    for i in 21..25 {
        ypos_2[i-20] = buf[i];
    }

    let x1 = f32::from_le_bytes(xpos_1);
    let y1 = f32::from_le_bytes(ypos_1);

    let x2 = f32::from_le_bytes(xpos_2);
    let y2 = f32::from_le_bytes(ypos_2);
    
    // TODO: add getting flip
    (x1,y1,x2,y2)
}

pub(crate) fn send_data(player: &mut Player, socket: &UdpSocket) {
    let player_xpos = player.physics.x().to_le_bytes(); 
    let player_ypos = player.physics.y().to_le_bytes();
    let flip: u8 = if player.is_looking_left { 1 } else { 0 };

    let portal_1_x: [u8; 4] = player.portal.portals[0].x().to_le_bytes();
    let portal_1_y: [u8; 4] = player.portal.portals[0].y().to_le_bytes();
    let portal_2_x: [u8; 4] = player.portal.portals[1].x().to_le_bytes();
    let portal_2_y: [u8; 4] = player.portal.portals[1].y().to_le_bytes();
    let buf = [player_xpos, player_ypos, flip, portal_1_x, portal_1_y, portal_2_x, portal_2_y].concat();
    socket.send(&buf);

}
