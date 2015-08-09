
extern crate radar_info;

#[test]
fn ri_helper_word_builder() {
  let radar_parser = radar_info::RadarFetcher::from_file("tests/sn.last");

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
fn ri_radar_fetcher() {
  let mut radar_fetcher = radar_info::RadarFetcher::from_file("tests/sn.last");

  // "SDUS54 KOUN 030251\r\r\nN0RTLX\r\r\n", 22178
  assert_eq!(radar_fetcher.get_last_read_size(), 0);
  // assert_eq!(radar_fetcher.fetch_bytes(30), "SDUS54 KOUN 030251\r\r\nN0RTLX\r\r\n"); // do the hard math
  assert_eq!(radar_fetcher.get_last_read_size(), 0);
  assert_eq!(radar_fetcher.fetch_byte(), 'S' as u8);
  assert_eq!(radar_fetcher.get_last_read_size(), 1);
  assert_eq!(radar_fetcher.fetch_byte(), 'D' as u8);
  assert_eq!(radar_fetcher.get_last_read_size(), 1);
  assert_eq!(radar_fetcher.fetch_word(), 0x5553);
  assert_eq!(radar_fetcher.get_last_read_size(), 2);
  assert_eq!(radar_fetcher.fetch_dword(), 0x3534204B);
  assert_eq!(radar_fetcher.get_last_read_size(), 4);

  assert_eq!(radar_fetcher.fetch_bytes(5), vec![0x4F, 0x55, 0x4E, 0x20, 0x30]);
}

#[test]
fn ri_read_a_file() {
  let mut radar_parser = radar_info::RadarFileParser::from_fetcher(
      radar_info::RadarFetcher::from_file("tests/sn.last")
      );

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
  assert_eq!(product_description_block.OffsetToSymbology, 60);
  assert_eq!(product_description_block.OffsetToGraphic, 0);
  assert_eq!(product_description_block.OffsetToTabular, 0);

  let product_symbology_block = radar_parser.decode_product_symbology_block();
  assert_eq!(product_symbology_block.Divider, 0xffff);
  assert_eq!(product_symbology_block.BlockId, 1);
  assert_eq!(product_symbology_block.LengthOfBlock, 22028);
  assert_eq!(product_symbology_block.NumberOfLayers, 1);

  let product_symbology_block_layer = radar_parser.decode_product_symbology_block_layer();
  assert_eq!(product_symbology_block_layer.Divider, 0xffff);
  assert_eq!(product_symbology_block_layer.LengthOfDataLayer, 22012);

  let radial_data_packet_header = radar_parser.decode_radial_data_packet_header();
  assert_eq!(radial_data_packet_header.PacketCode, 0xaf1f);
  assert_eq!(radial_data_packet_header.IndexOfFirstRangeBin, 0);
  assert_eq!(radial_data_packet_header.NumberOfRangeBins, 230);
  assert_eq!(radial_data_packet_header.ICenterOfSweep, 256);
  assert_eq!(radial_data_packet_header.JCenterOfSweep, 280);
  assert_eq!(radial_data_packet_header.ScaleFactor, 999);
  assert_eq!(radial_data_packet_header.NumberOfRadials, 360);

  let radial_data_packet_radial_header = radar_parser.decode_radial_data_packet_radial_header();
  assert_eq!(radial_data_packet_radial_header.NumberOfHalfWords, 30);
  assert_eq!(radial_data_packet_radial_header.RadialStartAngle, 2680);
  assert_eq!(radial_data_packet_radial_header.RadialAngleDelta, 10);

  let radial_data_packet_radial_run = radar_parser.fetch_radial_data_packet_radial_run();
  assert_eq!(radial_data_packet_radial_run.Length, 2);
  assert_eq!(radial_data_packet_radial_run.Color, 0);

  let radial_data_packet_radial_run = radar_parser.fetch_radial_data_packet_radial_run();
  assert_eq!(radial_data_packet_radial_run.Length, 3);
  assert_eq!(radial_data_packet_radial_run.Color, 2);

  let radial_data_packet_radial_runs = radar_parser.fetch_radial_data_packet_radial_runs(radial_data_packet_radial_header.NumberOfHalfWords - 1);

  let radial_data_packet_radial_runs_control = vec![
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 3 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 5, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 4, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 4 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 2, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 5, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 3, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 3, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 2, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 4, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 3, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 2, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 4, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 2, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 2, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 6, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 3, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 4 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 0 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 3 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 4 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 2, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 3, Color: 0 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 2, Color: 0 },
    radar_info::RadialDataPacketRadialRun { Length: 4, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 0 },
    radar_info::RadialDataPacketRadialRun { Length: 2, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 0xd, Color: 0 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 0xf, Color: 0 },
    radar_info::RadialDataPacketRadialRun { Length: 0xf, Color: 0 },
    radar_info::RadialDataPacketRadialRun { Length: 0xf, Color: 0 },
    radar_info::RadialDataPacketRadialRun { Length: 0xf, Color: 0 },
    radar_info::RadialDataPacketRadialRun { Length: 0xf, Color: 0 },
    radar_info::RadialDataPacketRadialRun { Length: 0xf, Color: 0 },
    radar_info::RadialDataPacketRadialRun { Length: 0xf, Color: 0 },
    radar_info::RadialDataPacketRadialRun { Length: 5, Color: 0 },
    radar_info::RadialDataPacketRadialRun { Length: 2, Color: 1 },
    radar_info::RadialDataPacketRadialRun { Length: 3, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 3 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 1, Color: 3 },
    radar_info::RadialDataPacketRadialRun { Length: 7, Color: 2 },
    radar_info::RadialDataPacketRadialRun { Length: 3, Color: 3 },
    radar_info::RadialDataPacketRadialRun { Length: 0, Color: 0 },
  ];

  assert_eq!(radial_data_packet_radial_runs[0].Length, radial_data_packet_radial_runs_control[0].Length);
  assert_eq!(radial_data_packet_radial_runs[1].Length, radial_data_packet_radial_runs_control[1].Length);
  assert_eq!(radial_data_packet_radial_runs[2].Length, radial_data_packet_radial_runs_control[2].Length);
  assert_eq!(radial_data_packet_radial_runs[3].Length, radial_data_packet_radial_runs_control[3].Length);
  assert_eq!(radial_data_packet_radial_runs[4].Length, radial_data_packet_radial_runs_control[4].Length);
  assert_eq!(radial_data_packet_radial_runs[5].Length, radial_data_packet_radial_runs_control[5].Length);
  assert_eq!(radial_data_packet_radial_runs[6].Length, radial_data_packet_radial_runs_control[6].Length);
  assert_eq!(radial_data_packet_radial_runs[7].Length, radial_data_packet_radial_runs_control[7].Length);
  assert_eq!(radial_data_packet_radial_runs[8].Length, radial_data_packet_radial_runs_control[8].Length);
  assert_eq!(radial_data_packet_radial_runs[9].Length, radial_data_packet_radial_runs_control[9].Length);
  assert_eq!(radial_data_packet_radial_runs[10].Length, radial_data_packet_radial_runs_control[10].Length);
  assert_eq!(radial_data_packet_radial_runs[11].Length, radial_data_packet_radial_runs_control[11].Length);
  assert_eq!(radial_data_packet_radial_runs[12].Length, radial_data_packet_radial_runs_control[12].Length);
  assert_eq!(radial_data_packet_radial_runs[13].Length, radial_data_packet_radial_runs_control[13].Length);
  assert_eq!(radial_data_packet_radial_runs[14].Length, radial_data_packet_radial_runs_control[14].Length);
  assert_eq!(radial_data_packet_radial_runs[15].Length, radial_data_packet_radial_runs_control[15].Length);
  assert_eq!(radial_data_packet_radial_runs[16].Length, radial_data_packet_radial_runs_control[16].Length);
  assert_eq!(radial_data_packet_radial_runs[17].Length, radial_data_packet_radial_runs_control[17].Length);
  assert_eq!(radial_data_packet_radial_runs[18].Length, radial_data_packet_radial_runs_control[18].Length);
  assert_eq!(radial_data_packet_radial_runs[19].Length, radial_data_packet_radial_runs_control[19].Length);
  assert_eq!(radial_data_packet_radial_runs[20].Length, radial_data_packet_radial_runs_control[20].Length);
  assert_eq!(radial_data_packet_radial_runs[21].Length, radial_data_packet_radial_runs_control[21].Length);
  assert_eq!(radial_data_packet_radial_runs[22].Length, radial_data_packet_radial_runs_control[22].Length);
  assert_eq!(radial_data_packet_radial_runs[23].Length, radial_data_packet_radial_runs_control[23].Length);
  assert_eq!(radial_data_packet_radial_runs[24].Length, radial_data_packet_radial_runs_control[24].Length);
  assert_eq!(radial_data_packet_radial_runs[25].Length, radial_data_packet_radial_runs_control[25].Length);
  assert_eq!(radial_data_packet_radial_runs[26].Length, radial_data_packet_radial_runs_control[26].Length);
  assert_eq!(radial_data_packet_radial_runs[27].Length, radial_data_packet_radial_runs_control[27].Length);
  assert_eq!(radial_data_packet_radial_runs[28].Length, radial_data_packet_radial_runs_control[28].Length);
  assert_eq!(radial_data_packet_radial_runs[29].Length, radial_data_packet_radial_runs_control[29].Length);
  assert_eq!(radial_data_packet_radial_runs[30].Length, radial_data_packet_radial_runs_control[30].Length);
  assert_eq!(radial_data_packet_radial_runs[31].Length, radial_data_packet_radial_runs_control[31].Length);
  assert_eq!(radial_data_packet_radial_runs[32].Length, radial_data_packet_radial_runs_control[32].Length);
  assert_eq!(radial_data_packet_radial_runs[33].Length, radial_data_packet_radial_runs_control[33].Length);
  assert_eq!(radial_data_packet_radial_runs[34].Length, radial_data_packet_radial_runs_control[34].Length);
  assert_eq!(radial_data_packet_radial_runs[35].Length, radial_data_packet_radial_runs_control[35].Length);
  assert_eq!(radial_data_packet_radial_runs[36].Length, radial_data_packet_radial_runs_control[36].Length);
  assert_eq!(radial_data_packet_radial_runs[37].Length, radial_data_packet_radial_runs_control[37].Length);
  assert_eq!(radial_data_packet_radial_runs[38].Length, radial_data_packet_radial_runs_control[38].Length);
  assert_eq!(radial_data_packet_radial_runs[39].Length, radial_data_packet_radial_runs_control[39].Length);
  assert_eq!(radial_data_packet_radial_runs[40].Length, radial_data_packet_radial_runs_control[40].Length);
  assert_eq!(radial_data_packet_radial_runs[41].Length, radial_data_packet_radial_runs_control[41].Length);
  assert_eq!(radial_data_packet_radial_runs[42].Length, radial_data_packet_radial_runs_control[42].Length);
  assert_eq!(radial_data_packet_radial_runs[43].Length, radial_data_packet_radial_runs_control[43].Length);
  assert_eq!(radial_data_packet_radial_runs[44].Length, radial_data_packet_radial_runs_control[44].Length);
  assert_eq!(radial_data_packet_radial_runs[45].Length, radial_data_packet_radial_runs_control[45].Length);
  assert_eq!(radial_data_packet_radial_runs[46].Length, radial_data_packet_radial_runs_control[46].Length);
  assert_eq!(radial_data_packet_radial_runs[47].Length, radial_data_packet_radial_runs_control[47].Length);
  assert_eq!(radial_data_packet_radial_runs[48].Length, radial_data_packet_radial_runs_control[48].Length);
  assert_eq!(radial_data_packet_radial_runs[49].Length, radial_data_packet_radial_runs_control[49].Length);
  assert_eq!(radial_data_packet_radial_runs[50].Length, radial_data_packet_radial_runs_control[50].Length);
  assert_eq!(radial_data_packet_radial_runs[51].Length, radial_data_packet_radial_runs_control[51].Length);
  assert_eq!(radial_data_packet_radial_runs[52].Length, radial_data_packet_radial_runs_control[52].Length);
  assert_eq!(radial_data_packet_radial_runs[53].Length, radial_data_packet_radial_runs_control[53].Length);
  assert_eq!(radial_data_packet_radial_runs[54].Length, radial_data_packet_radial_runs_control[54].Length);
  assert_eq!(radial_data_packet_radial_runs[55].Length, radial_data_packet_radial_runs_control[55].Length);
  assert_eq!(radial_data_packet_radial_runs[56].Length, radial_data_packet_radial_runs_control[56].Length);
  assert_eq!(radial_data_packet_radial_runs[57].Length, radial_data_packet_radial_runs_control[57].Length);

  assert_eq!(radial_data_packet_radial_runs[0].Color, radial_data_packet_radial_runs_control[0].Color);
  assert_eq!(radial_data_packet_radial_runs[1].Color, radial_data_packet_radial_runs_control[1].Color);
  assert_eq!(radial_data_packet_radial_runs[2].Color, radial_data_packet_radial_runs_control[2].Color);
  assert_eq!(radial_data_packet_radial_runs[3].Color, radial_data_packet_radial_runs_control[3].Color);
  assert_eq!(radial_data_packet_radial_runs[4].Color, radial_data_packet_radial_runs_control[4].Color);
  assert_eq!(radial_data_packet_radial_runs[5].Color, radial_data_packet_radial_runs_control[5].Color);
  assert_eq!(radial_data_packet_radial_runs[6].Color, radial_data_packet_radial_runs_control[6].Color);
  assert_eq!(radial_data_packet_radial_runs[7].Color, radial_data_packet_radial_runs_control[7].Color);
  assert_eq!(radial_data_packet_radial_runs[8].Color, radial_data_packet_radial_runs_control[8].Color);
  assert_eq!(radial_data_packet_radial_runs[9].Color, radial_data_packet_radial_runs_control[9].Color);
  assert_eq!(radial_data_packet_radial_runs[10].Color, radial_data_packet_radial_runs_control[10].Color);
  assert_eq!(radial_data_packet_radial_runs[11].Color, radial_data_packet_radial_runs_control[11].Color);
  assert_eq!(radial_data_packet_radial_runs[12].Color, radial_data_packet_radial_runs_control[12].Color);
  assert_eq!(radial_data_packet_radial_runs[13].Color, radial_data_packet_radial_runs_control[13].Color);
  assert_eq!(radial_data_packet_radial_runs[14].Color, radial_data_packet_radial_runs_control[14].Color);
  assert_eq!(radial_data_packet_radial_runs[15].Color, radial_data_packet_radial_runs_control[15].Color);
  assert_eq!(radial_data_packet_radial_runs[16].Color, radial_data_packet_radial_runs_control[16].Color);
  assert_eq!(radial_data_packet_radial_runs[17].Color, radial_data_packet_radial_runs_control[17].Color);
  assert_eq!(radial_data_packet_radial_runs[18].Color, radial_data_packet_radial_runs_control[18].Color);
  assert_eq!(radial_data_packet_radial_runs[19].Color, radial_data_packet_radial_runs_control[19].Color);
  assert_eq!(radial_data_packet_radial_runs[20].Color, radial_data_packet_radial_runs_control[20].Color);
  assert_eq!(radial_data_packet_radial_runs[21].Color, radial_data_packet_radial_runs_control[21].Color);
  assert_eq!(radial_data_packet_radial_runs[22].Color, radial_data_packet_radial_runs_control[22].Color);
  assert_eq!(radial_data_packet_radial_runs[23].Color, radial_data_packet_radial_runs_control[23].Color);
  assert_eq!(radial_data_packet_radial_runs[24].Color, radial_data_packet_radial_runs_control[24].Color);
  assert_eq!(radial_data_packet_radial_runs[25].Color, radial_data_packet_radial_runs_control[25].Color);
  assert_eq!(radial_data_packet_radial_runs[26].Color, radial_data_packet_radial_runs_control[26].Color);
  assert_eq!(radial_data_packet_radial_runs[27].Color, radial_data_packet_radial_runs_control[27].Color);
  assert_eq!(radial_data_packet_radial_runs[28].Color, radial_data_packet_radial_runs_control[28].Color);
  assert_eq!(radial_data_packet_radial_runs[29].Color, radial_data_packet_radial_runs_control[29].Color);
  assert_eq!(radial_data_packet_radial_runs[30].Color, radial_data_packet_radial_runs_control[30].Color);
  assert_eq!(radial_data_packet_radial_runs[31].Color, radial_data_packet_radial_runs_control[31].Color);
  assert_eq!(radial_data_packet_radial_runs[32].Color, radial_data_packet_radial_runs_control[32].Color);
  assert_eq!(radial_data_packet_radial_runs[33].Color, radial_data_packet_radial_runs_control[33].Color);
  assert_eq!(radial_data_packet_radial_runs[34].Color, radial_data_packet_radial_runs_control[34].Color);
  assert_eq!(radial_data_packet_radial_runs[35].Color, radial_data_packet_radial_runs_control[35].Color);
  assert_eq!(radial_data_packet_radial_runs[36].Color, radial_data_packet_radial_runs_control[36].Color);
  assert_eq!(radial_data_packet_radial_runs[37].Color, radial_data_packet_radial_runs_control[37].Color);
  assert_eq!(radial_data_packet_radial_runs[38].Color, radial_data_packet_radial_runs_control[38].Color);
  assert_eq!(radial_data_packet_radial_runs[39].Color, radial_data_packet_radial_runs_control[39].Color);
  assert_eq!(radial_data_packet_radial_runs[40].Color, radial_data_packet_radial_runs_control[40].Color);
  assert_eq!(radial_data_packet_radial_runs[41].Color, radial_data_packet_radial_runs_control[41].Color);
  assert_eq!(radial_data_packet_radial_runs[42].Color, radial_data_packet_radial_runs_control[42].Color);
  assert_eq!(radial_data_packet_radial_runs[43].Color, radial_data_packet_radial_runs_control[43].Color);
  assert_eq!(radial_data_packet_radial_runs[44].Color, radial_data_packet_radial_runs_control[44].Color);
  assert_eq!(radial_data_packet_radial_runs[45].Color, radial_data_packet_radial_runs_control[45].Color);
  assert_eq!(radial_data_packet_radial_runs[46].Color, radial_data_packet_radial_runs_control[46].Color);
  assert_eq!(radial_data_packet_radial_runs[47].Color, radial_data_packet_radial_runs_control[47].Color);
  assert_eq!(radial_data_packet_radial_runs[48].Color, radial_data_packet_radial_runs_control[48].Color);
  assert_eq!(radial_data_packet_radial_runs[49].Color, radial_data_packet_radial_runs_control[49].Color);
  assert_eq!(radial_data_packet_radial_runs[50].Color, radial_data_packet_radial_runs_control[50].Color);
  assert_eq!(radial_data_packet_radial_runs[51].Color, radial_data_packet_radial_runs_control[51].Color);
  assert_eq!(radial_data_packet_radial_runs[52].Color, radial_data_packet_radial_runs_control[52].Color);
  assert_eq!(radial_data_packet_radial_runs[53].Color, radial_data_packet_radial_runs_control[53].Color);
  assert_eq!(radial_data_packet_radial_runs[54].Color, radial_data_packet_radial_runs_control[54].Color);
  assert_eq!(radial_data_packet_radial_runs[55].Color, radial_data_packet_radial_runs_control[55].Color);
  assert_eq!(radial_data_packet_radial_runs[56].Color, radial_data_packet_radial_runs_control[56].Color);
  assert_eq!(radial_data_packet_radial_runs[57].Color, radial_data_packet_radial_runs_control[57].Color);
}

