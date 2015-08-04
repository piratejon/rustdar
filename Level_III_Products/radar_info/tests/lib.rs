
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
}

