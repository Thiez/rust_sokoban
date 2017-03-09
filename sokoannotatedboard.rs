use super::bdd::{Bdd};
use super::sokoboard::{SokoBoard, Field, Block, Man, Wall, Goal, BlockOnGoal, ManOnGoal};

/// Represents a sokoban playing field. The individual squares are annotated
/// with some data that is gathered at initialization.
pub struct SokoAnnotatedBoard {
  board: Vec<Vec<AnnotatedField>>,
}

impl SokoAnnotatedBoard {
  /// Creates a SokoAnnotatedBoard using a Sokoboard.
  pub fn fromSokoBoard(board: SokoBoard) -> SokoAnnotatedBoard {
    let SokoBoard(board) = board;
    let mut newBoard = Vec::new();
    for row in range(0, board.len()) {
      let mut newRow = Vec::new();
      for col in range(0, board[row].len()) {
        newRow.push( AnnotatedField::new( board[row][col], row, col ) );
      }
      newBoard.push(newRow.move_iter().collect::<Vec<AnnotatedField>>());
    }

    let mut result = SokoAnnotatedBoard{
      board: newBoard.move_iter().collect(),
    };
    reachability(&mut result);
    productivity(&mut result);
    sanityCheck(&result);
    assignIDs(&mut result);
    result
  }

  /*
  /// Prints a representation of the squares that have been identified as 'productive';
  /// that is, all squares that coudl contain a box without the game becomming
  /// unsolvable.
  pub fn showProductive(&self) {
    use super::std::strbuf::StrBuf;
    for row in self.board.iter() {
      let mut sb = StrBuf::new();
      for field in row.iter() {
        sb.push_char( if field.productive { '+' } else { ' ' } );
      }
      println!("{}",sb.as_slice());
    }
  }*/

  /*
  /// Prints a representation of the squares that are reachable by the man
  pub fn showReachable(&self) {
    use super::std::strbuf::StrBuf;
    for row in self.board.iter() {
      let mut sb = StrBuf::new();
      for field in row.iter() {
        sb.push_char( if field.reachable { 'X' } else { ' ' } );
      }
      println!("{}",sb.as_slice());
    }
  }*/
}

#[derive(Eq)]
/// An annotated field.
/// The pair (`row`,`col`) represent this field's coordinates w.r.t. the playing field.
/// `reachable` is `true` if this square is reachable
/// `productive` is `true` if this square is productive
/// `man_id` is a unique id for the property of the man being or not being in this square
/// `block_id` is a unique id for the property of a block being or not being in this square
struct AnnotatedField {
  field: Field,
  row: uint,
  col: uint,
  reachable: bool,
  productive: bool,
  man_id: Option<u32>,
  block_id: Option<u32>,
}

impl AnnotatedField {
  pub fn new(field: Field, row: uint, col: uint) -> AnnotatedField {
    AnnotatedField {
      field: field,
      row: row,
      col: col,
      reachable: false,
      productive: field == Goal || field == BlockOnGoal || field == ManOnGoal,
      man_id: None,
      block_id: None,
    }
  }
  pub fn hasBlock(&self) -> bool {
    match self.field {
      Block | BlockOnGoal => true,
      _ => false
    }
  }
  pub fn hasMan(&self) -> bool {
    match self.field {
      Man | ManOnGoal => true,
      _ => false
    }
  }
  pub fn isGoal(&self) -> bool {
    match self.field {
      Goal | ManOnGoal | BlockOnGoal => true,
      _ => false
    }
  }
}

fn valid_index<T>(row: uint, col:uint, fields: &[Vec<T>]) -> bool {
  row < fields.len() && col < fields[row].len()
}

fn reachability(soko: &mut SokoAnnotatedBoard) {

  fn visit(board: &mut [Vec<AnnotatedField>], row: uint, col: uint) {
    if !(board[row][col].reachable || board[row][col].field == Wall) {
      board[row][col].reachable = true;
      visit(board, row+1, col);
      visit(board, row-1, col);
      visit(board, row, col+1);
      visit(board, row, col-1);
    }
  }

  for row in range(0,soko.board.len()) {
    for col in range(0,soko.board[row].len()) {
      if soko.board[row][col].hasMan() {
        visit(soko.board.as_mut_slice(), row, col);
        return;
      }
    }
  }
}

fn productivity(soko: &mut SokoAnnotatedBoard) {

  fn check_productive(row: uint, col: uint, dr: uint, dc: uint, fields: &mut [Vec<AnnotatedField>]) -> bool {
    if valid_index(row,col,fields)
        && fields[row][col].reachable
        && valid_index(row+dr, col+dc, fields)
        && valid_index(row-dr, col-dc, fields)
        && !fields[row][col].productive
        && fields[row+dr][col+dc].productive
        && fields[row-dr][col-dc].reachable {
      fields[row][col].productive = true;
      true
    } else {
      false
    }
  }

  let mut change = true;
  let fields = soko.board.as_mut_slice();
  while change {
    change = false;
    for row in range(0, fields.len()) {
      for col in range(0, fields[row].len()) {
        change = change || check_productive(row,col,-1,0,fields);
        change = change || check_productive(row,col,1,0,fields);
        change = change || check_productive(row,col,0,-1,fields);
        change = change || check_productive(row,col,0,1,fields);
      }
    }
  }
}

fn sanityCheck(soko: &SokoAnnotatedBoard) {
  for row in soko.board.iter() {
    for field in row.iter() {
      if !field.productive && field.hasBlock() {
        panic!(format!("Impossible puzzle: Block on unproductive spot: [{},{}]",field.row,field.col))
      }
    }
  }
}

fn assignIDs(soko: &mut SokoAnnotatedBoard) {
  let mut id = 0;
  let fields = soko.board.as_mut_slice();
  for row in fields.mut_iter() {
    for field in row.mut_iter() {
      field.man_id = Some(id);
      id += 2;
      field.block_id = Some(id);
      id += 2;
    }
  }
}

fn block_var_at(fields: &[Vec<AnnotatedField>], row: uint, col: uint) -> u32 {
  fields[row][col].block_id.expect("Field without block_id!")
}

fn man_var_at(fields: &[Vec<AnnotatedField>], row: uint, col: uint) -> u32 {
  fields[row][col].man_id.expect("Field without man_id!")
}

fn to_bdd_init(fields: &[Vec<AnnotatedField>]) -> Bdd {
  let mut result = Bdd::bddTrue();
  for row in fields.iter() {
    for field in row.iter() {
      if field.reachable {
        let man = Bdd::fromId(man_var_at(fields, field.row, field.col));
        if field.hasMan() {
          result = result & man;
        } else {
          result = result & !man;
        }
        if field.productive {
          let block = Bdd::fromId(block_var_at(fields, field.row, field.col));
          if field.hasBlock() {
            result = result & block;
          } else {
            result = result & !block;
          }
        }
      }
    }
  }
  result
}

fn same_man(fields: &[Vec<AnnotatedField>], row: uint, col: uint) -> Bdd {
  let mva = man_var_at(fields,row,col);
  Bdd::fromId(mva).biimp( Bdd::fromId(mva+1) )
}

fn same_block(fields: &[Vec<AnnotatedField>], row: uint, col: uint) -> Bdd {
  let bva = block_var_at(fields,row,col);
  Bdd::fromId(bva).biimp( Bdd::fromId(bva+1) )
}

fn everything_else_same(fields: &[Vec<AnnotatedField>], except: &[(uint, uint)]) -> Bdd {
  let mut result = Bdd::bddTrue();
  let mut excepts = 0;
  for row in range(0,fields.len()) {
    for col in range(0, fields[row].len()) {
      if !except.contains(&(row,col)) {
        if fields[row][col].reachable {
          result = result & same_man(fields, row, col);
          if fields[row][col].productive {
            result = result & same_block(fields, row, col);
          }
        }
      } else {
        excepts += 1;
      }
    }
  }
  assert_eq!(excepts, except.len());
  result
}

fn to_bdd_transitions(row: uint, col: uint, dr: uint, dc: uint, fields: &[Vec<AnnotatedField>]) -> Bdd {
  if valid_index(row,col,fields)
      && fields[row][col].reachable
      && valid_index(row+dr,col+dc,fields)
      && fields[row+dr][col+dc].reachable {
    let mva0 = man_var_at(fields,row,col);
    let current_man = Bdd::fromId(mva0);
    let next_man = Bdd::fromId(mva0+1);

    let mva1 = man_var_at(fields,row+dr,col+dc);
    let current_man_dxdy = Bdd::fromId(mva1);
    let next_man_dxdy = Bdd::fromId(mva1+1);

    let bva0 = block_var_at(fields,row,col);
    let current_block = Bdd::fromId(bva0);
    let next_block = Bdd::fromId(bva0+1);

    let bva1 = block_var_at(fields,row+dr,col+dc);
    let current_block_dxdy = Bdd::fromId(bva1);
    let next_block_dxdy = Bdd::fromId(bva1+1);

    let man_moves = current_man & (!current_man_dxdy) & (!next_man) & next_man_dxdy;

    let no_block = if fields[row][col].productive && fields[row+dr][col+dc].productive {
      (!current_block) & (!current_block_dxdy) & (!next_block) & (!next_block_dxdy)
    } else if fields[row][col].productive {
      (!current_block) & (!next_block)
    } else if fields[row+dr][col+dc].productive {
      (!current_block_dxdy) & (!next_block_dxdy)
    } else {
      Bdd::bddTrue()
    };

    let here = (row, col);
    let there = (row+dr, col+dc);
    let mut result = Bdd::bddFalse();

    if valid_index(row+dr, col+dc, fields) && fields[row+dr][col+dc].reachable {
      let everything_same_2 = everything_else_same(fields, [here,there].as_slice());
      result = result | ( man_moves & no_block & everything_same_2);
      let (rrr,ccc) = (row+dr+dr,col+dc+dc);
      if valid_index(rrr,ccc,fields) && fields[rrr][ccc].productive {
        let mva2 = man_var_at(fields,rrr,ccc);
        let current_man_2dxdy = Bdd::fromId(mva2);
        let next_man_2dxdy = Bdd::fromId(mva2+1);
        let bva2 = block_var_at(fields,rrr,ccc);
        let current_block_2dxdy = Bdd::fromId(bva2);
        let next_block_2dxdy = Bdd::fromId(bva2+1);
        let block_moves = current_block_dxdy & (!current_block_2dxdy) & (!next_block_dxdy) & next_block_2dxdy;

        let everything_same_3 = everything_else_same(fields, [here,there,(rrr,ccc)].as_slice());
        let man_2dxdy_same = (!current_man_2dxdy).biimp(!next_man_2dxdy);
        let not_block_present = (!current_block).biimp(!next_block);
        result = result | (man_moves & not_block_present & block_moves & man_2dxdy_same & everything_same_3);
      }
    }
    result
  } else {
    Bdd::bddFalse()
  }
}

fn to_bdd_trans_direction(fields: &[Vec<AnnotatedField>], drow: uint, dcol: uint) -> Bdd {
  let mut result = Bdd::bddFalse();
  for row in fields.iter() {
    for field in row.iter() {
      if field.reachable {
        result = result | to_bdd_transitions(field.row, field.col, drow, dcol, fields);
      }
    }
  }
  result
}

fn to_bdd_trans_up(fields: &[Vec<AnnotatedField>]) -> Bdd {
  to_bdd_trans_direction(fields, -1, 0)
}

fn to_bdd_trans_down(fields: &[Vec<AnnotatedField>]) -> Bdd {
  to_bdd_trans_direction(fields, 1, 0)
}

fn to_bdd_trans_left(fields: &[Vec<AnnotatedField>]) -> Bdd {
  to_bdd_trans_direction(fields, 0, -1)
}

fn to_bdd_trans_right(fields: &[Vec<AnnotatedField>]) -> Bdd {
  to_bdd_trans_direction(fields, 0, 1)
}

fn to_bdd_goal(fields: &[Vec<AnnotatedField>]) -> Bdd {
  let mut result = Bdd::bddTrue();
  for row in fields.iter() {
    for field in row.iter() {
      if field.isGoal() {
        result = result & Bdd::fromId(block_var_at(fields, field.row, field.col));
      }
    }
  }
  result
}

fn to_bdd_trans(fields: &[Vec<AnnotatedField>]) -> Bdd {
  let up = to_bdd_trans_up(fields);
  let down = to_bdd_trans_down(fields);
  let left = to_bdd_trans_left(fields);
  let right = to_bdd_trans_right(fields);
  up | down | left | right
}

fn all_vars(fields: &[Vec<AnnotatedField>]) -> Bdd {
  let mut result = Bdd::bddFalse();
  for row in fields.iter() {
    for field in row.iter() {
      if field.reachable {
        let mva = man_var_at(fields, field.row, field.col);
        result = result | Bdd::fromId(mva) | Bdd::fromId(mva+1);
        if field.productive {
          let bva = block_var_at(fields, field.row, field.col);
          result = result | Bdd::fromId(bva) | Bdd::fromId(bva+1);
        }
      }
    }
  }
  result
}

fn reconstruct_path(visited: Vec<Bdd>, goal: Bdd, equalizer: Bdd, fields: &[Vec<AnnotatedField>]) {
  use super::std::strbuf::StrBuf;
  let allvars = all_vars(fields);
  let mut current = goal;
  let trans_up = to_bdd_trans_up(fields);
  let trans_down = to_bdd_trans_down(fields);
  let trans_left = to_bdd_trans_left(fields);
  let trans_right = to_bdd_trans_right(fields);
  let mut path = StrBuf::with_capacity( visited.len() );

  let bddFalse = Bdd::bddFalse();
  for i in range(1, visited.len()).rev() {
    current = current.relprods(equalizer,allvars);
    let (up,down,left,right) = (
      visited.get(i-1) & current.relprods_reversed(trans_up,allvars),
      visited.get(i-1) & current.relprods_reversed(trans_down,allvars),
      visited.get(i-1) & current.relprods_reversed(trans_left,allvars),
      visited.get(i-1) & current.relprods_reversed(trans_right,allvars)
    );
    current = if up != bddFalse {
      path.push_char('u');
      up
    } else if down != bddFalse {
      path.push_char('d');
      down
    } else if left != bddFalse {
      path.push_char('l');
      left
    } else if right != bddFalse {
      path.push_char('r');
      right
    } else {
      panic!("Backtracking error");
    };
  }
  println!("Solution: {}", path
      .as_slice()
      .chars_rev()
      .fold(StrBuf::with_capacity( visited.len() ), |sb,c|{let mut sb = sb;sb.push_char(c);sb})
      .as_slice());
}

fn solve_the_puzzle(initial: Bdd, transitions: Bdd, goal: Bdd, fields: &[Vec<AnnotatedField>]) {
  fn won(current: Bdd, goal: Bdd) -> bool {
    current & goal != Bdd::bddFalse()
  }

  let equalizer = everything_else_same(fields, &[]);
  let allvars = all_vars(fields);
  let mut visited = Vec::new();

  let mut result = initial;
  visited.push( result );

  println!("Starting");
  let mut i = 0;
  loop {
    let old = result;
    result = result.relprods_equalize(transitions, allvars, equalizer);
    result = result | old;
    visited.push(result);
    i += 1;
    if won(result, goal) || old == result {
      break;
    }
  }
  if won(result, goal) {
    println!("Won in {} steps",i);
    reconstruct_path(visited, goal, equalizer, fields);
  } else {
    println!("Fail in {} steps", i);
    println!("no solution");
    panic!()
  }
}

/// Initializes sylvan, our bdd-library
fn sylvan_init() {
  unsafe {
    super::raw::raw_init();
  }
}

/// Explores the puzzle using sylvan, and prints the solution.
pub fn do_sylvan(soko: &SokoAnnotatedBoard) {
  sylvan_init();
  let fields = soko.board.as_slice();
  let initial = to_bdd_init(fields);
  let transitions = to_bdd_trans(fields);
  let goal = to_bdd_goal(fields);
  // solve!
  solve_the_puzzle(initial, transitions, goal, fields);
}
