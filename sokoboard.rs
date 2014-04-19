use std::fmt;
use std::from_str::{FromStr};

pub struct SokoBoard( pub ~[~[Field]] );

impl FromStr for SokoBoard {
  fn from_str(s: &str) -> Option<SokoBoard> {
    use std::cmp::max;
    let lines: ~[&str] = s.lines().collect();
    let rows = lines.len();
    let cols = lines.iter().fold(0u,|maxL, row| max(maxL,row.len()));
    if rows == 0 || cols == 0 {
      None
    } else {
      let mut result = Vec::new();
      for line in lines.iter() {
        let mut row = Vec::new();
        for ix in range(0,line.len()) {
          let field: &str = line.slice_chars(ix, ix+1);
          let fld: Field = FromStr::from_str(field).expect(format!("Invalid character {:?} in screen!", field));
          row.push(fld);
        }
        while row.len() < cols {
          row.push(Empty);
        }
        result.push(row.move_iter().collect::<~[Field]>());
      }
      Some(SokoBoard(result.move_iter().collect()))
    }
  }
}

impl fmt::Show for SokoBoard {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    let &SokoBoard(ref cols) = self;
    for row in cols.iter() {
      for field in row.iter() {
        field.fmt(fmt);
      }
      fmt.buf.write_line("");
    }
    Ok(())
  }
}

#[deriving(Eq)]
pub enum Field {
  Empty,
  Wall,
  Man,
  Block,
  Goal,
  BlockOnGoal,
  ManOnGoal,
}

impl FromStr for Field {
  fn from_str(s: &str) -> Option<Field> {
    match s {
      " " => Some(Empty),
      "#" => Some(Wall),
      "@" => Some(Man),
      "$" => Some(Block),
      "." => Some(Goal),
      "*" => Some(BlockOnGoal),
      "+" => Some(ManOnGoal),
      _   => None,
    }
  }
}

impl fmt::Show for Field {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> super::std::fmt::Result {
    let chr = match *self {
      Empty => ' ',
      Wall => '#',
      Man => '@',
      Block => '$',
      Goal => '.',
      BlockOnGoal => '*',
      ManOnGoal => '+',
    };
    fmt.buf.write_char(chr);
    Ok(())
  }
}
