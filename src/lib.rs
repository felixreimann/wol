//! Simple Wake On LAN tool.
//!
//! Send the magic packet either per IPv4 with `send_magic_packet_v4` or per IPv6 with
//! `send_magic_packet_v6`. Therefore, the MAC address of the remote system is required. Use
//! `parse_mac` to parse MAC address strings like "AB:CD:01:02:03:04".
use std::net::UdpSocket;
use std::net::{Ipv6Addr, Ipv4Addr};
use std::net::ToSocketAddrs;

use std::fmt;

/// Parses the MAC address from a given string.
///
/// #Example
///
/// ```
/// let mac = wol::parse_mac("AA:FF:B0:12:34:56".to_string());
/// assert_eq!(mac, Ok(vec![0xAA, 0xFF, 0xB0, 0x12, 0x34, 0x56]))
/// ```
pub fn parse_mac(mac: String) -> Result<Vec<u8>, ParseError> {
    let vec: Result<Vec<u8>, std::num::ParseIntError> = mac.split(':')
        .map(|s| u8::from_str_radix(s, 16))
        .collect();
    match vec {
        Err(e) => Err(ParseError::Number(e)),
        Ok(vec) => {
            if vec.len() == 6 {
                Ok(vec)
            } else {
                Err(ParseError::Length)
            }
        },
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Number(std::num::ParseIntError),
    Length,
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::Number(ref err) => err.description(),
            ParseError::Length => "illegal MAC address length",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            ParseError::Number(ref err) => Some(err),
            ParseError::Length => None,
        }
    }
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Number(ref err) => err.fmt(f),
            ParseError::Length => write!(f, "illegal MAC address length"),
        }
    }
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> ParseError {
        ParseError::Number(err)
    }
}

/// Sends the magic packet per UDP/IPv4.
///
/// #Example
///
/// ```
/// wol::send_magic_packet_v4(vec![0xAA, 0xFF, 0xB0, 0x12, 0x34, 0x56]);
/// ```
pub fn send_magic_packet_v4(mac: Vec<u8>) -> Result<(), &'static str> {
    let buf = create_payload(mac);
    let socket = create_socket((Ipv4Addr::new(0, 0, 0, 0), 0)).expect("Could not create socket.");
    socket.connect((Ipv4Addr::new(255, 255, 255, 255), 0)).expect("Could not create connection.");
    socket.send(&buf).expect("Could not send packet.");
    Ok(())
}

/// Sends the magic packet per UDP/IPv6.
///
/// #Example
///
/// ```
/// wol::send_magic_packet_v6(vec![0xAA, 0xFF, 0xB0, 0x12, 0x34, 0x56]);
/// ```
pub fn send_magic_packet_v6(mac: Vec<u8>) -> Result<(), &'static str> {
    let buf = create_payload(mac);
    let socket = create_socket((Ipv6Addr::new(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00), 0))
        .expect("Could not create socket.");
    socket.connect((Ipv6Addr::new(0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02), 0))
        .expect("Could not create connection.");
    socket.send(&buf).expect("Could not send packet.");
    Ok(())
}

/// Creates the payload for the magic packet.
fn create_payload(mac: Vec<u8>) -> [u8; 17 * 6] {
    let mut buf: [u8; 17 * 6] = [0xFF; 17 * 6];
    for x in 1..17 {
        for y in 0..6 {
            buf[x * 6 + y] = mac[y];
        }
    }
    buf
}

/// Creates the UdpSocket.
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
    fn test_create_socket_v4() {
        assert!(super::create_socket("127.0.0.1:0").is_ok());
    }

    #[test]
    fn test_create_socket_v6() {
        assert!(super::create_socket("[::1]:0").is_ok());
    }

    #[test]
    fn test_create_payload() {
        let payload = super::create_payload(vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
        let result = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
                      0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
                      0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
                      0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
                      0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
                      0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
                      0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
                      0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
                      0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
        assert_eq!(payload.len(), result.len());
        assert!(payload.iter()
            .zip(result.iter())
            .all(|(a, b)| a == b));
    }
}
