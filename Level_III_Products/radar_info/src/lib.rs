
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

pub struct RadarFetcher {
  radar_reader: Option<BufReader>
}

impl std::default::Default for RadarFetcher {
  fn default() -> RadarFetcher {
    radar_reader: None
  }
}

impl RadarFetcher {
  pub fn open_file(&mut self, radar_file_name : &str) -> bool {
    let fin = match File::open(radar_file_name) {
      Ok(fin) => fin,
      Err(..) => panic!("unable to open radar file"),
    }

    self.radar_reader = BufReader::new(fin);

    return true;
  }
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
    let offset = 30;
    MessageHeader {
      MessageCode: self.word_maker(self.raw[offset + 0], self.raw[offset + 1]),
      DateOfMessage: self.word_maker(self.raw[offset + 2], self.raw[offset + 3]),
      TimeOfMessage: self.dword_maker(self.word_maker(self.raw[offset + 4], self.raw[offset + 5]), self.word_maker(self.raw[offset + 6], self.raw[offset + 7])),
      LengthOfMessage: self.dword_maker(self.word_maker(self.raw[offset + 8], self.raw[offset + 9]), self.word_maker(self.raw[offset + 10], self.raw[offset + 11])),
      SourceID: self.word_maker(self.raw[offset + 12], self.raw[offset + 13]),
      DestinationID: self.word_maker(self.raw[offset + 14], self.raw[offset + 15]),
      NumberOfBlocks: self.word_maker(self.raw[offset + 16], self.raw[offset + 17]),
    }
  }

  /*
  pub fn decode_product_description_block(&self) -> ProductDescriptionBlock {
    let offset = 47;
    ProductDescriptionBlock {
    }
    assert_eq!(product_description_block.Divider, 0xffff);
    assert_eq!(product_description_block.Latitude1K, 35333);
    assert_eq!(product_description_block.Longitude1K, -97278);
    assert_eq!(product_description_block.Height, 1277);
    assert_eq!(product_description_block.ProductCode, 19);
    assert_eq!(product_description_block.OperationalMode, 2);
    assert_eq!(product_description_block.VolumeCoveragePattern, 212);
    assert_eq!(product_description_block.SequenceNumber, 3380);
    assert_eq!(product_description_block.VolumeScanNumber, 0x2e);
    assert_eq!(product_description_block.VolumeScanDate, 16651);
    assert_eq!(product_description_block.VolumeScanStartTime, 10303);
    assert_eq!(product_description_block.ProductGenerationDate, 16651);
    assert_eq!(product_description_block.ProductGenerationTime, 10323);
    assert_eq!(product_description_block.ProductDependent[0], 0);
    assert_eq!(product_description_block.ProductDependent[1], 0);
    assert_eq!(product_description_block.ProductDependent[2], 5);
    assert_eq!(product_description_block.ProductDependent[3], 0x003b);
    assert_eq!(product_description_block.ProductDependent[4], 0);
    assert_eq!(product_description_block.ProductDependent[5], 0);
    assert_eq!(product_description_block.ProductDependent[6], 0);
    assert_eq!(product_description_block.ProductDependent[7], 0xc22f);
    assert_eq!(product_description_block.ProductDependent[8], 0xa50c);
    assert_eq!(product_description_block.ProductDependent[9], 0);
    assert_eq!(product_description_block.ElevationNumber, 1);
    assert_eq!(product_description_block.DataLevelThreshold[0], 0x8002);
    assert_eq!(product_description_block.DataLevelThreshold[1], 0x0005);
    assert_eq!(product_description_block.DataLevelThreshold[2], 0x000a);
    assert_eq!(product_description_block.DataLevelThreshold[3], 0x000f);
    assert_eq!(product_description_block.DataLevelThreshold[4], 0x0014);
    assert_eq!(product_description_block.DataLevelThreshold[5], 0x0019);
    assert_eq!(product_description_block.DataLevelThreshold[6], 0x001e);
    assert_eq!(product_description_block.DataLevelThreshold[7], 0x0023);
    assert_eq!(product_description_block.DataLevelThreshold[8], 0x0028);
    assert_eq!(product_description_block.DataLevelThreshold[9], 0x002d);
    assert_eq!(product_description_block.DataLevelThreshold[10], 0x0032);
    assert_eq!(product_description_block.DataLevelThreshold[11], 0x0037);
    assert_eq!(product_description_block.DataLevelThreshold[12], 0x003c);
    assert_eq!(product_description_block.DataLevelThreshold[13], 0x0041);
    assert_eq!(product_description_block.DataLevelThreshold[14], 0x0046);
    assert_eq!(product_description_block.DataLevelThreshold[15], 0x004b);
    assert_eq!(product_description_block.Version, 0);
    assert_eq!(product_description_block.SpotBlank, 0);
  }
  */

  pub fn word_maker(&self, hi : u8, lo : u8) -> u16 {
    return ((hi as u16) << 8) | (lo as u16);
  }

  pub fn dword_maker(&self, hi : u16, lo : u16) -> u32 {
    return ((hi as u32) << 16) | (lo as u32);
  }
}

