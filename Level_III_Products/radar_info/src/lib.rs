
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub struct MessageHeader {
  pub MessageCode: u16,
  pub DateOfMessage: u16,
  pub TimeOfMessage: u32,
  pub LengthOfMessage: u32,
  pub SourceID: u16,
  pub DestinationID: u16,
  pub NumberOfBlocks: u16
}

pub struct ProductDescription {
  packed_values: [u8; 51],
}

pub struct ProductSymbologyBlock {
  packed_values: [u8; 5],
}

pub struct RadialPacketHeader {
  packed_values: [u8; 7],
}

pub struct RasterPacketHeader {
  packed_values: [u8; 11],
}

pub struct RadarFileParser {
  raw: Vec<u8>
}

impl std::default::Default for RadarFileParser {
  fn default() -> RadarFileParser {
    RadarFileParser {
      raw: Vec::new(),
    }
  }
}

impl RadarFileParser {
  pub fn load_file(&mut self, radar_file_name : &str) -> usize {
    let fin = match File::open(radar_file_name) {
      Ok(fin) => fin,
      Err(..) => panic!("unable to open radar_file_name"),
    };

    let mut bfrdr = BufReader::new(fin);

    let result = match bfrdr.read_to_end(&mut self.raw) {
      Ok(result) => result,
      Err(..) => panic!("unable to read to end"),
    };

    return self.raw.len();
  }

  pub fn decode_text_header(&self) -> &str {
    return match std::str::from_utf8(&self.raw[0..30]) {
      Ok(ret) => ret,
      Err(..) => panic!("Unable to decode text header"),
    };
  }

  pub fn decode_message_header(&self) -> MessageHeader {
    let message_header_offset = 30;
    MessageHeader {
      MessageCode: self.word_maker(self.raw[message_header_offset + 0], self.raw[message_header_offset + 1]),
      DateOfMessage: self.word_maker(self.raw[message_header_offset + 2], self.raw[message_header_offset + 3]),
      TimeOfMessage: self.dword_maker(self.word_maker(self.raw[message_header_offset + 4], self.raw[message_header_offset + 5]), self.word_maker(self.raw[message_header_offset + 6], self.raw[message_header_offset + 7])),
      LengthOfMessage: self.dword_maker(self.word_maker(self.raw[message_header_offset + 8], self.raw[message_header_offset + 9]), self.word_maker(self.raw[message_header_offset + 10], self.raw[message_header_offset + 11])),
      SourceID: self.word_maker(self.raw[message_header_offset + 12], self.raw[message_header_offset + 13]),
      DestinationID: self.word_maker(self.raw[message_header_offset + 14], self.raw[message_header_offset + 15]),
      NumberOfBlocks: self.word_maker(self.raw[message_header_offset + 16], self.raw[message_header_offset + 17]),
    }
  }

  pub fn word_maker(&self, hi : u8, lo : u8) -> u16 {
    return ((hi as u16) << 8) | (lo as u16);
  }

  pub fn dword_maker(&self, hi : u16, lo : u16) -> u32 {
    return ((hi as u32) << 16) | (lo as u32);
  }
}

