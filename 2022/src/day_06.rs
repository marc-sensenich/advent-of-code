use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

type Result<T> = std::result::Result<T, MissingStartOfPacketError>;

pub fn part_one(input: String) -> Result<i32> {
    let start_of_packet_detector: StartOfPacketDetector = StartOfPacketDetector::new(4);

    return match start_of_packet_detector.run(input) {
        Ok(start_of_packet) => Ok(start_of_packet.index),
        Err(err) => Err(err),
    };
}

pub fn part_two(input: String) -> Result<i32> {
    let start_of_packet_detector: StartOfPacketDetector = StartOfPacketDetector::new(14);

    return match start_of_packet_detector.run(input) {
        Ok(start_of_packet) => Ok(start_of_packet.index),
        Err(err) => Err(err),
    };
}

#[derive(Debug, Clone)]
pub struct MissingStartOfPacketError;

impl fmt::Display for MissingStartOfPacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no start of packet discovered")
    }
}

struct StartOfPacket {
    index: i32,
}

struct StartOfPacketDetector {
    start_of_packet_length: i32,
}

impl StartOfPacketDetector {
    fn new(start_of_packet_length: i32) -> StartOfPacketDetector {
        StartOfPacketDetector {
            start_of_packet_length,
        }
    }

    fn run(&self, datastream: String) -> Result<StartOfPacket> {
        let len_comparator: usize = self.start_of_packet_length.try_into().unwrap();
        let mut signal_buffer: VecDeque<char> = VecDeque::with_capacity(len_comparator);
        let mut index: i32 = 0;

        for char in datastream.chars() {
            signal_buffer.push_back(char);
            index = index + 1;

            if signal_buffer.len() == len_comparator {
                let signal_set: HashSet<&char> = HashSet::from_iter(signal_buffer.iter());
                if signal_set.len() == len_comparator {
                    return Ok(StartOfPacket { index });
                }
                signal_buffer.pop_front();
            }
        }

        Err(MissingStartOfPacketError)
    }
}
