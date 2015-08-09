
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
  pub Divider: u16,
  pub Latitude1K: i32,
  pub Longitude1K: i32,
  pub Height: u16,
  pub ProductCode: u16,
  pub OperationalMode: u16,
  pub VolumeCoveragePattern: u16,
  pub SequenceNumber: u16,
  pub VolumeScanNumber: u16,
  pub VolumeScanDate: u16,
  pub VolumeScanStartTime: u32,
  pub ProductGenerationDate: u16,
  pub ProductGenerationTime: u32,
  pub ProductDependent: [u16; 10],
  pub ElevationNumber: u16,
  pub DataLevelThreshold: [u16; 16],
  pub Version: u8,
  pub SpotBlank: u8,
  pub OffsetToSymbology: u32,
  pub OffsetToGraphic: u32,
  pub OffsetToTabular: u32,
}

pub struct ProductSymbologyBlock {
  pub Divider: u16,
  pub BlockId: u16,
  pub LengthOfBlock: u32,
  pub NumberOfLayers: u16,
//  pub Layers: Vec<ProductSymbologyBlockLayer>,
}

pub struct ProductSymbologyBlockLayer {
  pub Divider: u16,
  pub LengthOfDataLayer: u32,
}

pub struct RadialDataPacketHeader {
  pub PacketCode: u16,
  pub IndexOfFirstRangeBin: u16,
  pub NumberOfRangeBins: u16,
  pub ICenterOfSweep: u16,
  pub JCenterOfSweep: u16,
  pub ScaleFactor: u16,
  pub NumberOfRadials: u16,
}

#[derive(Debug)]
pub struct RadialDataPacketRadialRun {
  pub Length: u8,
  pub Color: u8,
}

pub struct RadialDataPacketRadialHeader {
  pub NumberOfHalfWords: u16,
  pub RadialStartAngle: u16,
  pub RadialAngleDelta: u16,
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

  pub fn word_maker(&self, hi : u8, lo : u8) -> u16 {
    return ((hi as u16) << 8) | (lo as u16);
  }

  pub fn dword_maker(&self, hi : u16, lo : u16) -> u32 {
    return ((hi as u32) << 16) | (lo as u32);
  }
}

impl RadarFileParser {
  pub fn from_fetcher(radar_fetcher: RadarFetcher) -> RadarFileParser {
    RadarFileParser {
      fetcher: radar_fetcher
    }
  }

  pub fn decode_text_header(&mut self) -> String {
    let text_header_bytes = self.fetcher.fetch_bytes(30);
    return match String::from_utf8(text_header_bytes) {
      Ok(ret) => ret,
      Err(..) => panic!("Unable to decode text header"),
    };
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
    let mut pdb = ProductDescriptionBlock {
      Divider: self.fetcher.fetch_word(),
      Latitude1K: self.fetcher.fetch_dword() as i32,
      Longitude1K: self.fetcher.fetch_dword() as i32,
      Height: self.fetcher.fetch_word(),
      ProductCode: self.fetcher.fetch_word(),
      OperationalMode: self.fetcher.fetch_word(),
      VolumeCoveragePattern: self.fetcher.fetch_word(),
      SequenceNumber: self.fetcher.fetch_word(),
      VolumeScanNumber: self.fetcher.fetch_word(),
      VolumeScanDate: self.fetcher.fetch_word(),
      VolumeScanStartTime: self.fetcher.fetch_dword(),
      ProductGenerationDate: self.fetcher.fetch_word(),
      ProductGenerationTime: self.fetcher.fetch_dword(),
      ProductDependent: [
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        0, 0, 0, 0, 0, 0, 0, 0
      ],
      ElevationNumber: self.fetcher.fetch_word(),
      DataLevelThreshold: [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      ],
      Version: 0,
      SpotBlank: 0,
      OffsetToSymbology: 0,
      OffsetToGraphic: 0,
      OffsetToTabular: 0,
    };

    pdb.ProductDependent[2] = self.fetcher.fetch_word();

    pdb.DataLevelThreshold = [
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
    ];

    pdb.ProductDependent[3] = self.fetcher.fetch_word();
    pdb.ProductDependent[4] = self.fetcher.fetch_word();
    pdb.ProductDependent[5] = self.fetcher.fetch_word();
    pdb.ProductDependent[6] = self.fetcher.fetch_word();
    pdb.ProductDependent[7] = self.fetcher.fetch_word();
    pdb.ProductDependent[8] = self.fetcher.fetch_word();
    pdb.ProductDependent[9] = self.fetcher.fetch_word();

    pdb.Version = self.fetcher.fetch_byte();
    pdb.SpotBlank = self.fetcher.fetch_byte();

    pdb.OffsetToSymbology = self.fetcher.fetch_dword();
    pdb.OffsetToGraphic = self.fetcher.fetch_dword();
    pdb.OffsetToTabular = self.fetcher.fetch_dword();

    return pdb;
  }

  pub fn decode_product_symbology_block(&mut self) -> ProductSymbologyBlock {
    ProductSymbologyBlock {
      Divider: self.fetcher.fetch_word(),
      BlockId: self.fetcher.fetch_word(),
      LengthOfBlock: self.fetcher.fetch_dword(),
      NumberOfLayers: self.fetcher.fetch_word(),
    }
  }

  pub fn decode_product_symbology_block_layer(&mut self) -> ProductSymbologyBlockLayer {
    ProductSymbologyBlockLayer {
      Divider: self.fetcher.fetch_word(),
      LengthOfDataLayer: self.fetcher.fetch_dword(),
    }
  }

  pub fn decode_radial_data_packet_header(&mut self) -> RadialDataPacketHeader {
    RadialDataPacketHeader {
      PacketCode: self.fetcher.fetch_word(),
      IndexOfFirstRangeBin: self.fetcher.fetch_word(),
      NumberOfRangeBins: self.fetcher.fetch_word(),
      ICenterOfSweep: self.fetcher.fetch_word(),
      JCenterOfSweep: self.fetcher.fetch_word(),
      ScaleFactor: self.fetcher.fetch_word(),
      NumberOfRadials: self.fetcher.fetch_word(),
    }
  }

  pub fn fetch_radial_data_packet_radial_run(&mut self) -> RadialDataPacketRadialRun {
    let word = self.fetcher.fetch_byte();
    return RadialDataPacketRadialRun {
      Length: (word & 0xf0) >> 4,
      Color: (word & 0x0f),
    }
  }

  pub fn fetch_radial_data_packet_radial_runs(&mut self, runs : u16) -> Vec<RadialDataPacketRadialRun> {
    let mut radial_data_packet_radial_runs = Vec::<RadialDataPacketRadialRun>::new();
    for i in 0..(runs*2) {
      let word = self.fetcher.fetch_byte();
      radial_data_packet_radial_runs.push(RadialDataPacketRadialRun {
            Length: (word & 0xf0) >> 4,
            Color: (word & 0x0f),
          });
    }
    return radial_data_packet_radial_runs;
  }

  pub fn decode_radial_data_packet_radial_header(&mut self) -> RadialDataPacketRadialHeader {
    RadialDataPacketRadialHeader {
      NumberOfHalfWords: self.fetcher.fetch_word(),
      RadialStartAngle: self.fetcher.fetch_word(),
      RadialAngleDelta: self.fetcher.fetch_word(),
    }
  }
}

