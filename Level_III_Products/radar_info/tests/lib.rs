
extern crate radar_info;

#[test]
fn ri_read_a_file() {
  let mut radar_parser : radar_info::RadarFileParser = std::default::Default::default();

  assert_eq!(radar_parser.load_file("tests/sn.last"), 22178);
  assert_eq!(radar_parser.get_text_header(), "SDUS54 KOUN 030251\r\r\nN0RTLX\r\r\n");
}

