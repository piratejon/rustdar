
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
  pub decoder: RadarFileDecoder,
}

pub struct RadarFileDecoder {
  pub x: u32,
  // pub fetcher: RadarFetcher,
}

pub struct RadarFetcher {
  buffered_reader: BufReader<File>,
  pub last_read_size: usize,
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

      if buf.len() == bytes {
        break;
      }
    }

    self.last_read_size = buf.len();

    return buf;
  }

  pub fn fetch_remaining_bytes(&mut self) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    self.last_read_size = match self.buffered_reader.read_to_end(&mut buf) {
      Ok(bytes_read) => bytes_read,
      Err(..) => panic!("failed to read remaining bytes"),
    };
    return buf;
  }
}

impl RadarFileDecoder {
  pub fn decode_text_header(&self, fetcher: &mut RadarFetcher) -> String {
    let text_header_bytes = fetcher.fetch_bytes(30);
    return match String::from_utf8(text_header_bytes) {
      Ok(ret) => ret,
      Err(..) => panic!("Unable to decode text header"),
    };
  }

  pub fn decode_message_header(&self, fetcher: &mut RadarFetcher) -> MessageHeader {
    MessageHeader {
      message_code: fetcher.fetch_word(),
      date_of_message: fetcher.fetch_word(),
      time_of_message: fetcher.fetch_dword(),
      length_of_message: fetcher.fetch_dword(),
      source_id: fetcher.fetch_word(),
      destination_id: fetcher.fetch_word(),
      number_of_blocks: fetcher.fetch_word(),
    }
  }

  pub fn decode_product_description_block(&self, fetcher: &mut RadarFetcher) -> ProductDescriptionBlock {
    let mut pdb = ProductDescriptionBlock {
      divider: fetcher.fetch_word(),
      latitude_1k: fetcher.fetch_dword() as i32,
      longitude_1k: fetcher.fetch_dword() as i32,
      height: fetcher.fetch_word(),
      product_code: fetcher.fetch_word(),
      operational_mode: fetcher.fetch_word(),
      volume_coverage_pattern: fetcher.fetch_word(),
      sequence_number: fetcher.fetch_word(),
      volume_scan_number: fetcher.fetch_word(),
      volume_scan_date: fetcher.fetch_word(),
      volume_scan_start_time: fetcher.fetch_dword(),
      product_generation_date: fetcher.fetch_word(),
      product_generation_time: fetcher.fetch_dword(),
      product_dependent: [
        fetcher.fetch_word(),
        fetcher.fetch_word(),
        0, 0, 0, 0, 0, 0, 0, 0
      ],
      elevation_number: fetcher.fetch_word(),
      data_level_threshold: [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      ],
      version: 0,
      spot_blank: 0,
      offset_to_symbology: 0,
      offset_to_graphic: 0,
      offset_to_tabular: 0,
    };

    pdb.product_dependent[2] = fetcher.fetch_word();

    pdb.data_level_threshold = [
      fetcher.fetch_word(),
      fetcher.fetch_word(),
      fetcher.fetch_word(),
      fetcher.fetch_word(),
      fetcher.fetch_word(),
      fetcher.fetch_word(),
      fetcher.fetch_word(),
      fetcher.fetch_word(),
      fetcher.fetch_word(),
      fetcher.fetch_word(),
      fetcher.fetch_word(),
      fetcher.fetch_word(),
      fetcher.fetch_word(),
      fetcher.fetch_word(),
      fetcher.fetch_word(),
      fetcher.fetch_word(),
    ];

    pdb.product_dependent[3] = fetcher.fetch_word();
    pdb.product_dependent[4] = fetcher.fetch_word();
    pdb.product_dependent[5] = fetcher.fetch_word();
    pdb.product_dependent[6] = fetcher.fetch_word();
    pdb.product_dependent[7] = fetcher.fetch_word();
    pdb.product_dependent[8] = fetcher.fetch_word();
    pdb.product_dependent[9] = fetcher.fetch_word();

    pdb.version = fetcher.fetch_byte();
    pdb.spot_blank = fetcher.fetch_byte();

    pdb.offset_to_symbology = fetcher.fetch_dword();
    pdb.offset_to_graphic = fetcher.fetch_dword();
    pdb.offset_to_tabular = fetcher.fetch_dword();

    return pdb;
  }

  pub fn decode_product_symbology_block(&self, fetcher: &mut RadarFetcher) -> ProductSymbologyBlock {
    ProductSymbologyBlock {
      divider: fetcher.fetch_word(),
      block_id: fetcher.fetch_word(),
      length_of_block: fetcher.fetch_dword(),
      number_of_layers: fetcher.fetch_word(),
    }
  }

  pub fn decode_product_symbology_block_layer(&self, fetcher: &mut RadarFetcher) -> ProductSymbologyBlockLayer {
    ProductSymbologyBlockLayer {
      divider: fetcher.fetch_word(),
      length_of_data_layer: fetcher.fetch_dword(),
    }
  }

  pub fn decode_radial_data_packet_header(&self, fetcher: &mut RadarFetcher) -> RadialDataPacketHeader {
    RadialDataPacketHeader {
      packet_code: fetcher.fetch_word(),
      index_of_first_range_bin: fetcher.fetch_word(),
      number_of_range_bins: fetcher.fetch_word(),
      i_center_of_sweep: fetcher.fetch_word(),
      j_center_of_sweep: fetcher.fetch_word(),
      scale_factor: fetcher.fetch_word(),
      number_of_radials: fetcher.fetch_word(),
    }
  }

  pub fn fetch_radial_data_packet_radial_run(&self, fetcher: &mut RadarFetcher) -> RadialDataPacketRadialRun {
    let word = fetcher.fetch_byte();
    return RadialDataPacketRadialRun {
      length: (word & 0xf0) >> 4,
      color: (word & 0x0f),
    }
  }

  pub fn fetch_radial_data_packet_radial_runs(&self, fetcher: &mut RadarFetcher, runs : u16) -> Vec<RadialDataPacketRadialRun> {
    let mut radial_data_packet_radial_runs = Vec::<RadialDataPacketRadialRun>::new();
    for _ in 0..(runs*2) {
      radial_data_packet_radial_runs.push(self.fetch_radial_data_packet_radial_run(fetcher));
    }
    return radial_data_packet_radial_runs;
  }

  pub fn decode_radial_data_packet_radial_header(&self, fetcher: &mut RadarFetcher) -> RadialDataPacketRadialHeader {
    RadialDataPacketRadialHeader {
      number_of_half_word_words: fetcher.fetch_word(),
      radial_start_angle: fetcher.fetch_word(),
      radial_angle_delta: fetcher.fetch_word(),
    }
  }
}

impl RadarFileParser {
  pub fn from_decoder(&mut self, radar_decoder: RadarFileDecoder) -> RadarFileParser {
    RadarFileParser {
      decoder: radar_decoder
    }
  }
}

