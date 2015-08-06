
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

pub struct ProductDescriptionBlock {
  Divider: u16,
  Latitude1K: u16,
  Longitude1K: u16,
  Height: u16,
  ProductCode: u16,
  OperationalMode: u16,
  VolumeCoveragePattern: u16,
  SequenceNumber: u16,
  VolumeScanNumber: u16,
  VolumeScanDate: u16,
  VolumeScanStartTime: u16,
  ProductGenerationDate: u16,
  ProductGenerationTime: u16,
  ProductDependent: [u16; 10],
  ElevationNumber: u16,
  DataLevelThreshold: [u16; 16],
  Version: u16,
  SpotBlank: u16,
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
  fetcher: RadarFetcher,
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

impl RadarFileParser {
  pub fn from_fetcher(radar_fetcher: RadarFetcher) -> RadarFileParser {
    RadarFileParser {
      fetcher: radar_fetcher
    }
  }

  pub fn decode_text_header(&mut self) -> &str {
    let text_header_bytes = self.fetcher.fetch_bytes(30);
    let new_string = String::new(text_header_bytes);
    return match std::str::from_utf8(text_header_bytes.to_string()) {
      Ok(ret) => ret,
      Err(..) => panic!("Unable to decode text header"),
    };
    // return "SDUS54 KOUN 030251\r\r\nN0RTLX\r\r\n";
  }

  pub fn decode_message_header(&mut self) -> MessageHeader {
    let offset = 30;
    MessageHeader {
      MessageCode: self.fetcher.fetch_word(),
      DateOfMessage: self.fetcher.fetch_word(),
      TimeOfMessage: self.fetcher.fetch_dword(),
      LengthOfMessage: self.fetcher.fetch_dword(),
      SourceID: self.fetcher.fetch_word(),
      DestinationID: self.fetcher.fetch_word(),
      NumberOfBlocks: self.fetcher.fetch_word(),
    }
  }

  pub fn decode_product_description_block(&mut self) -> ProductDescriptionBlock {
    ProductDescriptionBlock {
      Divider: self.fetcher.fetch_word(),
      Latitude1K: self.fetcher.fetch_word(),
      Longitude1K: self.fetcher.fetch_word(),
      Height: self.fetcher.fetch_word(),
      ProductCode: self.fetcher.fetch_word(),
      OperationalMode: self.fetcher.fetch_word(),
      VolumeCoveragePattern: self.fetcher.fetch_word(),
      SequenceNumber: self.fetcher.fetch_word(),
      VolumeScanNumber: self.fetcher.fetch_word(),
      VolumeScanDate: self.fetcher.fetch_word(),
      VolumeScanStartTime: self.fetcher.fetch_word(),
      ProductGenerationDate: self.fetcher.fetch_word(),
      ProductGenerationTime: self.fetcher.fetch_word(),
      ProductDependent: [
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word()
      ],
      ElevationNumber: self.fetcher.fetch_word(),
      DataLevelThreshold: [
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
      ],
      Version: self.fetcher.fetch_word(),
      SpotBlank: self.fetcher.fetch_word(),
    }
  }

  pub fn word_maker(&self, hi : u8, lo : u8) -> u16 {
    return ((hi as u16) << 8) | (lo as u16);
  }

  pub fn dword_maker(&self, hi : u16, lo : u16) -> u32 {
    return ((hi as u32) << 16) | (lo as u32);
  }
}

