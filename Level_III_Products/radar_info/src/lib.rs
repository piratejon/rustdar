
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
  radar_fetcher: RadarFetcher,
}

pub struct RadarFetcher {
  buffered_reader: BufReader<File>,
  last_read_size: usize,
}

impl RadarFetcher {
  pub fn from_file(radar_file_name: &str) -> RadarFetcher {
    let fin = match File::open(radar_file_name) {
      Ok(fin) => fin,
      Err(..) => panic!("unable to open radar_file_name"),
    };

    return RadarFetcher {
      buffered_reader: BufReader::new(fin),
      last_read_size: 0,
    }
  }

  pub fn get_last_read_size(&self) -> usize {
    return self.last_read_size;
  }

  pub fn fetch_byte(&mut self) -> u8 {
    return self.fetch_bytes(1)[0];
  }

  pub fn fetch_word(&mut self) -> u16 {
    let buf = self.fetch_bytes(2);
    return (((buf[0] as u16) << 8) | (buf[1] as u16));
  }

  pub fn fetch_dword(&mut self) -> u32 {
    let buf = self.fetch_bytes(4);
    return (((buf[0] as u32) << 24) | ((buf[1] as u32) << 16) | ((buf[2] as u32) << 8) | (buf[3] as u32));
  }

  pub fn fetch_bytes(&mut self, bytes: usize) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    let mut byte_buf: [u8; 1] = [0xff];
    let mut bytes_read : usize;

    self.last_read_size = 0;

    loop {
      bytes_read = match self.buffered_reader.read(&mut byte_buf) {
        Ok(bytes_read) => bytes_read,
        Err(..) => panic!("failed to read a single byte gosh"),
      };

      if bytes_read == 1 {
        buf.push(byte_buf[0]);
      } else {
        break;
      }

      self.last_read_size += 1;

      if self.last_read_size == bytes {
        break;
      }
    }

    return buf;
  }
}

/*
impl RadarFileParser {
  pub fn from_fetcher(fetcher: RadarFetcher) -> RadarFileParser {
    RadarFileParser {
      radar_fetcher: fetcher
    }
  }

  pub fn decode_text_header(&self) -> &str {
    return match std::str::from_utf8(&self.raw[0..30]) {
      Ok(ret) => ret,
      Err(..) => panic!("Unable to decode text header"),
    };
  }

  /*
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
  */

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

  pub fn word_maker(&self, hi : u8, lo : u8) -> u16 {
    return ((hi as u16) << 8) | (lo as u16);
  }

  pub fn dword_maker(&self, hi : u16, lo : u16) -> u32 {
    return ((hi as u32) << 16) | (lo as u32);
  }
}
*/

