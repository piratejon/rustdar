
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub struct TextHeader {
  text_header: [u8; 30],
}

pub struct MessageHeader {
  packed_values: [u8; 9],
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
}

