
extern crate radar_info;

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
fn ri_decode_a_file() {
  let mut radar_fetcher = radar_info::RadarFetcher::from_file("tests/sn.last");
  let radar_decoder : radar_info::RadarFileDecoder = radar_info::RadarFileDecoder {
    x: 99,
  };

  assert_eq!(radar_decoder.decode_text_header(&mut radar_fetcher), "SDUS54 KOUN 030251\r\r\nN0RTLX\r\r\n");

  let message_header = radar_decoder.decode_message_header(&mut radar_fetcher);
  assert_eq!(message_header.message_code, 19);
  assert_eq!(message_header.date_of_message, 16651);
  assert_eq!(message_header.time_of_message, 10324);
  assert_eq!(message_header.length_of_message, 22148);
  assert_eq!(message_header.source_id, 1);
  assert_eq!(message_header.destination_id, 0);
  assert_eq!(message_header.number_of_blocks, 3);

  let product_description_block = radar_decoder.decode_product_description_block(&mut radar_fetcher);
  assert_eq!(product_description_block.divider, 0xffff);
  assert_eq!(product_description_block.latitude_1k, 35333);
  assert_eq!(product_description_block.longitude_1k, -97278);
  assert_eq!(product_description_block.height, 1277);
  assert_eq!(product_description_block.product_code, 19);
  assert_eq!(product_description_block.operational_mode, 2);
  assert_eq!(product_description_block.volume_coverage_pattern, 212);
  assert_eq!(product_description_block.sequence_number, 3380);
  assert_eq!(product_description_block.volume_scan_number, 0x2e);
  assert_eq!(product_description_block.volume_scan_date, 16651);
  assert_eq!(product_description_block.volume_scan_start_time, 10303);
  assert_eq!(product_description_block.product_generation_date, 16651);
  assert_eq!(product_description_block.product_generation_time, 10323);
  assert_eq!(product_description_block.product_dependent[0], 0);
  assert_eq!(product_description_block.product_dependent[1], 0);
  assert_eq!(product_description_block.product_dependent[2], 5);
  assert_eq!(product_description_block.product_dependent[3], 0x003b);
  assert_eq!(product_description_block.product_dependent[4], 0);
  assert_eq!(product_description_block.product_dependent[5], 0);
  assert_eq!(product_description_block.product_dependent[6], 0);
  assert_eq!(product_description_block.product_dependent[7], 0xc22f);
  assert_eq!(product_description_block.product_dependent[8], 0xa50c);
  assert_eq!(product_description_block.product_dependent[9], 0);
  assert_eq!(product_description_block.elevation_number, 1);
  assert_eq!(product_description_block.data_level_threshold[0], 0x8002);
  assert_eq!(product_description_block.data_level_threshold[1], 0x0005);
  assert_eq!(product_description_block.data_level_threshold[2], 0x000a);
  assert_eq!(product_description_block.data_level_threshold[3], 0x000f);
  assert_eq!(product_description_block.data_level_threshold[4], 0x0014);
  assert_eq!(product_description_block.data_level_threshold[5], 0x0019);
  assert_eq!(product_description_block.data_level_threshold[6], 0x001e);
  assert_eq!(product_description_block.data_level_threshold[7], 0x0023);
  assert_eq!(product_description_block.data_level_threshold[8], 0x0028);
  assert_eq!(product_description_block.data_level_threshold[9], 0x002d);
  assert_eq!(product_description_block.data_level_threshold[10], 0x0032);
  assert_eq!(product_description_block.data_level_threshold[11], 0x0037);
  assert_eq!(product_description_block.data_level_threshold[12], 0x003c);
  assert_eq!(product_description_block.data_level_threshold[13], 0x0041);
  assert_eq!(product_description_block.data_level_threshold[14], 0x0046);
  assert_eq!(product_description_block.data_level_threshold[15], 0x004b);
  assert_eq!(product_description_block.version, 0);
  assert_eq!(product_description_block.spot_blank, 0);
  assert_eq!(product_description_block.offset_to_symbology, 60);
  assert_eq!(product_description_block.offset_to_graphic, 0);
  assert_eq!(product_description_block.offset_to_tabular, 0);

  let product_symbology_block = radar_decoder.decode_product_symbology_block(&mut radar_fetcher);
  assert_eq!(product_symbology_block.divider, 0xffff);
  assert_eq!(product_symbology_block.block_id, 1);
  assert_eq!(product_symbology_block.length_of_block, 22028);
  assert_eq!(product_symbology_block.number_of_layers, 1);

  let product_symbology_block_layer = radar_decoder.decode_product_symbology_block_layer(&mut radar_fetcher);
  assert_eq!(product_symbology_block_layer.divider, 0xffff);
  assert_eq!(product_symbology_block_layer.length_of_data_layer, 22012);

  let radial_data_packet_header = radar_decoder.decode_radial_data_packet_header(&mut radar_fetcher);
  assert_eq!(radial_data_packet_header.packet_code, 0xaf1f);
  assert_eq!(radial_data_packet_header.index_of_first_range_bin, 0);
  assert_eq!(radial_data_packet_header.number_of_range_bins, 230);
  assert_eq!(radial_data_packet_header.i_center_of_sweep, 256);
  assert_eq!(radial_data_packet_header.j_center_of_sweep, 280);
  assert_eq!(radial_data_packet_header.scale_factor, 999);
  assert_eq!(radial_data_packet_header.number_of_radials, 360);

  let radial_data_packet_radial_header = radar_decoder.decode_radial_data_packet_radial_header(&mut radar_fetcher);
  assert_eq!(radial_data_packet_radial_header.number_of_half_word_words, 30);
  assert_eq!(radial_data_packet_radial_header.radial_start_angle, 2680);
  assert_eq!(radial_data_packet_radial_header.radial_angle_delta, 10);

  let radial_data_packet_radial_run = radar_decoder.fetch_radial_data_packet_radial_run(&mut radar_fetcher);
  assert_eq!(radial_data_packet_radial_run.length, 2);
  assert_eq!(radial_data_packet_radial_run.color, 0);

  let radial_data_packet_radial_run = radar_decoder.fetch_radial_data_packet_radial_run(&mut radar_fetcher);
  assert_eq!(radial_data_packet_radial_run.length, 3);
  assert_eq!(radial_data_packet_radial_run.color, 2);

  let radial_data_packet_radial_runs = radar_decoder.fetch_radial_data_packet_radial_runs(&mut radar_fetcher, radial_data_packet_radial_header.number_of_half_word_words - 1);

  let radial_data_packet_radial_runs_control = vec![
    radar_info::RadialDataPacketRadialRun { length: 1, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 3 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 5, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 4, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 4 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 2, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 5, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 3, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 3, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 2, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 4, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 3, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 2, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 4, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 2, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 2, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 6, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 3, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 4 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 0 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 3 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 4 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 2, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 3, color: 0 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 2, color: 0 },
    radar_info::RadialDataPacketRadialRun { length: 4, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 0 },
    radar_info::RadialDataPacketRadialRun { length: 2, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 0xd, color: 0 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 0xf, color: 0 },
    radar_info::RadialDataPacketRadialRun { length: 0xf, color: 0 },
    radar_info::RadialDataPacketRadialRun { length: 0xf, color: 0 },
    radar_info::RadialDataPacketRadialRun { length: 0xf, color: 0 },
    radar_info::RadialDataPacketRadialRun { length: 0xf, color: 0 },
    radar_info::RadialDataPacketRadialRun { length: 0xf, color: 0 },
    radar_info::RadialDataPacketRadialRun { length: 0xf, color: 0 },
    radar_info::RadialDataPacketRadialRun { length: 5, color: 0 },
    radar_info::RadialDataPacketRadialRun { length: 2, color: 1 },
    radar_info::RadialDataPacketRadialRun { length: 3, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 3 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 1, color: 3 },
    radar_info::RadialDataPacketRadialRun { length: 7, color: 2 },
    radar_info::RadialDataPacketRadialRun { length: 3, color: 3 },
    radar_info::RadialDataPacketRadialRun { length: 0, color: 0 },
  ];

  assert_eq!(radial_data_packet_radial_runs[0].length, radial_data_packet_radial_runs_control[0].length);
  assert_eq!(radial_data_packet_radial_runs[1].length, radial_data_packet_radial_runs_control[1].length);
  assert_eq!(radial_data_packet_radial_runs[2].length, radial_data_packet_radial_runs_control[2].length);
  assert_eq!(radial_data_packet_radial_runs[3].length, radial_data_packet_radial_runs_control[3].length);
  assert_eq!(radial_data_packet_radial_runs[4].length, radial_data_packet_radial_runs_control[4].length);
  assert_eq!(radial_data_packet_radial_runs[5].length, radial_data_packet_radial_runs_control[5].length);
  assert_eq!(radial_data_packet_radial_runs[6].length, radial_data_packet_radial_runs_control[6].length);
  assert_eq!(radial_data_packet_radial_runs[7].length, radial_data_packet_radial_runs_control[7].length);
  assert_eq!(radial_data_packet_radial_runs[8].length, radial_data_packet_radial_runs_control[8].length);
  assert_eq!(radial_data_packet_radial_runs[9].length, radial_data_packet_radial_runs_control[9].length);
  assert_eq!(radial_data_packet_radial_runs[10].length, radial_data_packet_radial_runs_control[10].length);
  assert_eq!(radial_data_packet_radial_runs[11].length, radial_data_packet_radial_runs_control[11].length);
  assert_eq!(radial_data_packet_radial_runs[12].length, radial_data_packet_radial_runs_control[12].length);
  assert_eq!(radial_data_packet_radial_runs[13].length, radial_data_packet_radial_runs_control[13].length);
  assert_eq!(radial_data_packet_radial_runs[14].length, radial_data_packet_radial_runs_control[14].length);
  assert_eq!(radial_data_packet_radial_runs[15].length, radial_data_packet_radial_runs_control[15].length);
  assert_eq!(radial_data_packet_radial_runs[16].length, radial_data_packet_radial_runs_control[16].length);
  assert_eq!(radial_data_packet_radial_runs[17].length, radial_data_packet_radial_runs_control[17].length);
  assert_eq!(radial_data_packet_radial_runs[18].length, radial_data_packet_radial_runs_control[18].length);
  assert_eq!(radial_data_packet_radial_runs[19].length, radial_data_packet_radial_runs_control[19].length);
  assert_eq!(radial_data_packet_radial_runs[20].length, radial_data_packet_radial_runs_control[20].length);
  assert_eq!(radial_data_packet_radial_runs[21].length, radial_data_packet_radial_runs_control[21].length);
  assert_eq!(radial_data_packet_radial_runs[22].length, radial_data_packet_radial_runs_control[22].length);
  assert_eq!(radial_data_packet_radial_runs[23].length, radial_data_packet_radial_runs_control[23].length);
  assert_eq!(radial_data_packet_radial_runs[24].length, radial_data_packet_radial_runs_control[24].length);
  assert_eq!(radial_data_packet_radial_runs[25].length, radial_data_packet_radial_runs_control[25].length);
  assert_eq!(radial_data_packet_radial_runs[26].length, radial_data_packet_radial_runs_control[26].length);
  assert_eq!(radial_data_packet_radial_runs[27].length, radial_data_packet_radial_runs_control[27].length);
  assert_eq!(radial_data_packet_radial_runs[28].length, radial_data_packet_radial_runs_control[28].length);
  assert_eq!(radial_data_packet_radial_runs[29].length, radial_data_packet_radial_runs_control[29].length);
  assert_eq!(radial_data_packet_radial_runs[30].length, radial_data_packet_radial_runs_control[30].length);
  assert_eq!(radial_data_packet_radial_runs[31].length, radial_data_packet_radial_runs_control[31].length);
  assert_eq!(radial_data_packet_radial_runs[32].length, radial_data_packet_radial_runs_control[32].length);
  assert_eq!(radial_data_packet_radial_runs[33].length, radial_data_packet_radial_runs_control[33].length);
  assert_eq!(radial_data_packet_radial_runs[34].length, radial_data_packet_radial_runs_control[34].length);
  assert_eq!(radial_data_packet_radial_runs[35].length, radial_data_packet_radial_runs_control[35].length);
  assert_eq!(radial_data_packet_radial_runs[36].length, radial_data_packet_radial_runs_control[36].length);
  assert_eq!(radial_data_packet_radial_runs[37].length, radial_data_packet_radial_runs_control[37].length);
  assert_eq!(radial_data_packet_radial_runs[38].length, radial_data_packet_radial_runs_control[38].length);
  assert_eq!(radial_data_packet_radial_runs[39].length, radial_data_packet_radial_runs_control[39].length);
  assert_eq!(radial_data_packet_radial_runs[40].length, radial_data_packet_radial_runs_control[40].length);
  assert_eq!(radial_data_packet_radial_runs[41].length, radial_data_packet_radial_runs_control[41].length);
  assert_eq!(radial_data_packet_radial_runs[42].length, radial_data_packet_radial_runs_control[42].length);
  assert_eq!(radial_data_packet_radial_runs[43].length, radial_data_packet_radial_runs_control[43].length);
  assert_eq!(radial_data_packet_radial_runs[44].length, radial_data_packet_radial_runs_control[44].length);
  assert_eq!(radial_data_packet_radial_runs[45].length, radial_data_packet_radial_runs_control[45].length);
  assert_eq!(radial_data_packet_radial_runs[46].length, radial_data_packet_radial_runs_control[46].length);
  assert_eq!(radial_data_packet_radial_runs[47].length, radial_data_packet_radial_runs_control[47].length);
  assert_eq!(radial_data_packet_radial_runs[48].length, radial_data_packet_radial_runs_control[48].length);
  assert_eq!(radial_data_packet_radial_runs[49].length, radial_data_packet_radial_runs_control[49].length);
  assert_eq!(radial_data_packet_radial_runs[50].length, radial_data_packet_radial_runs_control[50].length);
  assert_eq!(radial_data_packet_radial_runs[51].length, radial_data_packet_radial_runs_control[51].length);
  assert_eq!(radial_data_packet_radial_runs[52].length, radial_data_packet_radial_runs_control[52].length);
  assert_eq!(radial_data_packet_radial_runs[53].length, radial_data_packet_radial_runs_control[53].length);
  assert_eq!(radial_data_packet_radial_runs[54].length, radial_data_packet_radial_runs_control[54].length);
  assert_eq!(radial_data_packet_radial_runs[55].length, radial_data_packet_radial_runs_control[55].length);
  assert_eq!(radial_data_packet_radial_runs[56].length, radial_data_packet_radial_runs_control[56].length);
  assert_eq!(radial_data_packet_radial_runs[57].length, radial_data_packet_radial_runs_control[57].length);

  assert_eq!(radial_data_packet_radial_runs[0].color, radial_data_packet_radial_runs_control[0].color);
  assert_eq!(radial_data_packet_radial_runs[1].color, radial_data_packet_radial_runs_control[1].color);
  assert_eq!(radial_data_packet_radial_runs[2].color, radial_data_packet_radial_runs_control[2].color);
  assert_eq!(radial_data_packet_radial_runs[3].color, radial_data_packet_radial_runs_control[3].color);
  assert_eq!(radial_data_packet_radial_runs[4].color, radial_data_packet_radial_runs_control[4].color);
  assert_eq!(radial_data_packet_radial_runs[5].color, radial_data_packet_radial_runs_control[5].color);
  assert_eq!(radial_data_packet_radial_runs[6].color, radial_data_packet_radial_runs_control[6].color);
  assert_eq!(radial_data_packet_radial_runs[7].color, radial_data_packet_radial_runs_control[7].color);
  assert_eq!(radial_data_packet_radial_runs[8].color, radial_data_packet_radial_runs_control[8].color);
  assert_eq!(radial_data_packet_radial_runs[9].color, radial_data_packet_radial_runs_control[9].color);
  assert_eq!(radial_data_packet_radial_runs[10].color, radial_data_packet_radial_runs_control[10].color);
  assert_eq!(radial_data_packet_radial_runs[11].color, radial_data_packet_radial_runs_control[11].color);
  assert_eq!(radial_data_packet_radial_runs[12].color, radial_data_packet_radial_runs_control[12].color);
  assert_eq!(radial_data_packet_radial_runs[13].color, radial_data_packet_radial_runs_control[13].color);
  assert_eq!(radial_data_packet_radial_runs[14].color, radial_data_packet_radial_runs_control[14].color);
  assert_eq!(radial_data_packet_radial_runs[15].color, radial_data_packet_radial_runs_control[15].color);
  assert_eq!(radial_data_packet_radial_runs[16].color, radial_data_packet_radial_runs_control[16].color);
  assert_eq!(radial_data_packet_radial_runs[17].color, radial_data_packet_radial_runs_control[17].color);
  assert_eq!(radial_data_packet_radial_runs[18].color, radial_data_packet_radial_runs_control[18].color);
  assert_eq!(radial_data_packet_radial_runs[19].color, radial_data_packet_radial_runs_control[19].color);
  assert_eq!(radial_data_packet_radial_runs[20].color, radial_data_packet_radial_runs_control[20].color);
  assert_eq!(radial_data_packet_radial_runs[21].color, radial_data_packet_radial_runs_control[21].color);
  assert_eq!(radial_data_packet_radial_runs[22].color, radial_data_packet_radial_runs_control[22].color);
  assert_eq!(radial_data_packet_radial_runs[23].color, radial_data_packet_radial_runs_control[23].color);
  assert_eq!(radial_data_packet_radial_runs[24].color, radial_data_packet_radial_runs_control[24].color);
  assert_eq!(radial_data_packet_radial_runs[25].color, radial_data_packet_radial_runs_control[25].color);
  assert_eq!(radial_data_packet_radial_runs[26].color, radial_data_packet_radial_runs_control[26].color);
  assert_eq!(radial_data_packet_radial_runs[27].color, radial_data_packet_radial_runs_control[27].color);
  assert_eq!(radial_data_packet_radial_runs[28].color, radial_data_packet_radial_runs_control[28].color);
  assert_eq!(radial_data_packet_radial_runs[29].color, radial_data_packet_radial_runs_control[29].color);
  assert_eq!(radial_data_packet_radial_runs[30].color, radial_data_packet_radial_runs_control[30].color);
  assert_eq!(radial_data_packet_radial_runs[31].color, radial_data_packet_radial_runs_control[31].color);
  assert_eq!(radial_data_packet_radial_runs[32].color, radial_data_packet_radial_runs_control[32].color);
  assert_eq!(radial_data_packet_radial_runs[33].color, radial_data_packet_radial_runs_control[33].color);
  assert_eq!(radial_data_packet_radial_runs[34].color, radial_data_packet_radial_runs_control[34].color);
  assert_eq!(radial_data_packet_radial_runs[35].color, radial_data_packet_radial_runs_control[35].color);
  assert_eq!(radial_data_packet_radial_runs[36].color, radial_data_packet_radial_runs_control[36].color);
  assert_eq!(radial_data_packet_radial_runs[37].color, radial_data_packet_radial_runs_control[37].color);
  assert_eq!(radial_data_packet_radial_runs[38].color, radial_data_packet_radial_runs_control[38].color);
  assert_eq!(radial_data_packet_radial_runs[39].color, radial_data_packet_radial_runs_control[39].color);
  assert_eq!(radial_data_packet_radial_runs[40].color, radial_data_packet_radial_runs_control[40].color);
  assert_eq!(radial_data_packet_radial_runs[41].color, radial_data_packet_radial_runs_control[41].color);
  assert_eq!(radial_data_packet_radial_runs[42].color, radial_data_packet_radial_runs_control[42].color);
  assert_eq!(radial_data_packet_radial_runs[43].color, radial_data_packet_radial_runs_control[43].color);
  assert_eq!(radial_data_packet_radial_runs[44].color, radial_data_packet_radial_runs_control[44].color);
  assert_eq!(radial_data_packet_radial_runs[45].color, radial_data_packet_radial_runs_control[45].color);
  assert_eq!(radial_data_packet_radial_runs[46].color, radial_data_packet_radial_runs_control[46].color);
  assert_eq!(radial_data_packet_radial_runs[47].color, radial_data_packet_radial_runs_control[47].color);
  assert_eq!(radial_data_packet_radial_runs[48].color, radial_data_packet_radial_runs_control[48].color);
  assert_eq!(radial_data_packet_radial_runs[49].color, radial_data_packet_radial_runs_control[49].color);
  assert_eq!(radial_data_packet_radial_runs[50].color, radial_data_packet_radial_runs_control[50].color);
  assert_eq!(radial_data_packet_radial_runs[51].color, radial_data_packet_radial_runs_control[51].color);
  assert_eq!(radial_data_packet_radial_runs[52].color, radial_data_packet_radial_runs_control[52].color);
  assert_eq!(radial_data_packet_radial_runs[53].color, radial_data_packet_radial_runs_control[53].color);
  assert_eq!(radial_data_packet_radial_runs[54].color, radial_data_packet_radial_runs_control[54].color);
  assert_eq!(radial_data_packet_radial_runs[55].color, radial_data_packet_radial_runs_control[55].color);
  assert_eq!(radial_data_packet_radial_runs[56].color, radial_data_packet_radial_runs_control[56].color);
  assert_eq!(radial_data_packet_radial_runs[57].color, radial_data_packet_radial_runs_control[57].color);

  let radial_data_packet_radial_header = radar_decoder.decode_radial_data_packet_radial_header(&mut radar_fetcher);
  assert_eq!(radial_data_packet_radial_header.number_of_half_word_words, 34);
  assert_eq!(radial_data_packet_radial_header.radial_start_angle, 2690);
  assert_eq!(radial_data_packet_radial_header.radial_angle_delta, 9);

  radar_decoder.fetch_radial_data_packet_radial_runs(&mut radar_fetcher, radial_data_packet_radial_header.number_of_half_word_words);

  let radial_data_packet_radial_header = radar_decoder.decode_radial_data_packet_radial_header(&mut radar_fetcher);
  assert_eq!(radial_data_packet_radial_header.radial_start_angle, 2699);
  radar_decoder.fetch_radial_data_packet_radial_runs(&mut radar_fetcher, radial_data_packet_radial_header.number_of_half_word_words);

  for _ in 3..radial_data_packet_header.number_of_radials {
    let radial_data_packet_radial_header = radar_decoder.decode_radial_data_packet_radial_header(&mut radar_fetcher);
    assert!(radar_fetcher.last_read_size > 0);
    radar_decoder.fetch_radial_data_packet_radial_runs(&mut radar_fetcher, radial_data_packet_radial_header.number_of_half_word_words);
    assert!(radar_fetcher.last_read_size > 0);
  }

  let remaining_bytes = radar_fetcher.fetch_remaining_bytes();
  assert_eq!(remaining_bytes.len(), 0);
}

/*
#[test]
fn ri_parse_a_file() {
  let mut radar_parser =
    radar_info::RadarFileParser::from_decoder(
      radar_info::RadarFileDecoder::from_fetcher(
        radar_info::RadarFetcher::from_file("tests/sn.last")
        )
      );

  let parsed_file = radar_parser.parse();
  assert_eq!(parsed_file.get_text_header(), "SDUS54 KOUN 030251\r\r\nN0RTLX\r\r\n");
}
*/
