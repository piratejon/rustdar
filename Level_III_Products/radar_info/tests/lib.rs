
extern crate radar_info;

#[test]
fn ri_helper_word_builder() {
  let radar_parser : radar_info::RadarFileParser = std::default::Default::default();

  assert_eq!(radar_parser.word_maker(0, 0), 0);
  assert_eq!(radar_parser.word_maker(0, 1), 1);
  assert_eq!(radar_parser.word_maker(1, 0), 0x100);
  assert_eq!(radar_parser.word_maker(0x41, 0x0b), 0x410b);

  assert_eq!(radar_parser.dword_maker(0 as u16, 0 as u16), 0);
  assert_eq!(radar_parser.dword_maker(0, 1), 1);
  assert_eq!(radar_parser.dword_maker(1, 0), 0x10000);
  assert_eq!(radar_parser.dword_maker(0x41, 0x0b), 0x41000b);
}

#[test]
fn ri_read_a_file() {
  let mut radar_parser : radar_info::RadarFileParser = std::default::Default::default();

  assert_eq!(radar_parser.load_file("tests/sn.last"), 22178);
  assert_eq!(radar_parser.decode_text_header(), "SDUS54 KOUN 030251\r\r\nN0RTLX\r\r\n");

  let message_header = radar_parser.decode_message_header();
  assert_eq!(message_header.MessageCode, 19);
  assert_eq!(message_header.DateOfMessage, 16651);
  assert_eq!(message_header.TimeOfMessage, 10324);
  assert_eq!(message_header.LengthOfMessage, 22148);
  assert_eq!(message_header.SourceID, 1);
  assert_eq!(message_header.DestinationID, 0);
  assert_eq!(message_header.NumberOfBlocks, 3);

  let product_description_block = radar_parser.decode_product_description_block();
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

