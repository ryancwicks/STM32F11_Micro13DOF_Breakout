//! # IMU Reader Communication Protocol
//! 
//! This library defines the packets and parsers used by both the uC and the topside program to send data from the IMU to 
//! the topside and (eventually) to send control signals down to the imu.
//! 
//! The serialization relies on Serde and postcard. The Packet Type enum contains the actual sensor data and control commands, while a wrapper Packet struct 
//! is used to packetize the PacketType data.
//! 
//! | 
//! 
//! Here is an example of using the library to packetize and then recover a set of IMU Accelerometer data.
//! 
//! ```
//! 
//! ```
//! 

#![no_std]
use serde::{Serialize, Deserialize};
use postcard::{from_bytes, to_vec};
use heapless::{Vec, consts::*};

const DELIMITER: [u8; 2] = ['2' as u8, 'g' as u8];

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum PacketType {
    AccelData {ax: f32, ay: f32, az: f32},
    GyroData {wx: f32, wy: f32, ax:f32},
    MagnetometerData {mx: f32, my: f32, mz: f32},
    EnvironmentalData {pressure: f32, humidity: f32, temp: f32},
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Packet {
    delimiter: [u8; 2],
    time: u64,
    data: PacketType
}

impl Packet {
    /// Create a new packet from the time in uS and PacketType
    pub fn new( time: u64, data: PacketType) -> Packet {
        Packet{
            delimiter: DELIMITER,
            time: time,
            data: data
        } 
    }

    /// Takes in a raw vector of bytes and tries to convert it to a
    /// packet. Returns an option, none if there is a parse error or incomplete packet.
    /// The data should be cleared if there is a None return.
    pub fn parse_data (data: &[u8]) -> Option<Packet> {
        let index_of_first_delimiter = match data.iter().position(|&x| x == DELIMITER[0]){
            Some(num) => num,
            None => return None
        };
        let index_of_second_delimiter = match data.iter().position(|&x| x == DELIMITER[1]) {
            Some(num) => num,
            None => return None
        };

        if index_of_first_delimiter == index_of_second_delimiter-1 {
            match from_bytes(&data[index_of_first_delimiter..]) {
                Ok(packet) => packet,
                Err(_) => None
            }
        } else {
            None
        }
    }

    /// Serialize this packet into a heapless vector (max size 32 bytes.)
    /// Returns a postcard::Result that either contains a heapless vector of max size
    /// 32 bytes or a postcard::Error.
    pub fn serialize(&self) -> postcard::Result<Vec <u8, U32>> {
        to_vec(self)
    }


}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
