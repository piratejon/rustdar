
pub struct TextHeader {
  text_header: [u8; 30],
}

pub struct MessageHeader {
  packed_values: [u8; 9],
}

pub struct ProductDescription {
  packed_values: [u8; 51],
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
  x: bool
}

impl std::default::Default for RadarFileParser {
  fn default() -> RadarFileParser {
    RadarFileParser {
      x: true
    }
  }
}

impl RadarFileParser {
  pub fn load_file(&mut self, radar_file_name : &str) -> bool {
    return true;
  }
}

/*
impl std::default::Default for ProductDescription {
  fn default() -> ProductDescription {
    packed_values: u8[51]
  }
}

impl std::default::Default for RadarParser {
  fn default() -> RadarParser {
    RadarParser {
      text_header: TttSquareValue::X,
      msg_header: MessageHeader,
      prod_descr: str,
      symbology_header: str,
      packet_header: str,
      radial_header: str,
      raw_data: str,
      data: str
    }
  }
}

impl RadarParser {
  pub fn init(&str) { }
}

impl TttSquare {
  pub fn set_x(&mut self) { self.value = TttSquareValue::X }
  pub fn set_o(&mut self) { self.value = TttSquareValue::O }
  pub fn set_blank(&mut self) { self.value = TttSquareValue::Blank }
  pub fn get_value(&self) -> char {
    match self.value {
      TttSquareValue::X => 'x',
      TttSquareValue::O => 'o',
      TttSquareValue::Blank => '_',
    }
  }
  pub fn set_value(&mut self, value : char) {
    match value {
      'X' | 'x' => self.set_x(),
      'O' | 'o' => self.set_o(),
      _ => self.set_blank(),
    }
  }
}

pub struct TttBoard {
  squares : [TttSquare; 9],
}

impl std::default::Default for TttBoard {
  fn default() -> TttBoard {
    TttBoard {
      squares : [
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
        { TttSquare { value : TttSquareValue::Blank } },
      ]
    }
  }
}

impl TttBoard {
  pub fn count_blanks(&self) -> usize {
    let mut count = 0;
    for i in self.squares.iter() {
      if i.get_value() == '_' {
        count += 1;
      }
    }
    count
  }

  pub fn get_square(&self, index : usize) -> char {
    ///*
       1 | 2 | 3
       ---------
       4 | 5 | 6
       ---------
       7 | 8 | 9
     //  */
    self.squares[index-1].get_value()
  }

  pub fn set_square(&mut self, index : usize, value : char) {
    self.squares[index-1].set_value(value)
  }

  pub fn winner(&self) -> char {
    if self.get_square(7) == 'x' && self.get_square(8) == 'x' && self.get_square(9) == 'x' {
      'x'
    } else if self.get_square(1) == 'o' && self.get_square(5) == 'o' && self.get_square(9) == 'o' {
      'o'
    } else if self.get_square(1) == 'x' && self.get_square(5) == 'x' && self.get_square(9) == 'x' {
      'x'
    } else {
      '_'
    }
  }

  pub fn reset(&mut self) {
    for i in (0..self.squares.len()) {
      self.squares[i].set_value('_')
    }
  }
}

#[derive(Copy)]
pub struct TttBoardBinRep {
  xs : u16,
  os : u16,
}

impl std::default::Default for TttBoardBinRep {
  fn default() -> TttBoardBinRep {
    TttBoardBinRep {
      xs : 0,
      os : 0,
    }
  }
}

impl Clone for TttBoardBinRep {
  fn clone(&self) -> Self { 
    TttBoardBinRep {
      xs : self.xs,
      os : self.os,
    }
  }
}

impl TttBoardBinRep {
  pub fn count_blanks(&self) -> u16 {
    let mut count = 9;
    for i in 0..9 {
      count -= ((self.xs | self.os) >> i) & 1
    }
    count
  }

  pub fn get_square(&self, i : u8) -> char {
    if self.xs & (1 << (i-1)) > 0 { 'x' }
    else if self.os & (1 << (i-1)) > 0 { 'o' }
    else { '_' }
  }

  pub fn set_square(&mut self, i : u8, value : char) {
    if value == 'x' || value == 'X' {
      self.set_x(i)
    } else if value == 'o' || value == 'O' {
      self.set_o(i)
    } else {
      self.set_blank(i)
    }
  }

  pub fn disable_x(&mut self, i : u8) {
    self.xs &= 0b111_111_111 ^ (1 << (i-1));
  }

  pub fn disable_o(&mut self, i : u8) {
    self.os &= 0b111_111_111 ^ (1 << (i-1));
  }

  pub fn set_x(&mut self, i : u8) {
    self.disable_o(i);
    self.xs |= 1 << (i-1)
  }

  pub fn set_o(&mut self, i : u8) {
    self.disable_x(i);
    self.os |= 1 << (i-1)
  }

  pub fn set_blank(&mut self, i : u8) {
    self.disable_x(i);
    self.disable_o(i);
  }

  pub fn reset(&mut self) {
    self.xs = 0;
    self.os = 0;
  }

  pub fn check_positions(&self, p : u16) -> bool {
    p & 0b111_000_000 == 0b111_000_000
      || p & 0b000_111_000 == 0b000_111_000
      || p & 0b000_000_111 == 0b000_000_111
      || p & 0b100_100_100 == 0b100_100_100
      || p & 0b010_010_010 == 0b010_010_010
      || p & 0b001_001_001 == 0b001_001_001
      || p & 0b100_010_001 == 0b100_010_001
      || p & 0b001_010_100 == 0b001_010_100
  }

  pub fn winner(&self) -> char {
    if self.check_positions(self.xs) { 'x' }
    else if self.check_positions(self.os) { 'o' }
    else { '_' }
  }

  pub fn from_string(&mut self, state : &str) {
    if state.len() == 9 {
      self.set_square(1, state.char_at(0));
      self.set_square(2, state.char_at(1));
      self.set_square(3, state.char_at(2));
      self.set_square(4, state.char_at(3));
      self.set_square(5, state.char_at(4));
      self.set_square(6, state.char_at(5));
      self.set_square(7, state.char_at(6));
      self.set_square(8, state.char_at(7));
      self.set_square(9, state.char_at(8));
    }
  }

  pub fn as_string(&self) -> String {
    let mut out = String::with_capacity(9);
    out.push(self.get_square(1));
    out.push(self.get_square(2));
    out.push(self.get_square(3));
    out.push(self.get_square(4));
    out.push(self.get_square(5));
    out.push(self.get_square(6));
    out.push(self.get_square(7));
    out.push(self.get_square(8));
    out.push(self.get_square(9));
    out
  }

  pub fn compute_x_next_move(&self) -> u8 {
    9
  }

  pub fn get_open_positions(&self) -> Vec<u8> {
    let mut o = Vec::new();
    for i in 0..9 {
      if (((self.xs | self.os) >> i) & 1) == 0 {
        o.push(i+1);
      }
    }
    o
  }

  pub fn score_single_board(&self, p1 : char, p2 : char) -> u8 {
    let w = self.winner();
    if w == p1 {
      10
    } else if w == p2 {
      -10
    } else {
      0
    }
  }

  pub fn winning_move_list(&mut self, p1 : char, p2 : char) -> Vec<u8> {
    let mut w = Vec::new();
    if self.winner() == '_' {
      let o = self.get_open_positions();
      for i in o.iter() {
        self.set_square(*i, p1);
        if self.winner() == p1 {
          w.push(*i);
        }
        self.set_blank(*i);
      }
    }
    w
  }

  pub fn non_winning_move_list(&mut self, p1 : char, p2 : char) -> Vec<u8> {
    let mut w = Vec::new();
    if self.winner() == '_' {
      let o = self.get_open_positions();
      for i in o.iter() {
        self.set_square(*i, p1);
        if self.winner() == '_' {
          w.push(*i);
        }
        self.set_blank(*i);
      }
    }
    w
  }

  pub fn iterate(&mut self, p1 : char, p2 : char) {
    match self.winner() {
      'x' => {
        print!("x wins: {}", self.as_string());
      },
      'o' => {
        print!("o wins: {}", self.as_string());
      }
      _ => {
        let o = self.get_open_positions();
        for i in o.iter() {
          self.set_square(*i, p1);
          self.iterate(p2, p1);
          self.set_blank(*i);
        }
      }
    }
  }

  pub fn minimax_score(&mut self, p1 : char, p2 : char) -> i8 {
    let w = self.winner();
    if w == p1 {
      10 - (self.get_open_positions().len() as i8)
    } else if w == p2 {
      - self.minimax_score(p2, p1)
    } else {
      let o = self.get_open_positions();
      let mut tmp_best = 0;
      for i in o.iter() {
        self.set_square(*i, p1);
        let tmp_score = self.minimax_score(p2, p1);
        if tmp_score > tmp_best {
          tmp_best = tmp_score
        }
        self.set_blank(*i);
      }
      tmp_best
    }
  }
}

*/

