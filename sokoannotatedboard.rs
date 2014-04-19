use super::sokoboard::{SokoBoard, Field, Empty, Block, Man, Wall, Goal, BlockOnGoal, ManOnGoal};

pub struct SokoAnnotatedBoard( ~[~[AnnotatedField]] );

impl SokoAnnotatedBoard {
  pub fn fromSokoBoard(board: SokoBoard) -> SokoAnnotatedBoard {
    let SokoBoard(board) = board;
    let mut newBoard = Vec::new();
    for row in range(0, board.len()) {
      let mut newRow = Vec::new();
      for col in range(0, board[row].len()) {
        newRow.push( AnnotatedField::new( board[row][col], row, col ) );
      }
      newBoard.push(newRow.move_iter().collect::<~[AnnotatedField]>());
    }

    SokoAnnotatedBoard(newBoard.move_iter().collect())
  }
}

#[deriving(Eq)]
pub struct AnnotatedField {
  field: Field,
  row: uint,
  col: uint,
  reachable: bool,
  productive: bool,
}

impl AnnotatedField {
  pub fn new(field: Field, row: uint, col: uint) -> AnnotatedField {
    AnnotatedField {
      field: field,
      row: row,
      col: col,
      reachable: false,
      productive: field == Goal || field == BlockOnGoal || field == ManOnGoal,
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
