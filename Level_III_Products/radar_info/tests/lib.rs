
extern crate radar_info;

#[test]
fn ri_read_a_file() {
  let mut radar_parser : radar_info::RadarFileParser = std::default::Default::default();

  assert!(radar_parser.load_file("sn.last"));
}

