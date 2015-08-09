
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub struct MessageHeader {
  pub message_code: u16,
  pub date_of_message: u16,
  pub time_of_message: u32,
  pub length_of_message: u32,
  pub source_id: u16,
  pub destination_id: u16,
  pub number_of_blocks: u16
}

pub struct ProductDescriptionBlock {
  pub divider: u16,
  pub latitude_1k: i32,
  pub longitude_1k: i32,
  pub height: u16,
  pub product_code: u16,
  pub operational_mode: u16,
  pub volume_coverage_pattern: u16,
  pub sequence_number: u16,
  pub volume_scan_number: u16,
  pub volume_scan_date: u16,
  pub volume_scan_start_time: u32,
  pub product_generation_date: u16,
  pub product_generation_time: u32,
  pub product_dependent: [u16; 10],
  pub elevation_number: u16,
  pub data_level_threshold: [u16; 16],
  pub version: u8,
  pub spot_blank: u8,
  pub offset_to_symbology: u32,
  pub offset_to_graphic: u32,
  pub offset_to_tabular: u32,
}

pub struct ProductSymbologyBlock {
  pub divider: u16,
  pub block_id: u16,
  pub length_of_block: u32,
  pub number_of_layers: u16,
//  pub Layers: Vec<ProductSymbologyBlockLayer>,
}

pub struct ProductSymbologyBlockLayer {
  pub divider: u16,
  pub length_of_data_layer: u32,
}

pub struct RadialDataPacketHeader {
  pub packet_code: u16,
  pub index_of_first_range_bin: u16,
  pub number_of_range_bins: u16,
  pub i_center_of_sweep: u16,
  pub j_center_of_sweep: u16,
  pub scale_factor: u16,
  pub number_of_radials: u16,
}

#[derive(Debug)]
pub struct RadialDataPacketRadialRun {
  pub length: u8,
  pub color: u8,
}

pub struct RadialDataPacketRadialHeader {
  pub number_of_half_word_words: u16,
  pub radial_start_angle: u16,
  pub radial_angle_delta: u16,
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
    return ((buf[0] as u16) << 8) | (buf[1] as u16);
  }

  pub fn fetch_dword(&mut self) -> u32 {
    let buf = self.fetch_bytes(4);
    return ((buf[0] as u32) << 24) | ((buf[1] as u32) << 16) | ((buf[2] as u32) << 8) | (buf[3] as u32);
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
    MessageHeader {
      message_code: self.fetcher.fetch_word(),
      date_of_message: self.fetcher.fetch_word(),
      time_of_message: self.fetcher.fetch_dword(),
      length_of_message: self.fetcher.fetch_dword(),
      source_id: self.fetcher.fetch_word(),
      destination_id: self.fetcher.fetch_word(),
      number_of_blocks: self.fetcher.fetch_word(),
    }
  }

  pub fn decode_product_description_block(&mut self) -> ProductDescriptionBlock {
    let mut pdb = ProductDescriptionBlock {
      divider: self.fetcher.fetch_word(),
      latitude_1k: self.fetcher.fetch_dword() as i32,
      longitude_1k: self.fetcher.fetch_dword() as i32,
      height: self.fetcher.fetch_word(),
      product_code: self.fetcher.fetch_word(),
      operational_mode: self.fetcher.fetch_word(),
      volume_coverage_pattern: self.fetcher.fetch_word(),
      sequence_number: self.fetcher.fetch_word(),
      volume_scan_number: self.fetcher.fetch_word(),
      volume_scan_date: self.fetcher.fetch_word(),
      volume_scan_start_time: self.fetcher.fetch_dword(),
      product_generation_date: self.fetcher.fetch_word(),
      product_generation_time: self.fetcher.fetch_dword(),
      product_dependent: [
        self.fetcher.fetch_word(),
        self.fetcher.fetch_word(),
        0, 0, 0, 0, 0, 0, 0, 0
      ],
      elevation_number: self.fetcher.fetch_word(),
      data_level_threshold: [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      ],
      version: 0,
      spot_blank: 0,
      offset_to_symbology: 0,
      offset_to_graphic: 0,
      offset_to_tabular: 0,
    };

    pdb.product_dependent[2] = self.fetcher.fetch_word();

    pdb.data_level_threshold = [
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

    pdb.product_dependent[3] = self.fetcher.fetch_word();
    pdb.product_dependent[4] = self.fetcher.fetch_word();
    pdb.product_dependent[5] = self.fetcher.fetch_word();
    pdb.product_dependent[6] = self.fetcher.fetch_word();
    pdb.product_dependent[7] = self.fetcher.fetch_word();
    pdb.product_dependent[8] = self.fetcher.fetch_word();
    pdb.product_dependent[9] = self.fetcher.fetch_word();

    pdb.version = self.fetcher.fetch_byte();
    pdb.spot_blank = self.fetcher.fetch_byte();

    pdb.offset_to_symbology = self.fetcher.fetch_dword();
    pdb.offset_to_graphic = self.fetcher.fetch_dword();
    pdb.offset_to_tabular = self.fetcher.fetch_dword();

    return pdb;
  }

  pub fn decode_product_symbology_block(&mut self) -> ProductSymbologyBlock {
    ProductSymbologyBlock {
      divider: self.fetcher.fetch_word(),
      block_id: self.fetcher.fetch_word(),
      length_of_block: self.fetcher.fetch_dword(),
      number_of_layers: self.fetcher.fetch_word(),
    }
  }

  pub fn decode_product_symbology_block_layer(&mut self) -> ProductSymbologyBlockLayer {
    ProductSymbologyBlockLayer {
      divider: self.fetcher.fetch_word(),
      length_of_data_layer: self.fetcher.fetch_dword(),
    }
  }

  pub fn decode_radial_data_packet_header(&mut self) -> RadialDataPacketHeader {
    RadialDataPacketHeader {
      packet_code: self.fetcher.fetch_word(),
      index_of_first_range_bin: self.fetcher.fetch_word(),
      number_of_range_bins: self.fetcher.fetch_word(),
      i_center_of_sweep: self.fetcher.fetch_word(),
      j_center_of_sweep: self.fetcher.fetch_word(),
      scale_factor: self.fetcher.fetch_word(),
      number_of_radials: self.fetcher.fetch_word(),
    }
  }

  pub fn fetch_radial_data_packet_radial_run(&mut self) -> RadialDataPacketRadialRun {
    let word = self.fetcher.fetch_byte();
    return RadialDataPacketRadialRun {
      length: (word & 0xf0) >> 4,
      color: (word & 0x0f),
    }
  }

  pub fn fetch_radial_data_packet_radial_runs(&mut self, runs : u16) -> Vec<RadialDataPacketRadialRun> {
    let mut radial_data_packet_radial_runs = Vec::<RadialDataPacketRadialRun>::new();
    for _ in 0..(runs*2) {
      let word = self.fetcher.fetch_byte();
      radial_data_packet_radial_runs.push(RadialDataPacketRadialRun {
            length: (word & 0xf0) >> 4,
            color: (word & 0x0f),
          });
    }
    return radial_data_packet_radial_runs;
  }

  pub fn decode_radial_data_packet_radial_header(&mut self) -> RadialDataPacketRadialHeader {
    RadialDataPacketRadialHeader {
      number_of_half_word_words: self.fetcher.fetch_word(),
      radial_start_angle: self.fetcher.fetch_word(),
      radial_angle_delta: self.fetcher.fetch_word(),
    }
  }
}

