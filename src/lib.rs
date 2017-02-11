use std::net::UdpSocket;
use std::net::{Ipv6Addr, Ipv4Addr};
use std::net::ToSocketAddrs;
use std::fmt;

struct Payload {
    data: [u8; 102]
}

impl fmt::Debug for Payload {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.data[..].fmt(formatter)
    }
}

pub fn parse_mac(macstr: String) -> Result<Vec<u8>, &'static str> {
    let vec: Vec<u8> = macstr.split(':')
        .map(|s| u8::from_str_radix(s, 16).expect(&format!("Not a hex number: {}", s)))
        .collect();
    if vec.len() == 6 {
        Ok(vec)
    } else {
        Err("Illegal MAC address length.")
    }
}

pub fn send_magic_packet_v4(mac: Vec<u8>) -> Result<(), &'static str> {
    let buf = create_payload(mac);
    let socket = create_socket((Ipv4Addr::new(0, 0, 0, 0), 0)).expect("Could not create socket.");
    socket.connect((Ipv4Addr::new(255, 255, 255, 255), 0)).expect("Could not create connection.");
    socket.send(&buf).expect("Could not send packet.");
    Ok(())
}

pub fn send_magic_packet_v6(mac: Vec<u8>) -> Result<(), &'static str> {
    let buf = create_payload(mac);
    let socket = create_socket((Ipv6Addr::new(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00), 0))
        .expect("Could not create socket.");
    socket.connect((Ipv6Addr::new(0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02), 0))
        .expect("Could not create connection.");
    socket.send(&buf).expect("Could not send packet.");
    Ok(())
}

fn create_payload(mac: Vec<u8>) -> [u8; 17 * 6] {
    let mut buf: [u8; 17 * 6] = [0xFF; 17 * 6];
    for x in 1..17 {
        for y in 0..6 {
            buf[x * 6 + y] = mac[y];
        }
    }
    buf
}

fn create_socket<A: ToSocketAddrs>(address: A) -> Result<UdpSocket, std::io::Error> {
    let socket = UdpSocket::bind(address).unwrap();
    socket.set_broadcast(true)?;
    Ok(socket)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_mac() {
        assert_eq!(super::parse_mac("FF:FF:FF:FF:FF:FF".to_string()),
        Ok(vec![255, 255, 255, 255, 255, 255]));
        assert_eq!(super::parse_mac("00:00:00:00:00:00".to_string()),
        Ok(vec![0, 0, 0, 0, 0, 0]));
    }

    #[test]
    fn test_create_socket() {
        assert!(super::create_socket("127.0.0.1:0").is_ok());
        assert!(super::create_socket("[::1]:0").is_ok());
    }

    #[test]
    fn test_create_payload() {
        let payload = super::Payload { data : super::create_payload(vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]) };
        let result = super::Payload { data: [0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05] };
        assert_eq!(payload.data.len(), result.data.len());
        println!("");
        println!("payload {:?}", payload);
        println!("result  {:?}", result);
        assert!(payload.data.iter().zip(
        result.data.iter()).all(|(a, b)| a == b));
    }
}
